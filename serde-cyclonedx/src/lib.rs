#![doc(html_root_url = "https://docs.rs/serde-cyclonedx/0.9.0")]

//! # serde-cyclonedx
//!
//! This crate provides a type safe [serde](https://serde.rs/) compatible
//! [CycloneDx](https://cyclonedx.org/) format. It is intended for use
//! in Rust code which may need to read or write CycloneDx files.
//!
//! The latest [documentation can be found here](https://docs.rs/serde_cyclonedx).
//!
//! serde is a popular serialization framework for Rust. More information can be
//! found on the official repository:
//! [https://github.com/serde-rs/serde](https://github.com/serde-rs/serde)
//!
//! CycloneDx is an industry standard format for maintaining a Software Bill of Materials (SBOM). More //! information can be found on
//! the official website:
//! [https://cyclonedx.org/](https://cyclonedx.org/).
//!
//! ## Usage
//!
//! For most cases, simply use the root [cyclonedx::v_1_4::CycloneDx] struct with [serde] to read
//! and write to and from the struct.
//!
//! ## Example
//!
//! ```rust
//! use serde_cyclonedx::cyclonedx::v_1_4::CycloneDx;
//!
//! let cyclonedx: CycloneDx = serde_json::from_str(r#"{
//!   "bomFormat": "CycloneDX",
//!   "serialNumber": "urn:uuid:e1afae7d-7cca-4894-9dac-3f400bc10c4c",
//!   "specVersion": "1.4",
//!   "version": 1
//! }"#).unwrap();
//! ```
//!
//! Because many of the [cyclonedx::v_1_4::CycloneDx] structures contain a lot of optional fields,
//! it is often convenient to use the builder pattern to contstruct these structs.
//! Each structure has a builder with a default.
//!
//! ## Example
//!
//! ```rust
//! use serde_cyclonedx::cyclonedx::v_1_4::CycloneDxBuilder;
//!
//! let cyclonedx = CycloneDxBuilder::default()
//!   .bom_format("CycloneDX")
//!   .spec_version("1.4")
//!   .version(1)
//!   .build()
//!   .unwrap();
//! ```
//!
//! ## Internal Implementation Details
//!
//! The root struct is automatically generated from the parsed CycloneDX JSON schemas, this is done at build time (via the buildscript).
//!

pub mod cyclonedx;
