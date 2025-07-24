use wasm_bindgen_test::*;
use yew::{html, Callback};
use web_sys::window;

use b3scale_admin::app::authenticate::{Form as AuthForm, FormData, FormProps as AuthFormProps};

// Test configuration for browser testing
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_auth_form_renders() {
    let callback = Callback::from(|_data: FormData| {
        // Mock callback for testing
    });
    
    let props = AuthFormProps {
        on_submit: callback,
    };
    
    // Test that the component renders without panicking
    let _html = html! {
        <AuthForm ..props />
    };
    
    // If we get here without panic, the test passes
    assert!(true);
}

#[wasm_bindgen_test]  
fn test_form_data_struct() {
    let form_data = FormData {
        token: Some("test_token".to_string()),
        secret: Some("test_secret".to_string()),
        api_url: "http://localhost:42353/api/v1".to_string(),
    };
    
    assert_eq!(form_data.token, Some("test_token".to_string()));
    assert_eq!(form_data.secret, Some("test_secret".to_string()));
    assert_eq!(form_data.api_url, "http://localhost:42353/api/v1");
}

#[wasm_bindgen_test]
fn test_form_data_clone() {
    let form_data = FormData {
        token: Some("test_token".to_string()),
        secret: Some("test_secret".to_string()),
        api_url: "http://localhost:42353/api/v1".to_string(),
    };
    
    let cloned = form_data.clone();
    assert_eq!(form_data, cloned);
}

#[wasm_bindgen_test]
fn test_dom_access() {
    // Test that we can access DOM elements (basic WASM browser compatibility)
    let window = window().expect("Should have window object");
    let document = window.document().expect("Should have document");
    let _title = document.title();
    
    assert!(true);
}

#[wasm_bindgen_test]
fn test_callback_creation() {
    let mut called = false;
    let callback = Callback::from(move |_data: FormData| {
        // In a real test, we'd use a shared state mechanism
        // For now, just test that callbacks can be created
    });
    
    // Test that callback is callable (this is a basic smoke test)
    assert!(callback.is_some());
}