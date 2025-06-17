#![doc(html_root_url = "https://docs.rs/cargo-sbom/0.10.0")]

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
//! ### `--help`
//!
//! ```
//! Create software bill of materials (SBOM) for Rust
//!
//! Usage: cargo sbom [OPTIONS]
//!
//! Options:
//!       --cargo-package <CARGO_PACKAGE>
//!           The specific package (in a Cargo workspace) to generate an SBOM for. If not specified this is all packages in the workspace.
//!       --output-format <OUTPUT_FORMAT>
//!           The SBOM output format. [default: spdx_json_2_3] [possible values: spdx_json_2_3, cyclone_dx_json_1_4, cyclone_dx_json_1_5, cyclone_dx_json_1_6]
//!       --project-directory <PROJECT_DIRECTORY>
//!           The directory to the Cargo project. [default: .]
//!   -h, --help
//!           Print help
//!   -V, --version
//!           Print version
//! ```
//!
//! ## Examples
//!
//! ### Create a SPDX SBOM for a Cargo project
//!
//! In a shell:
//!
//! ```shell
//! $ cargo sbom
//! {
//!   "SPDXID": "SPDXRef-DOCUMENT",
//!   "creationInfo": {
//!     "created": "2023-07-04T12:38:15.211Z",
//!     "creators": [
//!       "Tool: cargo-sbom-v0.10.0"
//!     ]
//!   },
//!   "dataLicense": "CC0-1.0",
//!   "documentNamespace": "https://docs.rs/cargo_sbom/spdxdocs/cargo-sbom-0.10.0-9cae390a-4b46-457c-95b9-e59a5e62b57d",
//!   "files": [
//!     {
//!   <rest of output omitted>
//! ```
//!
//! ### Create a CycloneDx SBOM in Github Actions
//!
//! In a Github Actions workflow:
//!
//! ```yaml
//! jobs:
//!   sbom:
//!     runs-on: ubuntu-latest
//!     steps:
//!     - uses: actions/checkout@v3
//!     - uses: psastras/sbom-rs/actions/install-cargo-sbom@cargo-sbom-latest
//!     - name: Run cargo-sbom
//!       run: cargo-sbom --output-format=cyclone_dx_json_1_4
//! ```
//!
//! ### Create a CycloneDx 1.6 SBOM in Github Actions
//!
//! ```yaml
//!     runs-on: ubuntu-latest
//!     steps:
//!     - uses: actions/checkout@v3
//!     - uses: psastras/sbom-rs/actions/install-cargo-sbom@cargo-sbom-latest
//!     - name: Run cargo-sbom
//!       run: cargo-sbom --output-format=cyclone_dx_json_1_6
//! ```
//!
//! ### Create a CycloneDx 1.5 SBOM in Github Actions
//!
//! ```yaml
//!     runs-on: ubuntu-latest
//!     steps:
//!     - uses: actions/checkout@v3
//!     - uses: psastras/sbom-rs/actions/install-cargo-sbom@cargo-sbom-latest
//!     - name: Run cargo-sbom
//!       run: cargo-sbom --output-format=cyclone_dx_json_1_5
//! ```
//!
//! ### Check Dependencies against the Open Source Vulnerability Database (OSV)
//!
//! Assumming `osv-scanner` is installed (see [https://osv.dev/](https://osv.dev/))
//!
//! ```shell
//! $ cargo-sbom > sbom.spdx.json
//! $ osv-scanner --sbom=sbom.spdx.json
//! Scanned sbom.json as SPDX SBOM and found 91 packages
//! ╭─────────────────────────────────────┬──────┬───────────┬─────────┬─────────┬───────────╮
//! │ OSV URL                             │ CVSS │ ECOSYSTEM │ PACKAGE │ VERSION │ SOURCE    │
//! ├─────────────────────────────────────┼──────┼───────────┼─────────┼─────────┼───────────┤
//! │ https://osv.dev/GHSA-wcg3-cvx6-7396 │ 6.2, │ crates.io │ time    │ 0.1.45  │ sbom.json │
//! │ https://osv.dev/RUSTSEC-2020-0071   │ 6.2  │           │         │         │           │
//! ╰─────────────────────────────────────┴──────┴───────────┴─────────┴─────────┴───────────╯
//! ```
//!
//! More examples can be found by browsing the [examples section](https://github.com/psastras/sbom-rs/tree/main/examples).
//!
//! ## Supported SBOM Features
//!
//! ### SPDX
//!
//! | SPDX Field                       | Source                                                                                       |
//! |----------------------------------|----------------------------------------------------------------------------------------------|
//! | SPDXID                           | Set to "SPDXRef-Document"                                                                    |
//! | creationInfo.created             | Set as the current time                                                                      |
//! | creationInfo.creators            | Set to "Tool: cargo-sbom-v(tool version)                                                     |
//! | dataLicense                      | Set to "CC0-1.0"                                                                             |
//! | documentNamespace                | set to "https://spdx.org/spdxdocs/(crate-name)-(uuidv4)"                                     |
//! | files                            | parsed from Cargo.toml target names                                                          |
//! | name                             | Set to the project folder name                                                               |
//! | packages                         | Set to dependencies parsed from cargo-metadata                                               |
//! | packages.SPDXID                  | Written as SPDXRef-Package-(crate name)-(crate version)                                      |
//! | packages.description             | Read from Cargo.toml's "description" field                                                   |
//! | packages.downloadLocation        | Read from `cargo metadata` (usually "registry+https://github.com/rust-lang/crates.io-index") |
//! | packages.externalRefs            | If packages.downloadLocation is crates.io, written as a package url formatted string         |
//! | packages.homepage                | Read from Cargo.toml's "homepage" field                                                      |
//! | packages.licenseConcluded        | Parsed into a SPDX compliant license identifier from Cargo.toml's "license" field            |
//! | packages.licenseDeclared         | Read from Cargo.toml's "license" field                                                       |
//! | packages.name                    | Read from Cargo.toml's "name" field                                                          |
//! | relationships                    | Set to dependency relationships parsed from cargo-metadata                                   |
//! | relationships.relationshipType   | Set to dependency relationship parsed from cargo-metadata                                    |
//! | relationships.spdxElementId      | Set to dependency relationship source parsed from cargo-metadata                             |
//! | relationships.relatedSpdxElement | Set to dependency relationship target parsed from cargo-metadata                             |
//!
//!
//! ### CycloneDx
//!
//! | CycloneDx Field               | Source                                                                            |
//! |-------------------------------|-----------------------------------------------------------------------------------|
//! | bomFormat                     | Set to "CycloneDX"                                                                |
//! | serialNumber                  | Set to "urn:uuid:(uuidv4)"                                                        |
//! | specVersion                   | Set to 1.4                                                                        |
//! | version                       | Set to 1                                                                          |
//! | metadata                      |                                                                                   |
//! | metadata.component            | parsed from the root workspace                                                    |
//! | metadata.component.name       | Set to the root workspace folder name                                             |
//! | metadata.component.type       | Set to "application"                                                              |
//! | metadata.component.components | Set to each of the cargo workspace package components                             |
//! | components                    | Set to the componennts parse from cargo-metadata                                  |
//! | components.author             | Read from Cargo.toml's "authors" field                                            |
//! | components.bom-ref            | Set to "CycloneDxRef-Component-(crate-name)-(crate-version)"                      |
//! | components.description        | Read from Cargo.toml's "description" field                                        |
//! | copmonents.licenses           | Parsed into a SPDX compliant license identifier from Cargo.toml's "license" field |
//! | components.name               | Read from Cargo.toml's "name" field                                               |
//! | components.purl               | If the download location is crates.io, written as a package url formatted string  |
//! | components.type               | Read from cargo-metadata crate type                                               |
//! | components.version            | Read from Cargo.toml's "version" field                                            |
//! | dependencies                  | Set to dependency relationships parsed from cargo-metadata                        |
//! | dependencies.ref              | Set to source dependency reference id string                                      |
//! | dependencies.dependsOnn       | Set to target dependencies reference id strings                                   |

use anyhow::{anyhow, Ok, Result};
use cargo_metadata::{CargoOpt, MetadataCommand};
use clap::{Parser, ValueEnum};
use std::io::Write;
use std::{env, fmt::Debug, path::PathBuf};

mod graph;

mod util;

pub mod built_info {
  // The file has been placed there by the build script.
  include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(ValueEnum, Debug, Clone, PartialEq, Eq)]
#[clap(rename_all = "snake_case")]
#[allow(non_camel_case_types)]
enum OutputFormat {
  SpdxJson_2_3,
  CycloneDxJson_1_4,
  CycloneDxJson_1_5,
  CycloneDxJson_1_6,
}

fn get_default_cargo_manifest_path() -> PathBuf {
  PathBuf::from(".")
}

#[derive(Parser)]
#[clap(
  bin_name = "cargo sbom",
  about = "Cargo subcommand to produce a software bill of materials (SBOM)."
)]
#[command(author, version, about, long_about = None)]
struct Opt {
  #[clap(
    long,
    help = "The specific package (in a Cargo workspace) to generate an SBOM for. If not specified this is all packages in the workspace."
  )]
  cargo_package: Option<String>,
  #[clap(long, value_enum, help = "The SBOM output format.", default_value_t = OutputFormat::SpdxJson_2_3)]
  output_format: OutputFormat,
  #[clap(
    long,
    help = "The directory to the Cargo project.", default_value = get_default_cargo_manifest_path().into_os_string()
  )]
  project_directory: PathBuf,
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

  if !opt.project_directory.is_dir() {
    return Err(anyhow!(
      "Supplied project directory ({}) is not a directory.",
      opt.project_directory.to_string_lossy()
    ));
  }

  let mut cargo_manifest_path = opt.project_directory.clone();
  cargo_manifest_path.push("Cargo.toml");

  if !cargo_manifest_path.exists() {
    return Err(anyhow!(
      "Cargo manifest (Cargo.toml) does not exist in the supplied directory ({}).",
      opt.project_directory.canonicalize()?.to_string_lossy()
    ));
  }

  let metadata = MetadataCommand::new()
    .manifest_path(&cargo_manifest_path)
    .features(CargoOpt::AllFeatures)
    .exec()?;

  let graph = graph::build(&metadata)?;

  if matches!(opt.output_format, OutputFormat::CycloneDxJson_1_4) {
    let cyclonedx = util::cyclonedx::convert(
      opt.cargo_package,
      opt.project_directory,
      &graph,
    )?;
    writeln!(
      std::io::stdout(),
      "{}",
      serde_json::to_string_pretty(&cyclonedx)?
    )?;
  } else if matches!(opt.output_format, OutputFormat::CycloneDxJson_1_5) {
    let cyclonedx = util::cyclonedx::convert_1_5(
      opt.cargo_package,
      opt.project_directory,
      &graph,
    )?;
    writeln!(
      std::io::stdout(),
      "{}",
      serde_json::to_string_pretty(&cyclonedx)?
    )?;
  } else if matches!(opt.output_format, OutputFormat::CycloneDxJson_1_6) {
    let cyclonedx = util::cyclonedx::convert_1_6(
      opt.cargo_package,
      opt.project_directory,
      &graph,
    )?;
    writeln!(
      std::io::stdout(),
      "{}",
      serde_json::to_string_pretty(&cyclonedx)?
    )?;
  } else if matches!(opt.output_format, OutputFormat::SpdxJson_2_3) {
    let spdx = util::spdx::convert(
      opt.cargo_package,
      &opt.project_directory,
      &cargo_manifest_path,
      &graph,
    )?;
    writeln!(
      std::io::stdout(),
      "{}",
      serde_json::to_string_pretty(&spdx)?
    )?;
  }

  Ok(())
}
