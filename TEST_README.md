# 🧪 UI Testing Guide for b3scale-admin

This project now includes comprehensive UI tests for the Yew WebAssembly application!

## 🚀 Quick Start

### Run All Tests
```bash
# Using the convenient script
./scripts/run_tests.sh

# Or manually with wasm-pack
wasm-pack test --headless --chrome
```

### Prerequisites
1. **Install wasm-pack** (if not already installed):
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

2. **Add WebAssembly target**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

## 🧭 Test Structure

### `/tests/` Directory
- **`component_tests.rs`** - Tests for individual Yew components
- **`ui_tests.rs`** - Tests for UI rendering and styling
- **`integration_tests.rs`** - End-to-end UI workflow tests

### Key Test Categories

#### 🔧 Component Tests
- Authentication form validation
- Form data structure tests
- Callback functionality
- Component rendering without panics

#### 🎨 UI Tests  
- CSS class application
- Cyberpunk styling verification
- Form section accessibility
- Navigation structure
- Table rendering
- Alert message styling

#### 🔄 Integration Tests
- Complete form workflows
- API model integration
- Authentication flow
- List view structures
- Complex component interactions

## 🛠️ Running Specific Tests

### Run Individual Test Files
```bash
# Component tests only
wasm-pack test --headless --chrome -- --test component_tests

# UI tests only  
wasm-pack test --headless --chrome -- --test ui_tests

# Integration tests only
wasm-pack test --headless --chrome -- --test integration_tests
```

### Different Browsers
```bash
# Chrome (default)
wasm-pack test --headless --chrome

# Firefox
wasm-pack test --headless --firefox

# Safari (macOS only)
wasm-pack test --headless --safari
```

### Debug Mode (with browser window)
```bash
# Opens actual browser for debugging
wasm-pack test --chrome
```

## 🔍 Test Features

### ✅ What's Tested
- **Component Rendering**: All major UI components render without errors
- **Form Functionality**: Authentication and frontend/backend forms
- **Styling Verification**: Cyberpunk theme CSS classes apply correctly
- **Data Models**: API model creation and manipulation
- **Browser Compatibility**: Basic DOM access and Web APIs
- **Accessibility**: Form labels, text contrast, and structure

### 🎯 Accessibility Testing
Our tests specifically verify:
- High-contrast text with soft black backgrounds
- Proper form labeling and structure
- ARIA-compatible HTML structure
- Readable error and success messages

### 🔄 Continuous Integration Ready
Tests run in headless browsers, perfect for CI/CD pipelines:
```yaml
# Example GitHub Actions step
- name: Run UI Tests
  run: wasm-pack test --headless --chrome
```

## 📝 Adding New Tests

### Create a Component Test
```rust
use wasm_bindgen_test::*;
use yew::html;

#[wasm_bindgen_test]
fn test_my_component() {
    let html_content = html! {
        <MyComponent prop="value" />
    };
    
    // Test assertions here
    assert!(true);
}
```

### Test Styling
```rust
#[wasm_bindgen_test]
fn test_my_styling() {
    let styled_element = html! {
        <div class="form-section">
            <p class="form-text">{"Accessible text!"}</p>
        </div>
    };
    
    // Verify styling doesn't break rendering
    assert!(true);
}
```

## 🐛 Troubleshooting

### Common Issues

#### "wasm-bindgen-test not found"
```bash
cargo install wasm-bindgen-cli
```

#### "wasm32-unknown-unknown target not found"
```bash
rustup target add wasm32-unknown-unknown
```

#### Browser Not Found
- Install Chrome, Firefox, or Safari
- Use `--headless` flag for CI environments

#### Tests Timeout
- Increase timeout with `WASM_BINDGEN_TEST_TIMEOUT=60`
- Check console for JavaScript errors

### Debug Tips
1. Use `wasm-pack test --chrome` (no headless) to see browser window
2. Check browser console for detailed error messages
3. Use `gloo_console::log!()` in tests for debugging
4. Verify CSS files are being loaded correctly

## 🚀 Next Steps

The testing framework is ready for expansion! Consider adding:
- **Visual regression tests** with screenshot comparison
- **Performance tests** for rendering speed
- **Cross-browser compatibility** test suite
- **Accessibility audit** automation
- **API mocking** for isolated frontend tests

Happy testing! 🎉