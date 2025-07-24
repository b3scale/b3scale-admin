# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is b3scale-admin - a Rust/WebAssembly admin panel for managing b3scale, a load balancing system for BigBlueButton servers. The frontend compiles to WebAssembly and runs entirely in the browser.

## Essential Commands

### Development
```bash
# Start development server with hot reload
trunk serve

# Build for production
trunk build

# Build optimized release version
trunk build --release

# Clean build artifacts
trunk clean
```

### Testing
```bash
# Run UI tests in browser
./scripts/run_tests.sh

# Or manually with wasm-pack
wasm-pack test --headless --chrome

# Run tests with cargo
cargo test --target wasm32-unknown-unknown
```

### Rust/Cargo Commands
```bash
# Check code for errors without building
cargo check

# Run Rust linter
cargo clippy

# Format code
cargo fmt

# Update dependencies
cargo update
```

## Architecture Overview

### Technology Stack
- **Rust** with **Yew** framework (v0.19) for WebAssembly web application
- **Trunk** as the build tool and development server
- **Bootstrap 5** with dark theme for styling
- **SCSS** compiled with `grass` during pre-build step

### Key Components

1. **API Client** (`src/api/`)
   - HTTP client at `src/api/client.rs` handles all API requests
   - Models in `src/api/models/` are auto-generated from OpenAPI spec
   - Authentication context in `src/api/auth/` manages JWT tokens

2. **Application Routing** (`src/app/router.rs`)
   - Uses Yew Router for client-side routing
   - Main routes: `/`, `/frontends`, `/backends`
   - All routes require authentication

3. **Main Features**
   - **Frontends**: API clients that can create meetings (`src/app/frontends/`)
   - **Backends**: BigBlueButton servers that host meetings (`src/app/backends/`)

### Authentication Flow
1. JWT-based authentication required for all routes
2. Tokens stored in browser's localStorage
3. Authentication state managed via Yew context API
4. Automatic redirect to login on 401 responses

### Development Notes

- API calls are proxied to `http://localhost:42353/api/v1` during development (configured in Trunk.toml)
- SCSS styles must be compiled before building - this happens automatically via pre-build hook
- The application expects a b3scale API server running locally on port 42353
- Build output goes to `dist/` directory

### Working with API Models

The models in `src/api/models/` are auto-generated from the b3scale OpenAPI specification. Do not manually edit these files - they will be overwritten when regenerated.

### State Management

The application uses Yew's built-in state management:
- Component state for local UI state
- Context API for global state (authentication)
- Props for passing data between components