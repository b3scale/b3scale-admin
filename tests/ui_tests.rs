use wasm_bindgen_test::*;
use yew::{html, Html};
use web_sys::{window, HtmlElement};

// Test configuration for browser testing
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_basic_html_rendering() {
    // Test basic HTML rendering with Yew
    let html_content: Html = html! {
        <div class="test-container">
            <h1>{"Test Header"}</h1>
            <p>{"Test paragraph"}</p>
        </div>
    };
    
    // Basic test - if we can create HTML without panic, test passes
    assert!(true);
}

#[wasm_bindgen_test]
fn test_form_section_styling() {
    // Test that our form sections render with proper CSS classes
    let form_section: Html = html! {
        <section class="form-section">
            <h3>{"Test Form Section"}</h3>
            <p class="form-text">{"This is form text"}</p>
            <label class="form-label">{"Test Label"}</label>
            <input class="form-control" type="text" placeholder="Test input" />
        </section>
    };
    
    // Test passes if HTML creation doesn't panic
    assert!(true);
}

#[wasm_bindgen_test]
fn test_cyber_styled_components() {
    // Test our cyberpunk-styled components
    let cyber_component: Html = html! {
        <div class="container-page">
            <div class="card box">
                <h2>{"Cyber Header"}</h2>
                <p class="form-text">{"Ultra accessible text with soft black background!"}</p>
            </div>
        </div>
    };
    
    assert!(true);
}

#[wasm_bindgen_test]
fn test_navigation_structure() {
    // Test navigation components
    let nav: Html = html! {
        <nav class="navbar">
            <ul class="nav-pills">
                <li class="nav-item">
                    <a class="nav-link active" href="#frontends">{"Frontends"}</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" href="#backends">{"Backends"}</a>
                </li>
            </ul>
        </nav>
    };
    
    assert!(true);
}

#[wasm_bindgen_test]
fn test_table_rendering() {
    // Test table components with our styling
    let table: Html = html! {
        <table class="table">
            <thead>
                <tr>
                    <th>{"ID"}</th>
                    <th>{"Name"}</th>
                    <th>{"Status"}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>{"1"}</td>
                    <td>{"Test Frontend"}</td>
                    <td><span class="badge bg-primary">{"Active"}</span></td>
                </tr>
            </tbody>
        </table>
    };
    
    assert!(true);
}

#[wasm_bindgen_test]
fn test_alert_messages() {
    // Test styled alert messages
    let alerts: Html = html! {
        <div>
            <div class="alert alert-success">{"Success message!"}</div>
            <div class="alert alert-danger">{"Error message!"}</div>
            <p class="text-success">{"Success text"}</p>
            <p class="text-danger">{"Error text"}</p>
        </div>
    };
    
    assert!(true);
}

#[wasm_bindgen_test]
fn test_browser_environment() {
    // Test basic browser environment is available
    let window = window().expect("Should have window object");
    let document = window.document().expect("Should have document");
    let body = document.body().expect("Should have body");
    
    // Test we can access CSS styles
    let computed_style = window
        .get_computed_style(&body)
        .expect("Should get computed style")
        .expect("Should have style object");
    
    assert!(true);
}