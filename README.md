[![Workflow Status](https://github.com/psastras/sbom-rs/workflows/main/badge.svg)](https://github.com/psastras/sbom-rs/actions?query=workflow%3A%22main%22)
[![codecov](https://codecov.io/gh/psastras/sbom-rs/branch/main/graph/badge.svg?token=KSXYAZGS5U)](https://codecov.io/gh/psastras/sbom-rs)

# sbom-rs

A group of Rust projects for interacting with and producing software bill of materials (SBOMs).

## Examples

### cargo-sbom

#### Create a SPDX SBOM for a Cargo project

In a shell:

```shell
$ cargo sbom
{
  "SPDXID": "SPDXRef-DOCUMENT",
  "creationInfo": {
    "created": "2023-07-04T12:38:15.211Z",
    "creators": [
      "Tool: cargo-sbom-v0.9.1"
    ]
  },
  "dataLicense": "CC0-1.0",
  "documentNamespace": "https://docs.rs/cargo_sbom/spdxdocs/cargo-sbom-0.9.1-9cae390a-4b46-457c-95b9-e59a5e62b57d",
  "files": [
    {
  <rest of output omitted>
```

#### Create a CycloneDx SBOM in Github Actions

In a Github Actions workflow:

```yaml
jobs:
  sbom:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: psastras/sbom-rs/actions/install-cargo-sbom@cargo-sbom-latest
    - name: Run cargo-sbom
      run: cargo-sbom --output-format=cyclone_dx_json_1_4
```

#### Create a CycloneDx 1.5 SBOM in Github Actions

```yaml
jobs:
  sbom:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: psastras/sbom-rs/actions/install-cargo-sbom@cargo-sbom-latest
    - name: Run cargo-sbom
      run: cargo-sbom --output-format=cyclone_dx_json_1_5
```

#### Create a CycloneDx 1.6 SBOM in Github Actions

```yaml
jobs:
  sbom:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: psastras/sbom-rs/actions/install-cargo-sbom@cargo-sbom-latest
    - name: Run cargo-sbom
      run: cargo-sbom --output-format=cyclone_dx_json_1_6
```

#### Check Dependencies against the Open Source Vulnerability Database (OSV)

Assumming `osv-scanner` is installed (see [https://osv.dev/](https://osv.dev/))

```shell
$ cargo-sbom > sbom.spdx.json
$ osv-scanner --sbom=sbom.spdx.json
Scanned sbom.json as SPDX SBOM and found 91 packages
╭─────────────────────────────────────┬──────┬───────────┬─────────┬─────────┬───────────╮
│ OSV URL                             │ CVSS │ ECOSYSTEM │ PACKAGE │ VERSION │ SOURCE    │
├─────────────────────────────────────┼──────┼───────────┼─────────┼─────────┼───────────┤
│ https://osv.dev/GHSA-wcg3-cvx6-7396 │ 6.2, │ crates.io │ time    │ 0.1.45  │ sbom.json │
│ https://osv.dev/RUSTSEC-2020-0071   │ 6.2  │           │         │         │           │
╰─────────────────────────────────────┴──────┴───────────┴─────────┴─────────┴───────────╯
```

## Install

### cargo-sbom

`cargo-sbom` may be installed via `cargo`, [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) or directly downloaded from the
corresponding Github release.

#### Cargo

```shell
cargo install cargo-sbom
```

#### Cargo-binstall

```shell
cargo binstall cargo-sbom
```

#### Github Releases

The latest version is
[continuously published and tagged](https://github.com/psastras/sbom-rs/releases).

Using `curl`,

```shell
# make sure to adjust the target and version (you may also want to pin to a specific version)
curl -sSL https://github.com/psastras/sbom-rs/releases/download/cargo-sbom-latest/cargo-sbom-x86_64-unknown-linux-gnu -o cargo-sbom
```

## Documentation

See each subproject for more detailed information:

- `cargo-sbom`: CLI tool to produce an SBOM from a Cargo workspace.
  See the [Rust documentation](https://docs.rs/cargo_sbom/).
- `serde-cyclonedx`: Typesafe CycloneDX structures for serializing and deserializing
  CycloneDX information using [serde](https://serde.rs/). See the
  [Rust documentation](https://docs.rs/serde_cyclonedx/).
- `serde-spdx`: Typesafe SPDX structures for serializing and deserializing
  SPDX information using [serde](https://serde.rs/). See the
  [Rust documentation](https://docs.rs/serde_spdx/).
- `actions`: Github actions to use `cargo-sbom` and related tools in CI workflows See the [README.md](https://github.com/psastras/sbom-rs/tree/main/actions/README.md) for documentaiton.

[Also check the examples.](https://github.com/psastras/sbom-rs/tree/main/examples)

## Development

Before you begin, ensure the following programs are available on your machine:

- [`cargo`](https://rustup.rs/)

Assuming `cargo` is installed on your machine, the standard `cargo` commands can
be run to build and test all projects in the workspace:

```shell
cargo build
cargo test
```

For more information on specific configurations, refer to the
[`cargo` documentation](https://doc.rust-lang.org/cargo).

### Releasing

To release a new version (publish to crates.io), prefix the head commit with `release:` and update the relevant rust crate versions. Once merged into main the pipeline should pick up the change and publish a new version.

License: MIT
