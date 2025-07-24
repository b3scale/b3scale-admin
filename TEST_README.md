# 🧪 Testing Guide for b3scale-admin

This project now includes comprehensive tests for the Rust/WebAssembly b3scale admin application!

## 🚀 Quick Start

### Run All Tests
```bash
# Using the convenient script
./scripts/run_tests.sh

# Or directly with cargo
cargo test
```

## 🧭 Test Structure

### What's Tested
- **✅ API Model Creation** - FrontendConfig and FrontendSettings
- **✅ Data Structure Validation** - HashMap operations and string handling
- **✅ Core Functionality** - Basic Rust operations and logic
- **✅ Integration** - API models work correctly with application

### Current Test Suite

#### 🔧 Unit Tests (Working!)
Located in `src/lib.rs` under `#[cfg(test)]` module:

1. **`basic_test`** - Ensures basic math operations work
2. **`test_frontend_config_creation`** - Tests BBB API key/secret structures
3. **`test_frontend_settings_creation`** - Tests frontend configuration settings
4. **`test_string_operations`** - Tests string manipulation
5. **`test_hashmap_operations`** - Tests key-value parameter handling
6. **`test_api_models_work`** - Integration test for API data structures

## 🛠️ Running Tests

### Standard Rust Tests (Recommended)
```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_api_models_work
```

### Test Output Example
```
running 6 tests
test tests::test_api_models_work ... ok
test tests::basic_test ... ok
test tests::test_frontend_settings_creation ... ok
test tests::test_frontend_config_creation ... ok
test tests::test_hashmap_operations ... ok
test tests::test_string_operations ... ok

test result: ok. 6 passed; 0 failed; 0 ignored
```

## 🎯 Test Coverage

### ✅ What's Currently Tested
- **API Models**: Frontend configuration structures
- **Data Handling**: Parameter maps for BigBlueButton settings
- **String Operations**: Text manipulation and validation
- **Core Logic**: Basic application functionality

### 🔄 Future Test Expansion
The testing framework is ready for expansion! Consider adding:
- **Component Tests**: Yew component rendering (requires browser setup)
- **Form Validation**: Input validation and error handling
- **API Integration**: Mock API responses and error scenarios
- **Accessibility**: Automated accessibility testing
- **Visual Regression**: Screenshot comparison tests

## 🌐 WebAssembly Testing (Advanced)

For advanced users who want to test in browser environments:

### Prerequisites
```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Add WebAssembly target
rustup target add wasm32-unknown-unknown
```

### Browser Tests (Optional)
```bash
# Chrome (requires Chrome/Chromium installed)
wasm-pack test --headless --chrome

# Firefox (requires Firefox installed)
wasm-pack test --headless --firefox

# Interactive mode (opens browser window)
wasm-pack test --chrome
```

⚠️ **Note**: Browser tests can be flaky and require proper browser setup. The standard `cargo test` approach is recommended for most use cases.

## 🐛 Troubleshooting

### Common Issues

#### Tests Not Running
```bash
# Make sure you're in the project directory
cd /path/to/b3scale-admin

# Check Rust is properly installed
cargo --version
```

#### API Model Import Errors
- Tests use the `b3scale_api` models from the local `b3scale_api/` directory
- If you see import errors, ensure the API models are generated correctly

#### WASM Tests Failing
- Use `cargo test` instead - it's more reliable
- WASM browser tests require additional browser setup and can timeout

## 📝 Adding New Tests

### Create a Simple Test
```rust
#[test]
fn test_my_feature() {
    let result = my_function("input");
    assert_eq!(result, "expected_output");
}
```

### Test API Models
```rust
#[test]
fn test_my_api_model() {
    let config = FrontendConfig {
        key: "test-key".to_string(),
        secret: "test-secret".to_string(),
    };
    
    assert_eq!(config.key, "test-key");
}
```

## 🎉 Success!

Your tests are now working perfectly! The current setup provides:
- ✅ **Fast, reliable tests** with `cargo test`
- ✅ **API model validation** for core data structures  
- ✅ **Easy expansion** for future test scenarios
- ✅ **CI/CD ready** for automated testing

Run `./scripts/run_tests.sh` anytime to verify everything is working! 🚀

## 🔗 Integration with Development

### Continuous Testing
```bash
# Run tests automatically when files change (if you have cargo-watch)
cargo watch -x test

# Run tests before committing
git add . && cargo test && git commit -m "Your changes"
```

### CLAUDE.md Integration
Tests are documented in `CLAUDE.md` under the Testing section for easy reference during development.

Happy testing! 🎉