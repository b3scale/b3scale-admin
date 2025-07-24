// Test module for b3scale-admin UI components
// Configure wasm-bindgen-test for browser testing

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub mod ui_tests;
pub mod component_tests;
pub mod integration_tests;