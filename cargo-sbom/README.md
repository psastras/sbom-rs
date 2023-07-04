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
      "Tool: cargo-sbom-v0.1.0"
    ]
  },
  "dataLicense": "CC0-1.0",
  "documentNamespace": "https://docs.rs/cargo_sbom/spdxdocs/cargo-sbom-0.1.0-9cae390a-4b46-457c-95b9-e59a5e62b57d",
  "files": [
    {
  <rest of output omitted>
```

License: MIT
