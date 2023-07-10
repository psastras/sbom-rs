#![allow(clippy::derive_partial_eq_without_eq)]

pub mod v_1_4 {
  include!(concat!(env!("OUT_DIR"), "/cyclonedx_1_4.rs"));
}

pub mod v_1_5 {
  include!(concat!(env!("OUT_DIR"), "/cyclonedx_1_5.rs"));
}
