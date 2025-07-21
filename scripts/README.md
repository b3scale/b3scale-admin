# Scripts

## generate_models.sh

Generates Rust data models from the b3scale OpenAPI specification.

### Usage

```bash
./scripts/generate_models.sh
```

### What it does

1. Reads the OpenAPI spec from `./openapi/b3scale_v1.json`
2. Uses OpenAPI Generator to create Rust models (structs with serde derives)
3. Creates a standalone crate at `./b3scale_api/`
4. Formats the generated code with `cargo fmt`
5. Verifies the crate builds successfully

### Requirements

- Node.js and npm (for `npx` to run openapi-generator-cli)
- Rust toolchain (for `cargo fmt` and `cargo check`)

### Generated Crate

The generated `b3scale_api` crate contains:
- All API models as Rust structs
- Serde serialization/deserialization support
- Chrono for date/time types
- Re-exports all models at the crate root

To use in your project:
```toml
[dependencies]
b3scale_api = { path = "./b3scale_api" }
```

## compile_style.sh

Compiles SCSS to CSS using grass (existing script).