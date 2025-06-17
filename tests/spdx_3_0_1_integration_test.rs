use std::process::Command;

#[test]
fn test_cargo_sbom_spdx_3_0_1_output() {
    let output = Command::new("cargo")
        .args(&["run", "-p", "cargo-sbom", "--", "sbom", "--output-format=spdx_json_3_0_1"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo-sbom");

    assert!(output.status.success(), 
        "cargo-sbom command failed. stdout: {}, stderr: {}", 
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    
    // Parse the JSON output to verify it's valid SPDX 3.0.1
    let spdx_value: serde_json::Value = serde_json::from_str(&stdout)
        .expect("Failed to parse SPDX 3.0.1 JSON output");
    
    // Verify it has the correct context
    assert_eq!(
        spdx_value["@context"], 
        "https://spdx.org/rdf/3.0.1/spdx-context.jsonld"
    );
    
    // Verify it has a graph
    assert!(spdx_value["@graph"].is_array());
    let graph = spdx_value["@graph"].as_array().unwrap();
    assert!(!graph.is_empty());
    
    // Verify we have a CreationInfo element
    let has_creation_info = graph.iter().any(|element| {
        element["type"] == "CreationInfo"
    });
    assert!(has_creation_info, "SPDX 3.0.1 output should contain a CreationInfo element");
    
    // Verify we have software_Package elements
    let has_package = graph.iter().any(|element| {
        element["type"] == "software_Package"
    });
    assert!(has_package, "SPDX 3.0.1 output should contain software_Package elements");
}