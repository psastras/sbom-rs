[![Workflow Status](https://github.com/psastras/sbom-rs/workflows/main/badge.svg)](https://github.com/psastras/sbom-rs/actions?query=workflow%3A%22main%22)

# serde-spdx

This crate provides a type safe [serde](https://serde.rs/) compatible
[SPDX](https://spdx.dev/) format. It is intended for use
in Rust code which may need to read or write SPDX files.

The latest [documentation can be found here](https://docs.rs/serde_spdx).

serde is a popular serialization framework for Rust. More information can be
found on the official repository:
[https://github.com/serde-rs/serde](https://github.com/serde-rs/serde)

SDPX is an industry standard format for maintaining a Software Bill of Materials (SBOM). More information can be found on
the official website:
[https://spdx.dev/](https://spdx.dev/).

## Usage

For most cases, simply use the root [spdx::v_2_3::Spdx] struct with [serde] to read
and write to and from the struct.

## Example

```rust
use serde_spdx::spdx::v_2_3::Spdx;

let data = fs::read_to_string("sbom.spdx.json");
let spdx: Spdx = serde_json::from_str(&data).unwrap();
```

Because many of the [spdx::v_2_3::Spdx] structures contain a lot of optional fields,
it is often convenient to use the builder pattern to contstruct these structs.
Each structure has a builder with a default.

## Example

```rust
use serde_spdx::spdx::v_2_3::SpdxCreationInfoBuilder;

let creation_info = SpdxCreationInfoBuilder::default()
  .created("created")
  .creators(vec![])
  .build()
  .unwrap();
```

## Internal Implementation Details

The root struct is automatically generated from the parsed SPDX
JSON schema, this is done at build time (via the buildscript).

License: MIT
