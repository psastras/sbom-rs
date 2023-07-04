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

use anyhow::{anyhow, Ok, Result};
use cargo_metadata::{CargoOpt, MetadataCommand};
use clap::{Parser, ValueEnum};
use std::{env, path::PathBuf};
mod graph;
use petgraph::visit::EdgeRef;
use serde_spdx::spdx;

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

  let creation_info = spdx::SpdxCreationInfoBuilder::default()
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
    packages.push(
      spdx::SpdxItemPackagesBuilder::default()
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
        // TODO: a lot of cargo license strings don't comply with SPDX due to "/" instead of OR
        // TODO: should detect license file in package info
        .license_declared(
          package.license.as_ref().unwrap_or(&"UNKNOWN".to_string()),
        )
        .name(&package.name)
        .build()?,
    );

    edges.for_each(|e| {
      let source = &graph.graph[e.source()];
      let target = &graph.graph[e.target()];
      relationships.push(
        spdx::SpdxItemRelationshipsBuilder::default()
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
        spdx::SpdxItemFilesBuilder::default()
          .spdxid(format!("SPDXRef-File-{}", target.name))
          .checksums(vec![])
          .file_name(&target.name)
          .file_types(vec!["BINARY".to_string()])
          .build()
          .unwrap(),
      );
      relationships.push(
        spdx::SpdxItemRelationshipsBuilder::default()
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
  let spdx = spdx::SpdxBuilder::default()
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
