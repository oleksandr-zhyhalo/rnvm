# rnvm 🚀

<div align="center">

![GitHub release (latest by date)](https://img.shields.io/github/v/release/oleksandr-zhyhalo/rnvm)
![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

A blazingly fast Node.js version manager written in Rust. Simple, reliable, and cross-platform.

[Installation](#installation) •
[Features](#features) •
[Usage](#usage) •
[Contributing](#contributing)

</div>

## ✨ Features

- 🚀 **Blazingly Fast**: Written in Rust for maximum performance
- 🔄 **Smart Version Management**: Easy switching between Node.js versions
- 📦 **Project-Specific Versions**: Automatic version switching with `.nvmrc`
- 🏷️ **Aliases**: Create shortcuts for your most-used versions
- 🔍 **Smart Resolution**: Supports semantic versioning and LTS releases
- 💻 **Cross-Platform**: Works on Linux, macOS, and Windows
- 🛠️ **Zero Runtime Dependencies**: Single binary, no external requirements

## 🚀 Installation

### Using Install Script (Recommended)

```bash
curl -o- https://raw.githubusercontent.com/oleksandr-zhyhalo/rnvm/main/install.sh | bash
# or with wget
wget -qO- https://raw.githubusercontent.com/oleksandr-zhyhalo/rnvm/main/install.sh | bash
```

After installation, either:
- Restart your terminal, or
- Run: `source ~/.bashrc` (or `~/.zshrc` for Zsh users)

### Manual Installation

1. Download the binary for your platform from [releases page](https://github.com/oleksandr-zhyhalo/rnvm/releases)
2. Move it to `~/.rnvm/bin/rnvm`
3. Make it executable: `chmod +x ~/.rnvm/bin/rnvm`
4. Add to your shell configuration file (~/.bashrc, ~/.zshrc, etc.):
```bash
export PATH="$PATH:$HOME/.rnvm/bin"
```

## 📚 Usage

### Basic Commands

```bash
# Install Node.js versions
rnvm install 20.9.0    # Install specific version
rnvm install lts       # Install latest LTS version
rnvm install 20        # Install latest from major version

# Switch versions
rnvm use 20.9.0       # Use specific version
rnvm use lts          # Use LTS version

# List versions
rnvm list             # Show installed versions
rnvm list --remote    # Show available versions
rnvm list --remote --lts  # Show LTS versions
```

### Version Management

```bash
# Set default version
rnvm use 20.9.0 --default

# Project-specific version
rnvm local 20.9.0     # Creates .nvmrc in current directory

# Show versions
rnvm current          # Show current version
rnvm which            # Show version used in current directory
```

### Aliases

```bash
# Create aliases for easier version management
rnvm alias stable 20.9.0
rnvm alias latest 21.0.0
rnvm use stable

# Remove aliases
rnvm unalias stable
```

### Clean Up

```bash
# Remove versions you no longer need
rnvm uninstall 20.9.0
```

## 📂 Directory Structure

```
~/.rnvm/
├── versions/          # Installed Node.js versions
├── current           # Symlink to current version
└── config/
    └── aliases.json  # Stored aliases
```

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit changes: `git commit -am 'Add feature'`
4. Push to branch: `git push origin feature-name`
5. Submit a Pull Request

## 🔍 Troubleshooting

### Common Issues

1. **Permission Denied**
   ```bash
   # Fix permissions issues
   sudo chown -R $(whoami) ~/.rnvm
   ```

2. **Version Not Found**
   ```bash
   # Update remote version list
   rnvm list --remote
   ```

3. **Cannot Switch Versions**
   ```bash
   # Check current version and permissions
   rnvm current
   ls -la ~/.rnvm/current
   ```


## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [nvm](https://github.com/nvm-sh/nvm)
- Built with [Rust](https://www.rust-lang.org/)

---

<div align="center">
Made with ❤️

[Report Bug](https://github.com/oleksandr-zhyhalo/rnvm/issues) • [Request Feature](https://github.com/oleksandr-zhyhalo/rnvm/issues)
</div>