use std::{
  fs::{self},
  path::PathBuf,
};

use anyhow::Result;
use pretty_assertions::assert_eq;

#[test]
fn test_cargo_binary_spdx_example() -> Result<()> {
  let project_path = PathBuf::from("./examples/cargo-binary");
  let mut assert_cmd = assert_cmd::Command::from_std(
    escargot::CargoBuild::new()
      .manifest_path("../Cargo.toml")
      .bin("cargo-sbom")
      .env("RUSTFLAGS", "-C instrument-coverage")
      .run()
      .unwrap()
      .command(),
  );
  assert_cmd
    .arg("--project-directory")
    .arg(&project_path)
    .arg("--output-format")
    .arg("spdx_json_2_3");

  let mut project_sbom_path = project_path.clone();
  project_sbom_path.push("sbom.spdx.json");
  let expected_sbom: serde_spdx::spdx::v_2_3::Spdx =
    serde_json::from_str(&fs::read_to_string(project_sbom_path)?)?;

  let cmd = assert_cmd.assert().success();
  let output = cmd.get_output();
  let output_sbom: serde_spdx::spdx::v_2_3::Spdx =
    serde_json::from_slice(output.stdout.as_slice())?;

  assert_eq!(
    expected_sbom
      .packages
      .unwrap()
      .sort_by_key(|k| k.spdxid.clone()),
    output_sbom
      .packages
      .unwrap()
      .sort_by_key(|k| k.spdxid.clone())
  );

  assert_eq!(
    expected_sbom
      .relationships
      .unwrap()
      .sort_by_key(|k| k.spdx_element_id.clone()),
    output_sbom
      .relationships
      .unwrap()
      .sort_by_key(|k| k.spdx_element_id.clone())
  );
  Ok(())
}

#[test]
fn test_cargo_binary_cyclonedx_example() -> Result<()> {
  let project_path = PathBuf::from("./examples/cargo-binary");
  let mut assert_cmd = assert_cmd::Command::from_std(
    escargot::CargoBuild::new()
      .manifest_path("../Cargo.toml")
      .bin("cargo-sbom")
      .env("RUSTFLAGS", "-C instrument-coverage")
      .run()
      .unwrap()
      .command(),
  );
  assert_cmd
    .arg("--project-directory")
    .arg(&project_path)
    .arg("--output-format")
    .arg("cyclone_dx_json_1_4");

  let mut project_sbom_path = project_path.clone();
  project_sbom_path.push("sbom.cyclonedx.json");
  let expected_sbom: serde_cyclonedx::cyclonedx::v_1_4::CycloneDx =
    serde_json::from_str(&fs::read_to_string(project_sbom_path)?)?;

  let cmd = assert_cmd.assert().success();
  let output = cmd.get_output();
  let output_sbom: serde_cyclonedx::cyclonedx::v_1_4::CycloneDx =
    serde_json::from_slice(output.stdout.as_slice())?;

  assert_eq!(
    expected_sbom
      .components
      .unwrap()
      .sort_by_key(|k| k.bom_ref.clone()),
    output_sbom
      .components
      .unwrap()
      .sort_by_key(|k| k.bom_ref.clone())
  );

  assert_eq!(
    expected_sbom
      .dependencies
      .unwrap()
      .sort_by_key(|k| k.ref_.clone()),
    output_sbom
      .dependencies
      .unwrap()
      .sort_by_key(|k| k.ref_.clone())
  );
  Ok(())
}

#[test]
fn test_cargo_binary_cyclonedx_1_6_example() -> Result<()> {
  let project_path = PathBuf::from("./examples/cargo-binary");
  let mut assert_cmd = assert_cmd::Command::from_std(
    escargot::CargoBuild::new()
      .manifest_path("../Cargo.toml")
      .bin("cargo-sbom")
      .env("RUSTFLAGS", "-C instrument-coverage")
      .run()
      .unwrap()
      .command(),
  );
  assert_cmd
    .arg("--project-directory")
    .arg(&project_path)
    .arg("--output-format")
    .arg("cyclone_dx_json_1_6");

  let cmd = assert_cmd.assert().success();
  let output = cmd.get_output();
  let output_sbom: serde_cyclonedx::cyclonedx::v_1_6::CycloneDx =
    serde_json::from_slice(output.stdout.as_slice())?;

  // Basic validation that the CycloneDX 1.6 format is working
  assert_eq!(output_sbom.bom_format, "CycloneDX");
  assert_eq!(output_sbom.spec_version, "1.6");
  assert_eq!(output_sbom.version, Some(1));

  // Ensure we have components and metadata
  assert!(output_sbom.components.is_some());
  assert!(output_sbom.metadata.is_some());
  assert!(output_sbom.components.unwrap().len() > 0);

  Ok(())
}

#[test]
fn test_cargo_binary_cyclonedx_1_5_example() -> Result<()> {
  let project_path = PathBuf::from("./examples/cargo-binary");
  let mut assert_cmd = assert_cmd::Command::from_std(
    escargot::CargoBuild::new()
      .manifest_path("../Cargo.toml")
      .bin("cargo-sbom")
      .env("RUSTFLAGS", "-C instrument-coverage")
      .run()
      .unwrap()
      .command(),
  );
  assert_cmd
    .arg("--project-directory")
    .arg(&project_path)
    .arg("--output-format")
    .arg("cyclone_dx_json_1_5");

  let cmd = assert_cmd.assert().success();
  let output = cmd.get_output();
  let output_sbom: serde_cyclonedx::cyclonedx::v_1_5::CycloneDx =
    serde_json::from_slice(output.stdout.as_slice())?;

  // Basic validation that the CycloneDX 1.5 format is working
  assert_eq!(output_sbom.bom_format, "CycloneDX");
  assert_eq!(output_sbom.spec_version, "1.5");
  assert_eq!(output_sbom.version, Some(1));

  // Ensure we have components and metadata
  assert!(output_sbom.components.is_some());
  assert!(output_sbom.metadata.is_some());
  assert!(output_sbom.components.unwrap().len() > 0);

  Ok(())
}
