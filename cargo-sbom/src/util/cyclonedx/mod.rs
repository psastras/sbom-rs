use std::{
  collections::{HashMap, HashSet},
  path::PathBuf,
};

use crate::graph::Graph;
use anyhow::{anyhow, Ok, Result};
use petgraph::visit::EdgeRef;

pub mod built_info {
  // The file has been placed there by the build script.
  include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

struct HashableCycloneDxComponent(serde_cyclonedx::cyclonedx::v_1_4::Component);

impl std::hash::Hash for HashableCycloneDxComponent {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.name.hash(state);
    if let Some(version) = &self.0.version {
      version.hash(state);
    }
  }
}

impl std::cmp::PartialEq for HashableCycloneDxComponent {
  fn eq(&self, other: &Self) -> bool {
    self.0.name == other.0.name && self.0.version == other.0.version
  }
}

impl std::cmp::Eq for HashableCycloneDxComponent {}

#[allow(dead_code)]
struct HashableCycloneDxDependency(
  serde_cyclonedx::cyclonedx::v_1_4::Dependency,
);

impl std::hash::Hash for HashableCycloneDxDependency {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.ref_.hash(state);
    if let Some(depends_on) = &self.0.depends_on {
      depends_on.hash(state);
    }
  }
}

impl std::cmp::PartialEq for HashableCycloneDxDependency {
  fn eq(&self, other: &Self) -> bool {
    self.0.ref_ == other.0.ref_ && self.0.depends_on == other.0.depends_on
  }
}

impl std::cmp::Eq for HashableCycloneDxDependency {}

// V1.6 specific hashable types
struct HashableCycloneDxComponent1_6(serde_cyclonedx::cyclonedx::v_1_6::Component);

impl std::hash::Hash for HashableCycloneDxComponent1_6 {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.name.hash(state);
    if let Some(version) = &self.0.version {
      version.hash(state);
    }
  }
}

impl std::cmp::PartialEq for HashableCycloneDxComponent1_6 {
  fn eq(&self, other: &Self) -> bool {
    self.0.name == other.0.name && self.0.version == other.0.version
  }
}

impl std::cmp::Eq for HashableCycloneDxComponent1_6 {}

#[allow(dead_code)]
struct HashableCycloneDxDependency1_6(
  serde_cyclonedx::cyclonedx::v_1_6::Dependency,
);

impl std::hash::Hash for HashableCycloneDxDependency1_6 {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.ref_.hash(state);
    if let Some(depends_on) = &self.0.depends_on {
      depends_on.hash(state);
    }
  }
}

impl std::cmp::PartialEq for HashableCycloneDxDependency1_6 {
  fn eq(&self, other: &Self) -> bool {
    self.0.ref_ == other.0.ref_ && self.0.depends_on == other.0.depends_on
  }
}

impl std::cmp::Eq for HashableCycloneDxDependency1_6 {}

pub fn convert(
  cargo_package: Option<String>,
  project_directory: PathBuf,
  graph: &Graph,
) -> Result<serde_cyclonedx::cyclonedx::v_1_4::CycloneDx> {
  let absolute_project_directory = project_directory.canonicalize()?;
  let manifest_folder = absolute_project_directory
    .file_name()
    .ok_or(anyhow!("Failed to determine parent folder of Cargo.toml. Unable to assign a SPDX document name."))?;
  let name = cargo_package
    .clone()
    .unwrap_or_else(|| manifest_folder.to_string_lossy().to_string());
  let mut metadata =
    serde_cyclonedx::cyclonedx::v_1_4::MetadataBuilder::default();
  let mut root_component_builder =
    serde_cyclonedx::cyclonedx::v_1_4::ComponentBuilder::default();
  let mut root_component_components = vec![];

  // We traverse through the dependency graph multiple times in a Cargo workspace (once per package), so we need to keep a unique
  // set of dependencies and their relationships
  let mut components = HashSet::new();
  let mut dependencies: HashMap<String, HashSet<String>> = HashMap::new();

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
      let mut cyclonedx_component_builder =
        serde_cyclonedx::cyclonedx::v_1_4::ComponentBuilder::default();
      cyclonedx_component_builder
        .type_(if package.targets[0].is_lib() {
          "library"
        } else {
          "application"
        })
        .bom_ref(format!(
          "CycloneDxRef-Component-{}-{}",
          package.name, package.version
        ))
        .version(package.version.to_string())
        .name(package.name.clone());

      if let Some(description) = package.description.as_ref() {
        cyclonedx_component_builder.description(description);
      }

      let mut external_references = vec![];
      if let Some(documentation) = package.documentation.as_ref() {
        external_references.push(
          serde_cyclonedx::cyclonedx::v_1_4::ExternalReferenceBuilder::default(
          )
          .type_("documentation")
          .url(documentation)
          .build()?,
        )
      }
      if let Some(homepage) = package.homepage.as_ref() {
        external_references.push(
          serde_cyclonedx::cyclonedx::v_1_4::ExternalReferenceBuilder::default(
          )
          .type_("website")
          .url(homepage)
          .build()?,
        )
      }
      if let Some(repository) = package.repository.as_ref() {
        external_references.push(
          serde_cyclonedx::cyclonedx::v_1_4::ExternalReferenceBuilder::default(
          )
          .type_("vcs")
          .url(repository)
          .build()?,
        )
      }

      cyclonedx_component_builder.external_references(external_references);
      cyclonedx_component_builder.author(package.authors.join(", "));

      let cyclonedx_license =
        serde_cyclonedx::cyclonedx::v_1_4::LicenseChoiceBuilder::default()
          .expression(
            super::spdx::license::normalize_license_string(
              package.license.as_ref().unwrap_or(&"UNKNOWN".to_string()),
            )
            .unwrap_or("NOASSERTION".to_string()),
          )
          .build()?;

      cyclonedx_component_builder.licenses(vec![cyclonedx_license]);

      if let Some(source) = package.source.as_ref() {
        if source.is_crates_io() {
          let purl = packageurl::PackageUrl::new::<&str, &str>(
            "cargo",
            package.name.as_ref(),
          )
          .expect("only fails if type is invalid")
          .with_version(package.version.to_string())
          .to_string();
          cyclonedx_component_builder.purl(purl);
        }
      }

      if &package.id == root_package_id {
        root_component_components.push(cyclonedx_component_builder.build()?)
      } else {
        components.insert(HashableCycloneDxComponent(
          cyclonedx_component_builder.build()?,
        ));
      }

      edges.for_each(|e| {
        let source = &graph.graph[e.source()];
        let target = &graph.graph[e.target()];
        let source_ref =
          format!("CycloneDxRef-Component-{}-{}", source.name, source.version);
        let target_ref =
          format!("CycloneDxRef-Component-{}-{}", target.name, target.version);
        if let Some(depends_on) = dependencies.get_mut(&source_ref) {
          depends_on.insert(target_ref);
        } else {
          dependencies.insert(source_ref, HashSet::from([target_ref]));
        }
      });
    }
  }

  let cyclonedx =
    serde_cyclonedx::cyclonedx::v_1_4::CycloneDxBuilder::default()
      .metadata(
        metadata
          .component(
            root_component_builder
              .name(name)
              .type_("application")
              .components(root_component_components)
              .build()?,
          )
          .tools(vec![
            serde_cyclonedx::cyclonedx::v_1_4::ToolBuilder::default()
              .name(built_info::PKG_NAME)
              .version(built_info::PKG_VERSION)
              .build()?,
          ])
          .authors(vec![serde_cyclonedx::cyclonedx::v_1_4::OrganizationalContactBuilder::default().name(whoami::realname()).build()?])
          .timestamp(
            chrono::Utc::now()
              .format("%Y-%m-%dT%H:%M:%S%.3fZ")
              .to_string(),
          )
          .build()?,
      )
      .bom_format("CycloneDX")
      .components(components.iter().map(|p| p.0.clone()).collect::<Vec<_>>())
      .dependencies(
        dependencies
          .iter()
          .map(|p| {
            serde_cyclonedx::cyclonedx::v_1_4::DependencyBuilder::default()
              .ref_(p.0)
              .depends_on(p.1.iter().cloned().collect::<Vec<String>>())
              .build()
              .unwrap()
          })
          .collect::<Vec<_>>(),
      )
      .serial_number(format!("urn:uuid:{}", uuid::Uuid::new_v4()))
      .spec_version("1.4")
      .version(1)
      .build()?;

  Ok(cyclonedx)
}

pub fn convert_1_6(
  cargo_package: Option<String>,
  project_directory: PathBuf,
  graph: &Graph,
) -> Result<serde_cyclonedx::cyclonedx::v_1_6::CycloneDx> {
  let absolute_project_directory = project_directory.canonicalize()?;
  let manifest_folder = absolute_project_directory
    .file_name()
    .ok_or(anyhow!("Failed to determine parent folder of Cargo.toml. Unable to assign a SPDX document name."))?;
  let name = cargo_package
    .clone()
    .unwrap_or_else(|| manifest_folder.to_string_lossy().to_string());
  let mut metadata =
    serde_cyclonedx::cyclonedx::v_1_6::MetadataBuilder::default();
  let mut root_component_builder =
    serde_cyclonedx::cyclonedx::v_1_6::ComponentBuilder::default();
  let mut root_component_components = vec![];

  // We traverse through the dependency graph multiple times in a Cargo workspace (once per package), so we need to keep a unique
  // set of dependencies and their relationships
  let mut components = HashSet::new();
  let mut dependencies: HashMap<String, HashSet<String>> = HashMap::new();

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
      let mut cyclonedx_component_builder =
        serde_cyclonedx::cyclonedx::v_1_6::ComponentBuilder::default();
      cyclonedx_component_builder
        .type_(if package.targets[0].is_lib() {
          "library"
        } else {
          "application"
        })
        .bom_ref(format!(
          "CycloneDxRef-Component-{}-{}",
          package.name, package.version
        ))
        .version(package.version.to_string())
        .name(package.name.clone());

      if let Some(description) = package.description.as_ref() {
        cyclonedx_component_builder.description(description);
      }

      let mut external_references = vec![];
      if let Some(documentation) = package.documentation.as_ref() {
        external_references.push(
          serde_cyclonedx::cyclonedx::v_1_6::ExternalReferenceBuilder::default(
          )
          .type_("documentation")
          .url(serde_json::Value::String(documentation.clone()))
          .build()?,
        )
      }
      if let Some(homepage) = package.homepage.as_ref() {
        external_references.push(
          serde_cyclonedx::cyclonedx::v_1_6::ExternalReferenceBuilder::default(
          )
          .type_("website")
          .url(serde_json::Value::String(homepage.clone()))
          .build()?,
        )
      }
      if let Some(repository) = package.repository.as_ref() {
        external_references.push(
          serde_cyclonedx::cyclonedx::v_1_6::ExternalReferenceBuilder::default(
          )
          .type_("vcs")
          .url(serde_json::Value::String(repository.clone()))
          .build()?,
        )
      }

      cyclonedx_component_builder.external_references(external_references);
      cyclonedx_component_builder.author(package.authors.join(", "));

      let cyclonedx_license = serde_cyclonedx::cyclonedx::v_1_6::LicenseChoiceUrl::Variant1(
        vec![serde_cyclonedx::cyclonedx::v_1_6::LicenseChoiceUrlVariant1ItemLicenseBuilder::default()
          .expression(
            super::spdx::license::normalize_license_string(
              package.license.as_ref().unwrap_or(&"UNKNOWN".to_string()),
            )
            .unwrap_or("NOASSERTION".to_string()),
          )
          .build()?]
      );

      cyclonedx_component_builder.licenses(cyclonedx_license);

      if let Some(source) = package.source.as_ref() {
        if source.is_crates_io() {
          let purl = packageurl::PackageUrl::new::<&str, &str>(
            "cargo",
            package.name.as_ref(),
          )
          .expect("only fails if type is invalid")
          .with_version(package.version.to_string())
          .to_string();
          cyclonedx_component_builder.purl(purl);
        }
      }

      if &package.id == root_package_id {
        root_component_components.push(cyclonedx_component_builder.build()?)
      } else {
        components.insert(HashableCycloneDxComponent1_6(
          cyclonedx_component_builder.build()?,
        ));
      }

      edges.for_each(|e| {
        let source = &graph.graph[e.source()];
        let target = &graph.graph[e.target()];
        let source_ref =
          format!("CycloneDxRef-Component-{}-{}", source.name, source.version);
        let target_ref =
          format!("CycloneDxRef-Component-{}-{}", target.name, target.version);
        if let Some(depends_on) = dependencies.get_mut(&source_ref) {
          depends_on.insert(target_ref);
        } else {
          dependencies.insert(source_ref, HashSet::from([target_ref]));
        }
      });
    }
  }

  let cyclonedx =
    serde_cyclonedx::cyclonedx::v_1_6::CycloneDxBuilder::default()
      .metadata(
        metadata
          .component(
            root_component_builder
              .name(name)
              .type_("application")
              .components(root_component_components)
              .build()?,
          )
          .tools(serde_cyclonedx::cyclonedx::v_1_6::MetadataTools::Variant1(vec![
            serde_cyclonedx::cyclonedx::v_1_6::ToolBuilder::default()
              .name(built_info::PKG_NAME)
              .version(built_info::PKG_VERSION)
              .build()?,
          ]))
          .authors(vec![serde_cyclonedx::cyclonedx::v_1_6::OrganizationalContactBuilder::default().name(whoami::realname()).build()?])
          .timestamp(
            chrono::Utc::now()
              .format("%Y-%m-%dT%H:%M:%S%.3fZ")
              .to_string(),
          )
          .build()?,
      )
      .bom_format("CycloneDX")
      .components(components.iter().map(|p| p.0.clone()).collect::<Vec<_>>())
      .dependencies(
        dependencies
          .iter()
          .map(|p| {
            serde_cyclonedx::cyclonedx::v_1_6::DependencyBuilder::default()
              .ref_(p.0)
              .depends_on(p.1.iter().cloned().collect::<Vec<String>>())
              .build()
              .unwrap()
          })
          .collect::<Vec<_>>(),
      )
      .serial_number(format!("urn:uuid:{}", uuid::Uuid::new_v4()))
      .spec_version("1.6")
      .version(1)
      .build()?;

  Ok(cyclonedx)
}
