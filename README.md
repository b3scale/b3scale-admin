# 🔥 b3scale-admin 🔥

*The most elite WebAssembly admin panel for b3scale - built with pure Rust fire* 🦀⚡

<div align="center">

[![Rust](https://img.shields.io/badge/rust-1.0%2B-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=FFF)](https://webassembly.org/)
[![Yew](https://img.shields.io/badge/Yew-v0.19-green?style=for-the-badge)](https://yew.rs/)
[![Nix](https://img.shields.io/badge/Nix-5277C3?style=for-the-badge&logo=nixos&logoColor=white)](https://nixos.org/)

**1337 Development Setup Available** | **Zero-JS Frontend** | **Maximum Performance**

</div>

---

## 💜 A Note From Annie 

*Sooo.* I started this project like 3 years ago and it collected dust. Never had the time or energy to actually work on this.

**BUT** I decided to turn this into an experiment and just **vibe the rest of the fucking owl.** 🦉

Let's make something absolutely legendary! ✨

---

## 🎯 What Is This?

b3scale-admin is a **web-based administration interface** for [b3scale](https://github.com/b3scale/b3scale) - a load balancing system for BigBlueButton servers. 

**The entire frontend runs in your browser as WebAssembly** - giving you native-like performance with the security and portability of the web. No JavaScript. Just pure Rust compiled to WASM. 🚀

![b3scale-admin Screenshot](propaganda/Screenshot%20from%202025-07-25%2008-56-43.png)

*The vibec0re dark theme admin interface in all its glory* ✨

## ✨ Features That Hit Different

- **🦀 Pure Rust/WASM** - Zero JavaScript, maximum performance
- **🔐 JWT Authentication** - Secure token-based auth system
- **🎨 Vibec0re Dark Theme** - Beautiful Bootstrap 5 dark theme for late-night coding
- **⚡ Real-time Updates** - Instant feedback on all operations
- **📱 Responsive AF** - Works flawlessly on desktop and mobile
- **🔥 1337 Dev Setup** - Nix flakes for the most elite development experience

## 🛠️ Tech Stack (The Good Stuff)

- **Language**: Rust 🦀 (because we're not animals)
- **Framework**: [Yew](https://yew.rs/) v0.19 (React but better)
- **Build Tool**: [Trunk](https://trunkrs.dev/) (blazing fast)
- **Styling**: Bootstrap 5 + SCSS (dark mode everything)
- **Target**: WebAssembly (the future is now)
- **Dev Environment**: Nix flakes (reproducible elite setup)

---

# 🚀 Quick Start Options

## Option 1: 1337 Nix Setup (RECOMMENDED)

*For developers who want the most elite experience*

```bash
# Modern flakes way (recommended)
nix develop

# Legacy shell.nix way (compatibility)  
nix-shell

# Direnv way (automatic, most 1337)
direnv allow
```

**What you get instantly:**
- Rust with `wasm32-unknown-unknown` target
- trunk for blazing WebAssembly builds
- wasm-pack, cargo-watch, rust-analyzer
- Node.js 20 + all system dependencies
- Zero configuration, maximum vibes ✨

## Option 2: Manual Setup (Still Good)

```bash
# Install prerequisites
cargo install trunk
rustup target add wasm32-unknown-unknown

# Clone and run
git clone https://github.com/b3scale/b3scale-admin.git
cd b3scale-admin
trunk serve
```

---

# 🔥 NIX DEVELOPMENT SETUP (ELITE MODE)

*Complete development environment with zero hassle*

## ⚡ Instant Setup

```bash
# Clone the repo
git clone https://github.com/b3scale/b3scale-admin.git
cd b3scale-admin

# Enter the matrix (choose your fighter)
nix develop     # Modern flakes
nix-shell       # Legacy compatibility  
direnv allow    # Automatic (requires direnv)
```

## 🏗️ Architecture

### Modern Flakes (`flake.nix`)
- Uses `rust-overlay` for bleeding-edge toolchain
- Declarative, reproducible, cacheable  
- Supports dev shell + package building
- Auto-includes WebAssembly targets

### Legacy Shell (`shell.nix`)
- Compatible with older Nix installations
- Same rust-overlay for consistency
- Fallback for systems without flakes

### Direnv Integration (`.envrc`) 
- Automatic environment activation
- No manual commands needed
- Perfect VS Code/editor integration

## 🛠️ Development Commands

```bash
# Start development server
trunk serve

# Build for production  
trunk build --release

# Run tests
./scripts/run_tests.sh

# Lint & format
cargo clippy
cargo fmt

# Watch for changes
cargo watch -x check
```

## 🔧 Nix Setup Instructions

### 1. Enable Flakes

Add to `/etc/nix/nix.conf` or `~/.config/nix/nix.conf`:
```
experimental-features = nix-command flakes
```

### 2. Install Direnv (Recommended)

```bash
# NixOS
nix-env -iA nixos.direnv

# macOS/Linux  
nix-env -iA nixpkgs.direnv
```

Add to shell config (`.bashrc`, `.zshrc`, etc.):
```bash
eval "$(direnv hook bash)"  # or zsh, fish, etc.
```

### 3. Activate Environment

```bash
# Method 1: Direnv (automatic)
direnv allow

# Method 2: Flakes (manual)
nix develop  

# Method 3: Legacy (fallback)
nix-shell
```

## 🎯 IDE Integration

### VS Code
1. Install `rust-analyzer` extension
2. Use direnv or configure `rust-analyzer.server.path`  
3. Enjoy blazing fast development

### Vim/Neovim
Works out of the box with CoC, nvim-lsp, or any LSP client using direnv.

## 🔥 Pro Tips

### Elite Development Workflow
```bash
# Terminal 1: Dev server
trunk serve

# Terminal 2: Tests in watch mode
cargo watch -x test  

# Terminal 3: Code quality
cargo watch -x clippy
```

### Binary Cache (Faster Builds)
Add to `~/.config/nix/nix.conf`:
```
substituters = https://cache.nixos.org/ https://nix-community.cachix.org
trusted-public-keys = cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs=
```

---

# 📁 Project Structure

```
b3scale-admin/
├── 🔥 Nix Setup
│   ├── flake.nix          # Modern Nix flakes
│   ├── shell.nix          # Legacy Nix shell  
│   └── .envrc             # Direnv integration
├── src/
│   ├── api/               # API client and models
│   │   ├── client.rs      # HTTP client implementation
│   │   ├── auth/          # Authentication context
│   │   └── models/        # Auto-generated API models
│   ├── app/               # Application components
│   │   ├── router.rs      # Client-side routing
│   │   ├── frontends/     # Frontend management UI
│   │   └── backends/      # Backend server management  
│   └── main.rs            # Application entry point
├── styles/                # SCSS stylesheets
├── Trunk.toml             # Build configuration
└── Cargo.toml             # Rust dependencies
```

# 🔧 Configuration

The development server proxies API requests to `http://localhost:42353/api/v1`. Modify this in `Trunk.toml` if your b3scale API runs elsewhere.

---

# 🧩 Core Concepts

## Frontends
**API clients** that create and manage BigBlueButton meetings:
- 🔑 Unique API keys for authentication
- ⚙️ Configurable settings  
- 📊 Usage statistics and monitoring

## Backends  
**BigBlueButton servers** that host the actual meetings:
- 💚 Health monitoring and status
- ⚖️ Load balancing configuration
- 📈 Performance metrics and analytics

---

# 🐛 Troubleshooting

## Nix Environment Issues

### `command not found: trunk`
```bash
# Check if in nix environment
echo $IN_NIX_SHELL

# If not, enter environment  
nix develop
# or
direnv allow
```

### `No such file or directory: /nix/store/...`
```bash
# Rebuild environment
nix develop --rebuild

# Or for shell.nix
nix-shell --pure
```

### WebAssembly target missing
```bash
# This is automatic in Nix, but if needed:
rustup target add wasm32-unknown-unknown
```

## General Issues

### Build fails with SCSS errors
```bash
# Install grass (handled by Nix automatically)
cargo install grass
```

### API connection refused
- Ensure b3scale API runs on port `42353`
- Check CORS settings in b3scale config
- Verify firewall/network settings

### Authentication failures  
- Clear browser localStorage
- Verify b3scale API configuration
- Check JWT token expiration

---

# 📝 Development Notes

- 🚫 **Don't edit** `src/api/models/` - auto-generated from OpenAPI
- 🔍 Run `cargo clippy` before commits for code quality
- 🎨 Use `cargo fmt` for consistent formatting  
- 📦 SCSS compiles automatically via pre-build hooks
- 🔥 Use the Nix environment for best experience

---

# 🤝 Contributing (Join The Vibe)

Ready to make this project even more legendary? Let's go! 🚀

```bash
# 1. Fork the repository (hit that fork button)
# 2. Create your feature branch
git checkout -b feature/EpicFeature

# 3. Make your changes (using the Nix environment!)  
nix develop
# ... code like a legend ...

# 4. Commit your changes
git commit -m 'Add some EpicFeature'

# 5. Push to your branch
git push origin feature/EpicFeature

# 6. Open a Pull Request and describe your epic contribution
```

**Contribution Guidelines:**
- 🦀 Keep it Rust-y and idiomatic
- 🧪 Add tests for new features
- 📚 Update docs for significant changes
- 🎨 Follow the existing code style
- 🔥 Use the Nix environment for development

---

# 📄 License

MIT License - because sharing is caring! See `LICENSE` file for details.

---

# 🙏 Acknowledgments & Shoutouts

- 🦀 **Rust Team** - For creating the most amazing language
- ⚛️ **Yew Framework** - React but better, in Rust
- 🌐 **WebAssembly** - The future of web development
- ❄️ **Nix Community** - For reproducible development environments  
- 💜 **b3scale Team** - For the epic BigBlueButton load balancing
- 🦉 **Everyone who vibes with the owl** - You know who you are

---

<div align="center">

**Made with 💜, 🦀, and pure elite vibes**

*Now go build something absolutely legendary!* ✨🚀

[![Built with Nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

</div>
