#!/bin/bash
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
OPENAPI_SPEC="$PROJECT_ROOT/openapi/b3scale_v1.json"
OUTPUT_DIR="$PROJECT_ROOT/b3scale_api"
TEMP_DIR="$PROJECT_ROOT/.openapi-generator-tmp"

# Check if OpenAPI spec exists
if [ ! -f "$OPENAPI_SPEC" ]; then
    echo -e "${RED}Error: OpenAPI spec not found at $OPENAPI_SPEC${NC}"
    exit 1
fi

echo -e "${GREEN}🔧 Generating b3scale API models...${NC}"

# Check if openapi-generator-cli is installed
if ! command -v openapi-generator-cli &> /dev/null; then
    # Check if we can use npx instead
    if command -v npx &> /dev/null; then
        echo -e "${YELLOW}Using npx to run openapi-generator-cli...${NC}"
        OPENAPI_GENERATOR="npx @openapitools/openapi-generator-cli"
    else
        echo -e "${RED}Error: openapi-generator-cli not found and npx not available${NC}"
        echo -e "${RED}Please install with: npm install -g @openapitools/openapi-generator-cli${NC}"
        echo -e "${RED}Or install Node.js/npm to use npx${NC}"
        exit 1
    fi
else
    OPENAPI_GENERATOR="openapi-generator-cli"
fi

# Clean up previous generation
if [ -d "$TEMP_DIR" ]; then
    rm -rf "$TEMP_DIR"
fi

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# Generate models only (no client)
echo -e "${GREEN}📦 Generating Rust models from OpenAPI spec...${NC}"
$OPENAPI_GENERATOR generate \
    -i "$OPENAPI_SPEC" \
    -g rust \
    -o "$TEMP_DIR" \
    --global-property models \
    --additional-properties packageName=b3scale_api,packageVersion=0.1.0,library=reqwest,supportAsync=false,useSingleRequestParameter=false

# Create the crate structure if it doesn't exist
if [ ! -f "$OUTPUT_DIR/Cargo.toml" ]; then
    echo -e "${GREEN}📝 Creating b3scale_api crate...${NC}"
    
    # Create Cargo.toml
    cat > "$OUTPUT_DIR/Cargo.toml" << 'EOF'
[package]
name = "b3scale_api"
version = "0.1.0"
edition = "2021"
authors = ["b3scale-admin"]
description = "Auto-generated data models for b3scale API"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_with = "3.0"
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
tokio = { version = "1", features = ["full"] }
EOF

    # Create .gitignore
    cat > "$OUTPUT_DIR/.gitignore" << 'EOF'
/target
Cargo.lock
EOF

    # Create src directory first
    mkdir -p "$OUTPUT_DIR/src"
    
    # Create lib.rs placeholder - will be replaced with generated content
    cat > "$OUTPUT_DIR/src/lib.rs" << 'EOF'
//! Auto-generated b3scale API data models
//! 
//! This crate contains all the data models for the b3scale API,
//! generated from the OpenAPI specification.

#![allow(unused_imports)]
#![allow(clippy::all)]

// This file will be replaced with the generated model exports
EOF
fi

# Create src directory if it doesn't exist
mkdir -p "$OUTPUT_DIR/src/models"

# Copy generated models
echo -e "${GREEN}📂 Copying generated models...${NC}"
if [ -d "$TEMP_DIR/src/models" ]; then
    cp -r "$TEMP_DIR/src/models/"* "$OUTPUT_DIR/src/models/"
    
    # Create mod.rs that exports all models
    echo "//! Auto-generated API models" > "$OUTPUT_DIR/src/models/mod.rs"
    echo "" >> "$OUTPUT_DIR/src/models/mod.rs"
    
    # Add all model files to mod.rs
    for model_file in "$OUTPUT_DIR/src/models"/*.rs; do
        if [ -f "$model_file" ] && [ "$(basename "$model_file")" != "mod.rs" ]; then
            model_name=$(basename "$model_file" .rs)
            echo "pub mod $model_name;" >> "$OUTPUT_DIR/src/models/mod.rs"
            echo "pub use $model_name::*;" >> "$OUTPUT_DIR/src/models/mod.rs"
        fi
    done
else
    echo -e "${RED}Error: Generated models not found in expected location${NC}"
    exit 1
fi

# Clean up temporary directory
rm -rf "$TEMP_DIR"

# Format the generated code
echo -e "${GREEN}🎨 Formatting generated code...${NC}"
cd "$OUTPUT_DIR" && cargo fmt

# Check if the crate builds
echo -e "${GREEN}🔨 Checking if crate builds...${NC}"
cd "$OUTPUT_DIR" && cargo check

echo -e "${GREEN}✅ Successfully generated b3scale_api crate!${NC}"
echo -e "${GREEN}📍 Location: $OUTPUT_DIR${NC}"
echo ""
echo -e "${YELLOW}To use this crate in your project, add to Cargo.toml:${NC}"
echo -e "  b3scale_api = { path = \"./b3scale_api\" }"