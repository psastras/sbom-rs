use anyhow::Result;
use std::fs;
use std::iter::FromIterator;
use std::path::PathBuf;

#[test]
fn test_spdx() -> Result<()> {
  let cargo_manifest_directory =
    fs::canonicalize(PathBuf::from(env!("CARGO_MANIFEST_DIR")))?;
  let cargo_workspace_directory = fs::canonicalize(PathBuf::from_iter(
    [cargo_manifest_directory.clone(), PathBuf::from("..")].iter(),
  ))?;

  let cargo_manifest_directory = fs::canonicalize(PathBuf::from_iter(
    [cargo_manifest_directory, PathBuf::from("./tests/data/spdx")].iter(),
  ))?;

  duct_sh::sh(
    "RUSTFLAGS='-C instrument-coverage' cargo build --bin cargo-sbom",
  )
  .dir(cargo_workspace_directory.clone())
  .run()?;

  let cargo_sbom_bin = fs::canonicalize(PathBuf::from_iter(
    [
      cargo_workspace_directory.clone(),
      PathBuf::from("./target/debug/cargo-sbom"),
    ]
    .iter(),
  ))?;

  let cmd =
    format!("{} --output-format=spdx", cargo_sbom_bin.to_str().unwrap());

  let output = duct_sh::sh_dangerous(cmd.as_str())
    .dir(cargo_manifest_directory)
    .unchecked()
    .env("NO_COLOR", "1")
    .read()?;
  Ok(())
}
