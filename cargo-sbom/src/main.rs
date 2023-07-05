#![doc(html_root_url = "https://docs.rs/cargo-sbom/0.3.1")]

//! # cargo-sbom
//!
//! This crate provides a command line tool to create software bill of materials (SBOM) for Cargo / Rust workspaces. It supports both SPDX and CycloneDX outputs.
//!
//! The latest [documentation can be found here](https://docs.rs/cargo_sbom).
//!
//! SBOM or Software Bill of Materials is an industry standard term used to trace and maintain the supply chain security of software.
//!
//! ## Installation
//!
//! `cargo-sbom` may be installed via `cargo`
//!
//! ```shell
//! cargo install cargo-sbom
//! ```
//!
//! via [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)
//!
//! ```shell
//! cargo binstall cargo-sbom
//! ```
//!
//! or downloaded directly from Github Releases
//!
//! ```shell
//! # make sure to adjust the target and version (you may also want to pin to a specific version)
//! curl -sSL https://github.com/psastras/sbom-rs/releases/download/cargo-sbom-latest/cargo-sbom-x86_64-unknown-linux-gnu -o cargo-sbom
//! ```
//!
//! ## Usage
//!
//! For most cases, simply `cd` into a cargo workspace and run `cargo sbom`.
//!
//! ## Example
//!
//! ```shell
//! $ cargo sbom
//! {
//!   "SPDXID": "SPDXRef-DOCUMENT",
//!   "creationInfo": {
//!     "created": "2023-07-04T12:38:15.211Z",
//!     "creators": [
//!       "Tool: cargo-sbom-v0.3.1"
//!     ]
//!   },
//!   "dataLicense": "CC0-1.0",
//!   "documentNamespace": "https://docs.rs/cargo_sbom/spdxdocs/cargo-sbom-0.3.1-9cae390a-4b46-457c-95b9-e59a5e62b57d",
//!   "files": [
//!     {
//!   <rest of output omitted>
//! ```
//!
//! ## Supported SBOM Features
//!
//! ### SPDX
//!
//! | SPDX Field                | Source                                                                                             |
//! |---------------------------|----------------------------------------------------------------------------------------------------|
//! | packages.SPDXID           |                                            Written as SPDXRef-Package-crate name-crate version     |
//! | packages.description      |                                                         Read from Cargo.toml's "description" field |
//! | packages.downloadLocation | Read from `cargo metadata` (usually "registry+https://github.com/rust-lang/crates.io-index")       |
//! | packages.externalRefs     | If packages.downloadLocation is crates.io, written as a package url formatted string               |
//! | packages.homepage         |                                                            Read from Cargo.toml's "homepage" field |
//! | packages.licenseConcluded |                                                          Parsed from Cargo.toml's "homepage" field |
//! | packages.licenseDeclared  |                                                             Read from Cargo.toml's "license" field |
//! | packages.name             |                                                                Read from Cargo.toml's "name" field |
//!
//! ### CycloneDx
//!
//! None
//!
//!
use anyhow::{anyhow, Ok, Result};
use cargo_metadata::{CargoOpt, MetadataCommand};
use clap::{Parser, ValueEnum};
use std::{env, fmt::Debug, path::PathBuf};
mod graph;
use petgraph::visit::EdgeRef;

mod util;

pub mod built_info {
  // The file has been placed there by the build script.
  include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(ValueEnum, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "snake_case")]
enum OutputFormat {
  Spdx,
  CycloneDx,
}

fn get_default_cargo_manifest_path() -> PathBuf {
  PathBuf::from("./Cargo.toml")
}

#[derive(Parser)]
#[clap(
  bin_name = "cargo sbom",
  about = "Cargo subcommand to produce a software bill of materials (SBOM)."
)]
#[command(author, version, about, long_about = None)]
struct Opt {
  #[clap(long, help = "The path to a Cargo.toml", default_value = get_default_cargo_manifest_path().into_os_string())]
  cargo_manifest_path: PathBuf,
  #[clap(long, value_enum, help = "The SBOM output format.", default_value_t = OutputFormat::Spdx)]
  output_format: OutputFormat,
}

fn main() {
  if let Err(err) = try_main() {
    eprintln!("ERROR: {}", err);
    err
      .chain()
      .skip(1)
      .for_each(|cause| eprintln!("because: {}", cause));
    std::process::exit(1);
  }
}

fn try_main() -> Result<()> {
  // Drop extra `sbom` argument when called by `cargo`.
  let args = env::args().enumerate().filter_map(|(i, x)| {
    if (i, x.as_str()) == (1, "sbom") {
      None
    } else {
      Some(x)
    }
  });
  let opt = Opt::parse_from(args);
  if matches!(opt.output_format, OutputFormat::CycloneDx) {
    return Err(anyhow!("Output format not yet supported."));
  }

  let metadata = MetadataCommand::new()
    .manifest_path(opt.cargo_manifest_path)
    .features(CargoOpt::AllFeatures)
    .exec()?;

  let graph = graph::build(metadata)?;
  if graph.root.is_none() {
    return Err(anyhow!("Does not support cargo root workspaces yet. Rerun this tool targeting a specific package within the workspace (see --cargo-manifest-path)."));
  }

  let creation_info = serde_spdx::spdx::SpdxCreationInfoBuilder::default()
    .created(
      chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string(),
    )
    .creators(vec![format!(
      "Tool: {}-v{}",
      built_info::PKG_NAME,
      built_info::PKG_VERSION
    )])
    .build()?;

  let root_package_id = graph
    .root
    .as_ref()
    .ok_or(anyhow!("No root node. Shouldn't reach here."))?;
  let root_node_index = graph
    .nodes
    .get(root_package_id)
    .ok_or(anyhow!("No root node. Shouldn't reach here."))?;
  let mut dfs = petgraph::visit::Dfs::new(&graph.graph, *root_node_index);

  let mut packages = vec![];
  let mut relationships = vec![];

  while let Some(nx) = dfs.next(&graph.graph) {
    let edges = graph.graph.edges(nx);
    let package = &graph.graph[nx];
    let mut spdx_package_builder =
      serde_spdx::spdx::SpdxItemPackagesBuilder::default();

    spdx_package_builder
      .spdxid(format!(
        "SPDXRef-Package-{}-{}",
        package.name, package.version
      ))
      .download_location(
        package
          .source
          .as_ref()
          .map(|source| source.to_string())
          .unwrap_or("NONE".to_string()),
      )
      .license_concluded(
        util::spdx::license::normalize_license_string(
          package.license.as_ref().unwrap_or(&"UNKNOWN".to_string()),
        )
        .unwrap_or("NOASSERTION".to_string()),
      )
      .name(&package.name);

    if let Some(license_declared) = package.license.as_ref() {
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
          serde_spdx::spdx::SpdxItemPackagesItemExternalRefsBuilder::default()
            .reference_category("PACKAGE-MANAGER")
            .reference_type("purl")
            .reference_locator(purl)
            .build()?;
        spdx_package_builder.external_refs(vec![external_refs]);
      }
    }

    // spdx_package_builder.originator(package.authors.join(", "));
    packages.push(spdx_package_builder.build()?);

    edges.for_each(|e| {
      let source = &graph.graph[e.source()];
      let target = &graph.graph[e.target()];
      relationships.push(
        serde_spdx::spdx::SpdxItemRelationshipsBuilder::default()
          .spdx_element_id(format!(
            "SPDXRef-Package-{}-{}",
            source.name, source.version
          ))
          .related_spdx_element(format!(
            "SPDXRef-Package-{}-{}",
            target.name, target.version
          ))
          .relationship_type("DEPENDS_ON")
          .build()
          .unwrap(),
      )
    });
  }

  let root = &graph.graph[*root_node_index];
  let mut files = vec![];
  root
    .targets
    .iter()
    .filter(|target| target.is_bin() || target.is_lib())
    .for_each(|target| {
      files.push(
        serde_spdx::spdx::SpdxItemFilesBuilder::default()
          .spdxid(format!("SPDXRef-File-{}", target.name))
          .checksums(vec![])
          .file_name(&target.name)
          .file_types(vec!["BINARY".to_string()])
          .build()
          .unwrap(),
      );
      relationships.push(
        serde_spdx::spdx::SpdxItemRelationshipsBuilder::default()
          .spdx_element_id(format!("SPDXRef-File-{}", target.name))
          .related_spdx_element(format!(
            "SPDXRef-Package-{}-{}",
            root.name, root.version
          ))
          .relationship_type("GENERATED_FROM")
          .build()
          .unwrap(),
      );
    });

  let uuid = uuid::Uuid::new_v4();
  let spdx = serde_spdx::spdx::SpdxBuilder::default()
    .spdxid("SPDXRef-DOCUMENT")
    .creation_info(creation_info)
    .data_license("CC0-1.0")
    .document_namespace(match root.documentation.as_ref() {
      Some(docs) => {
        format!("{}/spdxdocs/{}-{}-{}", docs, root.name, root.version, uuid)
      }
      // TODO: should figure out better fallback
      _ => "UNKNOWN".to_string(),
    })
    .files(files)
    .name(format!("{}-{}", root.name, root.version))
    .spdx_version("SPDX-2.3")
    .packages(packages)
    .relationships(relationships)
    .build()?;

  println!("{}", serde_json::to_string_pretty(&spdx)?);

  Ok(())
}
