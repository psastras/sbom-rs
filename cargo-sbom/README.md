[![Workflow Status](https://github.com/psastras/sbom-rs/workflows/main/badge.svg)](https://github.com/psastras/sbom-rs/actions?query=workflow%3A%22main%22)

# cargo-sbom

This crate provides a command line tool to create software bill of materials (SBOM) for Cargo / Rust workspaces. It supports both SPDX and CycloneDX outputs.

The latest [documentation can be found here](https://docs.rs/cargo_sbom).

SBOM or Software Bill of Materials is an industry standard term used to trace and maintain the supply chain security of software.

## Installation

`cargo-sbom` may be installed via `cargo`

```shell
cargo install cargo-sbom
```

via [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```shell
cargo binstall cargo-sbom
```

or downloaded directly from Github Releases

```shell
# make sure to adjust the target and version (you may also want to pin to a specific version)
curl -sSL https://github.com/psastras/sbom-rs/releases/download/cargo-sbom-latest/cargo-sbom-x86_64-unknown-linux-gnu -o cargo-sbom
```

## Usage

For most cases, simply `cd` into a cargo workspace and run `cargo sbom`.

## Example

```shell
$ cargo sbom
{
  "SPDXID": "SPDXRef-DOCUMENT",
  "creationInfo": {
    "created": "2023-07-04T12:38:15.211Z",
    "creators": [
      "Tool: cargo-sbom-v0.5.0"
    ]
  },
  "dataLicense": "CC0-1.0",
  "documentNamespace": "https://docs.rs/cargo_sbom/spdxdocs/cargo-sbom-0.5.0-9cae390a-4b46-457c-95b9-e59a5e62b57d",
  "files": [
    {
  <rest of output omitted>
```

More examples can be found by browsing the [examples section](https://github.com/psastras/sbom-rs/tree/main/cargo-sbom/examples).

## Supported SBOM Features

### SPDX

| SPDX Field                       | Source                                                                                       |
|----------------------------------|----------------------------------------------------------------------------------------------|
| SPDXID                           | Set to "SPDXRef-Document"                                                                    |
| creationInfo.created             | Set as the current time                                                                      |
| creationInfo.creators            | Set to "Tool: cargo-sbom-v(tool version)                                                     |
| dataLicense                      | Set to "CC0-1.0"                                                                             |
| documentNamespace                | set to "https://spdx.org/spdxdocs/(crate-name)-(uuidv4)"                                     |
| files                            | parsed from Cargo.toml target names                                                          |
| name                             | Set to the project folder name                                                               |
| packages                         | Set to dependencies parsed from cargo-metadata                                               |
| packages.SPDXID                  | Written as SPDXRef-Package-(crate name)-(crate version)                                      |
| packages.description             | Read from Cargo.toml's "description" field                                                   |
| packages.downloadLocation        | Read from `cargo metadata` (usually "registry+https://github.com/rust-lang/crates.io-index") |
| packages.externalRefs            | If packages.downloadLocation is crates.io, written as a package url formatted string         |
| packages.homepage                | Read from Cargo.toml's "homepage" field                                                      |
| packages.licenseConcluded        | Parsed into a SPDX compliant license identifier from Cargo.toml's "license" field            |
| packages.licenseDeclared         | Read from Cargo.toml's "license" field                                                       |
| packages.name                    | Read from Cargo.toml's "name" field                                                          |
| relationships                    | Set to dependency relationships parsed from cargo-metadata                                   |
| relationships.relationshipType   | Set to dependency relationship parsed from cargo-metadata                                    |
| relationships.spdxElementId      | Set to dependency relationship source parsed from cargo-metadata                             |
| relationships.relatedSpdxElement | Set to dependency relationship target parsed from cargo-metadata                             |



### CycloneDx

None

License: MIT
