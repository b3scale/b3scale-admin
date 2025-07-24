// Test module to include and run all UI tests
use wasm_bindgen_test::*;

// Configure wasm-bindgen-test for browser testing
wasm_bindgen_test_configure!(run_in_browser);

// Include test modules
include!("../tests/component_tests.rs");
include!("../tests/ui_tests.rs"); 
include!("../tests/integration_tests.rs");

#[wasm_bindgen_test]
fn test_lib_loads() {
    // Basic smoke test to ensure the library loads
    assert!(true);
}