[![Workflow Status](https://github.com/psastras/sbom-rs/workflows/main/badge.svg)](https://github.com/psastras/sbom-rs/actions?query=workflow%3A%22main%22)
[![codecov](https://codecov.io/gh/psastras/sbom-rs/branch/main/graph/badge.svg?token=KSXYAZGS5U)](https://codecov.io/gh/psastras/sbom-rs)

# sbom-rs

A group of Rust projects for interacting with and producing software bill of materials (SBOMs).

## Example

Create a SPDX SBOM for a Cargo project.

```shell
$ cargo sbom --sbom-format=spdx
```

## Install

`cargo-sbom` may be installed via `cargo`, [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) or directly downloaded from the
corresponding Github release.

### Cargo

```shell
cargo install cargo-sbom
```

### Cargo-binstall

```shell
cargo binstall cargo-sbom
```

### Github Releases

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
