pub mod license;
use std::{
  collections::HashSet,
  path::{Path, PathBuf},
};

use crate::graph::Graph;
use anyhow::{anyhow, Ok, Result};
use cargo_metadata::Package;
use petgraph::visit::EdgeRef;
use sha1::{Digest, Sha1};

pub mod built_info {
  // The file has been placed there by the build script.
  include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

struct HashableSpdxItemPackages(serde_spdx::spdx::v_2_3::SpdxItemPackages);

impl std::hash::Hash for HashableSpdxItemPackages {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.spdxid.hash(state)
  }
}

impl std::cmp::PartialEq for HashableSpdxItemPackages {
  fn eq(&self, other: &Self) -> bool {
    self.0.spdxid == other.0.spdxid
  }
}

impl std::cmp::Eq for HashableSpdxItemPackages {}

struct HashableSpdxItemRelationships(
  serde_spdx::spdx::v_2_3::SpdxItemRelationships,
);

impl std::hash::Hash for HashableSpdxItemRelationships {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.spdx_element_id.hash(state);
    self.0.related_spdx_element.hash(state);
    self.0.relationship_type.hash(state);
  }
}

impl std::cmp::PartialEq for HashableSpdxItemRelationships {
  fn eq(&self, other: &Self) -> bool {
    self.0.spdx_element_id == other.0.spdx_element_id
      && self.0.related_spdx_element == other.0.related_spdx_element
      && self.0.relationship_type == other.0.relationship_type
  }
}

impl std::cmp::Eq for HashableSpdxItemRelationships {}

/// All the data we use comes from the lock file, so we treat that
/// as the root file of the package so we have something to checksum.
fn process_root_file(
  spdx_id: &str,
  project_directory: &Path,
  cargo_manifest_path: &Path,
) -> Result<serde_spdx::spdx::v_2_3::SpdxItemFiles> {
  let lockfile = cargo_manifest_path.canonicalize()?.with_extension("lock");
  let contents = std::fs::read(&lockfile)?;
  let checksum = Sha1::digest(&contents);

  // SHA1 only algorithm supported
  let checksum_element =
    serde_spdx::spdx::v_2_3::SpdxItemFilesItemChecksumsBuilder::default()
      .algorithm("SHA1")
      .checksum_value(base16ct::lower::encode_string(&checksum))
      .build()
      .unwrap();

  let relative_lockfile = PathBuf::from(&".")
    .join(lockfile.strip_prefix(project_directory.canonicalize()?)?);
  let relative_lockfile_string = relative_lockfile
    .to_str()
    .ok_or_else(|| anyhow!(&"Non-UTF8 relative lockfile path"))?;

  std::result::Result::Ok(
    serde_spdx::spdx::v_2_3::SpdxItemFilesBuilder::default()
      .spdxid(spdx_id)
      .file_name(relative_lockfile_string)
      .checksums(vec![checksum_element])
      .file_types(vec!["SOURCE".to_string(), "TEXT".to_string()])
      .build()
      .unwrap(),
  )
}

/// Generate SPDXRef-Package-... and replace characters that are common but not permitted
fn generate_project_id(package: &Package) -> String {
  // underscores become two dashes to avoid collisions with similarly named packages
  format!("SPDXRef-Package-{}-{}", package.name, package.version)
    .replace("_", "--")
    .replace("+", "-plus-")
}

pub fn convert(
  cargo_package: Option<String>,
  project_directory: &Path,
  cargo_manifest_path: &Path,
  graph: &Graph,
) -> Result<serde_spdx::spdx::v_2_3::Spdx> {
  let creation_info =
    serde_spdx::spdx::v_2_3::SpdxCreationInfoBuilder::default()
      .created(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string())
      .creators(vec![format!(
        "Tool: {}-v{}",
        built_info::PKG_NAME,
        built_info::PKG_VERSION
      )])
      .build()?;

  // We traverse through the dependency graph multiple times in a Cargo workspace (once per package), so we need to keep a unique
  // set of dependencies and their relationships
  let mut packages = HashSet::new();
  let mut relationships = HashSet::new();

  let mut files = vec![];

  for root_package_id in graph.root_packages.iter() {
    let root_node_index = graph
      .nodes
      .get(root_package_id)
      .ok_or(anyhow!("No root node. Shouldn't reach here."))?;
    let root = graph.graph[*root_node_index];
    if let Some(r) = cargo_package.as_ref() {
      if r != &root.name {
        continue;
      }
    }

    let mut dfs = petgraph::visit::Dfs::new(&graph.graph, *root_node_index);
    while let Some(nx) = dfs.next(&graph.graph) {
      let edges = graph.graph.edges(nx);
      let package = graph.graph[nx];
      let mut spdx_package_builder =
        serde_spdx::spdx::v_2_3::SpdxItemPackagesBuilder::default();
      let normalized_license = package
        .license
        .as_ref()
        .and_then(|license| license::normalize_license_string(license).ok());

      spdx_package_builder
        .spdxid(generate_project_id(package))
        .version_info(package.version.to_string())
        .download_location(
          package
            .source
            .as_ref()
            .map(|source| source.to_string())
            .unwrap_or("NONE".to_string()),
        )
        .license_concluded(
          normalized_license.as_deref().unwrap_or("NOASSERTION"),
        )
        .name(&package.name);

      if let Some(license_declared) = normalized_license {
        spdx_package_builder.license_declared(license_declared);
      }

      if let Some(description) = package.description.as_ref() {
        spdx_package_builder.description(description);
      }

      if let Some(homepage) = package.homepage.as_ref() {
        spdx_package_builder.homepage(homepage);
      }

      if let Some(source) = package.source.as_ref() {
        if source.is_crates_io() {
          let purl = packageurl::PackageUrl::new::<&str, &str>(
            "cargo",
            package.name.as_ref(),
          )
          .expect("only fails if type is invalid")
          .with_version(package.version.to_string())
          .to_string();
          let external_refs =
            serde_spdx::spdx::v_2_3::SpdxItemPackagesItemExternalRefsBuilder::default(
            )
            .reference_category("PACKAGE-MANAGER")
            .reference_type("purl")
            .reference_locator(purl)
            .build()?;
          spdx_package_builder.external_refs(vec![external_refs]);
        }
      }

      packages.insert(HashableSpdxItemPackages(spdx_package_builder.build()?));

      edges.for_each(|e| {
        let source = &graph.graph[e.source()];
        let target = &graph.graph[e.target()];
        relationships.insert(HashableSpdxItemRelationships(
          serde_spdx::spdx::v_2_3::SpdxItemRelationshipsBuilder::default()
            .spdx_element_id(generate_project_id(source))
            .related_spdx_element(generate_project_id(target))
            .relationship_type("DEPENDS_ON")
            .build()
            .unwrap(),
        ));
      });
    }

    root
      .targets
      .iter()
      .filter(|target| target.is_bin() || target.is_lib())
      .for_each(|target| {
        let spdx_id = format!("SPDXRef-File-{}", target.name);

        files.push(
          process_root_file(&spdx_id, project_directory, cargo_manifest_path)
            .unwrap(),
        );
        relationships.insert(HashableSpdxItemRelationships(
          serde_spdx::spdx::v_2_3::SpdxItemRelationshipsBuilder::default()
            .spdx_element_id(&spdx_id)
            .related_spdx_element(generate_project_id(root))
            .relationship_type("GENERATED_FROM")
            .build()
            .unwrap(),
        ));

        relationships.insert(HashableSpdxItemRelationships(
          serde_spdx::spdx::v_2_3::SpdxItemRelationshipsBuilder::default()
            .spdx_element_id("SPDXRef-DOCUMENT")
            .related_spdx_element(&spdx_id)
            .relationship_type("DESCRIBES")
            .build()
            .unwrap(),
        ));
      });
  }

  let absolute_project_directory = project_directory.canonicalize()?;
  let manifest_folder = absolute_project_directory
    .file_name()
    .ok_or(anyhow!("Failed to determine parent folder of Cargo.toml. Unable to assign a SPDX document name."))?;
  let name = cargo_package
    .unwrap_or_else(|| manifest_folder.to_string_lossy().to_string());
  let uuid = uuid::Uuid::new_v4();
  Ok(
    serde_spdx::spdx::v_2_3::SpdxBuilder::default()
      .spdxid("SPDXRef-DOCUMENT")
      .creation_info(creation_info)
      .data_license("CC0-1.0")
      .document_namespace(format!("https://spdx.org/spdxdocs/{name}-{uuid}"))
      .files(files)
      .name(name)
      .spdx_version("SPDX-2.3")
      .packages(packages.iter().map(|p| p.0.clone()).collect::<Vec<_>>())
      .relationships(
        relationships
          .iter()
          .map(|p| p.0.clone())
          .collect::<Vec<_>>(),
      )
      .build()?,
  )
}
