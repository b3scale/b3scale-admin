{ pkgs ? import <nixpkgs> {
    overlays = [ (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz")) ];
  }
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain with WebAssembly target
    (rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" "rust-analyzer" ];
      targets = [ "wasm32-unknown-unknown" ];
    })
    
    # Build tools
    trunk
    wasm-pack
    wasm-bindgen-cli
    
    # Development tools
    cargo-watch
    cargo-edit
    cargo-audit
    
    # Web development
    nodejs_20
    
    # System dependencies
    pkg-config
    openssl
    
    # Optional but useful
    ripgrep
    fd
    git
  ];

  shellHook = ''
    echo "🚀 b3scale-admin dev environment (shell.nix) loaded!"
    echo "Available commands:"
    echo "  trunk serve     - Start development server"
    echo "  trunk build     - Build for production"
    echo "  ./scripts/run_tests.sh - Run tests"
    echo "  cargo clippy    - Run linter"
    echo "  cargo fmt       - Format code"
    echo ""
    echo "Rust version: $(rustc --version)"
    if command -v trunk &> /dev/null; then
      echo "Trunk version: $(trunk --version)"
    else
      echo "Warning: trunk not found in PATH"
    fi
  '';

  # Environment variables
  RUST_BACKTRACE = "1";
}
