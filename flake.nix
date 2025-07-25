{
  description = "b3scale-admin - Rust/WebAssembly admin panel";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain with WebAssembly target
            rustToolchain
            
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
            echo "🚀 b3scale-admin dev environment loaded!"
            echo "Available commands:"
            echo "  trunk serve     - Start development server"
            echo "  trunk build     - Build for production"
            echo "  ./scripts/run_tests.sh - Run tests"
            echo "  cargo clippy    - Run linter"
            echo "  cargo fmt       - Format code"
            echo ""
            echo "Rust version: $(rustc --version)"
            echo "Trunk version: $(trunk --version)"
          '';

          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = "1";
        };

        # Optional: Define packages that can be built
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "b3scale-admin";
          version = "0.1.0";
          
          src = ./.;
          
          nativeBuildInputs = with pkgs; [
            rustToolchain
            trunk
            wasm-pack
          ];
          
          buildPhase = ''
            trunk build --release
          '';
          
          installPhase = ''
            mkdir -p $out
            cp -r dist/* $out/
          '';
        };
      });
}