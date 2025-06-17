use serde_spdx::spdx::v_3_0_1::Spdx;

#[test]
fn test_spdx_3_0_1_basic_structure() {
    let spdx_json = r#"{
        "@context": "https://spdx.org/rdf/3.0.1/spdx-context.jsonld",
        "@graph": [
            {
                "type": "CreationInfo",
                "@id": "_:creationinfo",
                "createdBy": ["http://spdx.example.com/Agent/JoshuaWatt"],
                "specVersion": "3.0.1",
                "created": "2024-03-06T00:00:00Z"
            }
        ]
    }"#;

    let spdx: Result<Spdx, _> = serde_json::from_str(spdx_json);
    assert!(spdx.is_ok(), "Failed to parse SPDX 3.0.1: {:?}", spdx);
    
    let spdx = spdx.unwrap();
    assert_eq!(spdx._context, "https://spdx.org/rdf/3.0.1/spdx-context.jsonld");
    assert_eq!(spdx._graph.len(), 1);
}

#[test]
fn test_spdx_3_0_1_package_sbom() {
    let spdx_json = r#"{
        "@context": "https://spdx.org/rdf/3.0.1/spdx-context.jsonld",
        "@graph": [
            {
                "type": "CreationInfo",
                "@id": "_:creationinfo",
                "createdBy": ["http://spdx.example.com/Agent/JoshuaWatt"],
                "specVersion": "3.0.1",
                "created": "2024-03-06T00:00:00Z"
            },
            {
                "type": "Person",
                "spdxId": "http://spdx.example.com/Agent/JoshuaWatt",
                "name": "Joshua Watt",
                "creationInfo": "_:creationinfo"
            },
            {
                "type": "SpdxDocument",
                "spdxId": "http://spdx.example.com/Document1",
                "creationInfo": "_:creationinfo",
                "rootElement": ["http://spdx.example.com/BOM1"],
                "profileConformance": ["core", "software"]
            }
        ]
    }"#;

    let spdx: Result<Spdx, _> = serde_json::from_str(spdx_json);
    assert!(spdx.is_ok(), "Failed to parse SPDX 3.0.1 package SBOM: {:?}", spdx);
    
    let spdx = spdx.unwrap();
    assert_eq!(spdx._graph.len(), 3);
}