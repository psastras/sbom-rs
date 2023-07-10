#![doc(html_root_url = "https://docs.rs/serde-spdx/0.8.0")]

//! # serde-spdx
//!
//! This crate provides a type safe [serde](https://serde.rs/) compatible
//! [SPDX](https://spdx.dev/) format. It is intended for use
//! in Rust code which may need to read or write SPDX files.
//!
//! The latest [documentation can be found here](https://docs.rs/serde_spdx).
//!
//! serde is a popular serialization framework for Rust. More information can be
//! found on the official repository:
//! [https://github.com/serde-rs/serde](https://github.com/serde-rs/serde)
//!
//! SDPX is an industry standard format for maintaining a Software Bill of Materials (SBOM). More information can be found on
//! the official website:
//! [https://spdx.dev/](https://spdx.dev/).
//!
//! ## Usage
//!
//! For most cases, simply use the root [spdx::Spdx] struct with [serde] to read
//! and write to and from the struct.
//!
//! ## Example
//!
//! ```rust
//! use serde_spdx::spdx::Spdx;
//!
//! let data = fs::read_to_string("sbom.spdx.json");
//! let spdx: Spdx = serde_json::from_str(&data).unwrap();
//! ```
//!
//! Because many of the [spdx::SPDX] structures contain a lot of optional fields,
//! it is often convenient to use the builder pattern to contstruct these structs.
//! Each structure has a builder with a default.
//!
//! ## Example
//!
//! ```rust
//! use serde_spdx::spdx::SpdxCreationInfoBuilder;
//!
//! let creation_info = SpdxCreationInfoBuilder::default()
//!   .created("created")
//!   .creators(vec![])
//!   .build()
//!   .unwrap();
//! ```
//!
//! ## Internal Implementation Details
//!
//! The root [spdx::Spdx] struct is automatically generated from the latest SPDX
//! JSON schema, this is done at build time (via the buildscript).

pub mod spdx;
