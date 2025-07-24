# b3scale-admin 🚀

A modern, blazing-fast admin panel for managing b3scale - built with Rust and WebAssembly! ✨

## A note from Annie

Sooo. I started this project like 3 years or so ago and
it collected dust. I never had the time or energy to actually
work on this.

I decided to turn this into an experiment and just vibe the rest
of the fucking owl.

## 🎯 Overview

b3scale-admin is a web-based administration interface for [b3scale](https://github.com/b3scale/b3scale), a load balancing system for BigBlueButton servers. The entire frontend runs in your browser as WebAssembly, providing native-like performance with the security and portability of the web.

## ✨ Features

- **🦀 Pure Rust/WASM** - Entire frontend compiled to WebAssembly for maximum performance
- **🔐 JWT Authentication** - Secure token-based authentication system
- **🎨 Dark Theme** - Beautiful Bootstrap 5 dark theme for comfortable viewing
- **⚡ Real-time Updates** - Instant feedback on all operations
- **📱 Responsive Design** - Works seamlessly on desktop and mobile devices

## 🛠️ Tech Stack

- **Language**: Rust 🦀
- **Framework**: [Yew](https://yew.rs/) v0.19
- **Build Tool**: [Trunk](https://trunkrs.dev/)
- **Styling**: Bootstrap 5 + SCSS
- **Target**: WebAssembly

## 🚀 Quick Start

### Prerequisites

- Rust (latest stable)
- trunk (`cargo install trunk`)
- A running b3scale API server on `http://localhost:42353`

### Development

```bash
# Clone the repository
git clone https://github.com/b3scale/b3scale-admin.git
cd b3scale-admin

# Start the development server with hot reload
trunk serve

# Open your browser at http://localhost:8080
```

### Production Build

```bash
# Create optimized production build
trunk build --release

# Output will be in dist/ directory
```

## 📁 Project Structure

```
b3scale-admin/
├── src/
│   ├── api/           # API client and models
│   │   ├── client.rs  # HTTP client implementation
│   │   ├── auth/      # Authentication context
│   │   └── models/    # Auto-generated API models
│   ├── app/           # Application components
│   │   ├── router.rs  # Client-side routing
│   │   ├── frontends/ # Frontend management UI
│   │   └── backends/  # Backend server management
│   └── main.rs        # Application entry point
├── styles/            # SCSS stylesheets
├── Trunk.toml         # Build configuration
└── Cargo.toml         # Rust dependencies
```

## 🔧 Configuration

The development server is configured to proxy API requests to `http://localhost:42353/api/v1`. You can modify this in `Trunk.toml` if your b3scale API runs on a different port.

## 🧩 Core Concepts

### Frontends
API clients that can create and manage BigBlueButton meetings. Each frontend has:
- Unique API key for authentication
- Configurable settings
- Usage statistics

### Backends
BigBlueButton servers that host the actual meetings. Features include:
- Health monitoring
- Load balancing configuration
- Performance metrics

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 Development Tips

- API models in `src/api/models/` are auto-generated - don't edit them manually
- Run `cargo clippy` before committing to catch common issues
- Use `cargo fmt` to maintain consistent code style
- The SCSS is automatically compiled via pre-build hooks

## 🐛 Troubleshooting

### Common Issues

**Build fails with SCSS errors**
- Make sure you have `grass` installed: `cargo install grass`

**API connection refused**
- Ensure b3scale API is running on port 42353
- Check CORS settings in your b3scale configuration

**Authentication failures**
- Clear localStorage and try logging in again
- Verify your b3scale API is configured correctly

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Built with ❤️ using the amazing [Yew framework](https://yew.rs/)
- Powered by the incredible Rust and WebAssembly ecosystem
- Part of the [b3scale](https://github.com/b3scale/b3scale) project family

---

Made with 💜 and 🦀 by the b3scale team
