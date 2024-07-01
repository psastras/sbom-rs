#![allow(clippy::derive_partial_eq_without_eq, clippy::empty_docs)]

pub mod v_1_4 {
  include!(concat!(env!("OUT_DIR"), "/cyclonedx_1_4.rs"));
}

#[allow(clippy::large_enum_variant)]
pub mod v_1_5 {
  include!(concat!(env!("OUT_DIR"), "/cyclonedx_1_5.rs"));
}

#[allow(clippy::large_enum_variant)]
pub mod v_1_6 {
  include!(concat!(env!("OUT_DIR"), "/cyclonedx_1_6.rs"));
}
