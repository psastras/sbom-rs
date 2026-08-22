// ABOUTME: Tests the generated CycloneDx 1.7 bindings in serde-cyclonedx.
// ABOUTME: Covers deserialization, round tripping and the builder API.
use anyhow::Result;
use pretty_assertions::assert_eq;
use serde_cyclonedx::cyclonedx::v_1_7::{
  AlgorithmFamiliesEnum, ComponentBuilder,
  CryptoPropertiesAlgorithmPropertiesBuilder, CryptoPropertiesBuilder,
  CycloneDx, CycloneDxBuilder, EllipticCurvesEnum,
};

const BOM_1_7: &str = r#"{
  "bomFormat": "CycloneDX",
  "specVersion": "1.7",
  "serialNumber": "urn:uuid:e1afae7d-7cca-4894-9dac-3f400bc10c4c",
  "version": 1,
  "components": [
    {
      "type": "library",
      "bom-ref": "pkg:cargo/serde@1.0.229",
      "name": "serde",
      "version": "1.0.229",
      "isExternal": true
    },
    {
      "type": "cryptographic-asset",
      "bom-ref": "crypto/ecdsa-p256",
      "name": "ECDSA-P256",
      "cryptoProperties": {
        "assetType": "algorithm",
        "algorithmProperties": {
          "primitive": "signature",
          "algorithmFamily": "ECDSA",
          "ellipticCurve": "secg/secp256r1",
          "nistQuantumSecurityLevel": 0
        }
      }
    }
  ],
  "citations": [
    {
      "timestamp": "2026-08-21T00:00:00Z",
      "attributedTo": "pkg:cargo/serde@1.0.229",
      "pointers": ["/components/0/version"],
      "note": "Version read from Cargo.lock"
    }
  ]
}"#;

#[test]
fn test_deserialize_cyclonedx_1_7() -> Result<()> {
  let bom: CycloneDx = serde_json::from_str(BOM_1_7)?;

  assert_eq!(bom.bom_format, "CycloneDX");
  assert_eq!(bom.spec_version, "1.7");
  assert_eq!(bom.version, Some(1));

  let components = bom.components.expect("components");
  assert_eq!(components.len(), 2);

  // `isExternal` and `versionRange` are new in 1.7
  assert_eq!(components[0].name, "serde");
  assert_eq!(components[0].version.as_deref(), Some("1.0.229"));
  assert_eq!(components[0].is_external, Some(true));

  // the cryptography enums are defined in the cryptography-defs subschema
  let crypto_properties = components[1]
    .crypto_properties
    .as_ref()
    .expect("cryptoProperties");
  assert_eq!(crypto_properties.asset_type, "algorithm");
  let algorithm_properties = crypto_properties
    .algorithm_properties
    .as_ref()
    .expect("algorithmProperties");
  assert_eq!(
    algorithm_properties.algorithm_family,
    Some(AlgorithmFamiliesEnum::Ecdsa)
  );
  assert_eq!(
    algorithm_properties.elliptic_curve,
    Some(EllipticCurvesEnum::SecgSecp256R1)
  );

  // `citations` is new in 1.7
  let citations = bom.citations.expect("citations");
  assert_eq!(citations.len(), 1);
  assert_eq!(citations[0].timestamp, "2026-08-21T00:00:00Z");
  assert_eq!(
    citations[0].pointers.as_deref(),
    Some(["/components/0/version".to_string()].as_slice())
  );

  Ok(())
}

#[test]
fn test_round_trip_cyclonedx_1_7() -> Result<()> {
  let bom: CycloneDx = serde_json::from_str(BOM_1_7)?;
  let round_tripped: CycloneDx =
    serde_json::from_str(&serde_json::to_string(&bom)?)?;

  assert_eq!(bom, round_tripped);

  Ok(())
}

#[test]
fn test_build_cyclonedx_1_7() -> Result<()> {
  let bom = CycloneDxBuilder::default()
    .bom_format("CycloneDX")
    .spec_version("1.7")
    .version(1)
    .components(vec![ComponentBuilder::default()
      .type_("cryptographic-asset")
      .name("ML-KEM-768")
      .crypto_properties(
        CryptoPropertiesBuilder::default()
          .asset_type("algorithm")
          .algorithm_properties(
            CryptoPropertiesAlgorithmPropertiesBuilder::default()
              .primitive("kem")
              .algorithm_family(AlgorithmFamiliesEnum::MlKem)
              .parameter_set_identifier("768")
              .build()?,
          )
          .build()?,
      )
      .build()?])
    .build()?;

  let serialized = serde_json::to_value(&bom)?;
  assert_eq!(serialized["specVersion"], "1.7");
  assert_eq!(
    serialized["components"][0]["cryptoProperties"]["algorithmProperties"]
      ["algorithmFamily"],
    "ML-KEM"
  );

  Ok(())
}

// The variant name of a value starting with a digit cannot be the value
// itself, so serialization relies entirely on its rename attribute.
#[test]
fn test_algorithm_family_with_numeric_start() -> Result<()> {
  let family: AlgorithmFamiliesEnum = serde_json::from_str("\"3DES\"")?;

  assert_eq!(family, AlgorithmFamiliesEnum::_3Des);
  assert_eq!(serde_json::to_string(&family)?, "\"3DES\"");

  Ok(())
}
