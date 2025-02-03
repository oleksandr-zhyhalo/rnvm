#!/bin/bash

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO="oleksandr-zhyhalo/rnvm"
RNVM_DIR="${HOME}/.rnvm"
GITHUB_LATEST="https://api.github.com/repos/${REPO}/releases/latest"

print_success() {
    echo -e "${GREEN}$1${NC}"
}

print_info() {
    echo -e "${YELLOW}$1${NC}"
}

# Detect OS and architecture
detect_platform() {
  local OS
  local ARCH
  OS=$(uname -s | tr '[:upper:]' '[:lower:]')
  ARCH=$(uname -m)

  case "$OS" in
    linux)
      case "$ARCH" in
        x86_64)
          # Force the musl build
          echo "linux-amd64-musl"
          ;;
        aarch64)
          # If you have a separate musl build for ARM64, you'd do:
          echo "linux-arm64-musl"
          # or if you keep it as "linux-arm64", do so here
          ;;
        *)
          echo "Unsupported architecture: $ARCH" && exit 1
          ;;
      esac
      ;;
    darwin)
      case "$ARCH" in
        x86_64) echo "macos-amd64" ;;
        arm64) echo "macos-arm64" ;;
        *)
          echo "Unsupported architecture: $ARCH" && exit 1
          ;;
      esac
      ;;
    *)
      echo "Unsupported OS: $OS" && exit 1
      ;;
  esac
}

# Configure shell
setup_shell() {
    local SHELL_CONFIG
    local FOUND=0

    # Detect shell configuration file
    for file in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.config/fish/config.fish" "$HOME/.profile"; do
        if [ -f "$file" ]; then
            SHELL_CONFIG="$file"
            FOUND=1
            break
        fi
    done

    if [ $FOUND -eq 0 ]; then
        SHELL_CONFIG="$HOME/.bashrc"
        touch "$SHELL_CONFIG"
    fi

    print_info "Configuring shell ($SHELL_CONFIG)..."

    # Remove any existing rnvm entries
    if [ -f "$SHELL_CONFIG" ]; then
        sed -i.bak '/export PATH="$PATH:$HOME\/.rnvm\/bin"/d' "$SHELL_CONFIG"
        sed -i.bak '/# rnvm configuration/d' "$SHELL_CONFIG"
        sed -i.bak '/alias node=/d' "$SHELL_CONFIG"
        sed -i.bak '/alias npm=/d' "$SHELL_CONFIG"
        sed -i.bak '/alias npx=/d' "$SHELL_CONFIG"
    fi

    # Add new configuration
    {
        echo ""
        echo "# rnvm configuration"
        echo 'export PATH="$PATH:$HOME/.rnvm/bin"'
        echo 'alias node="$HOME/.rnvm/current/bin/node"'
        echo 'alias npm="$HOME/.rnvm/current/bin/npm"'
        echo 'alias npx="$HOME/.rnvm/current/bin/npx"'
    } >> "$SHELL_CONFIG"

    print_success "Shell configured successfully!"
}

main() {
    print_info "Starting rnvm installation..."

    # Create installation directory
    mkdir -p "$RNVM_DIR/bin"
    mkdir -p "$RNVM_DIR/versions"

    # Get latest release information
    print_info "Fetching latest release..."
    local PLATFORM
    PLATFORM=$(detect_platform)
    local DOWNLOAD_URL
    DOWNLOAD_URL=$(
      curl -s "$GITHUB_LATEST" \
      | jq -r '.assets[] | select(.name == "rnvm-'${PLATFORM}'") | .browser_download_url'
    )
    # Download binary
    print_info "Downloading rnvm..."
    curl -L "$DOWNLOAD_URL" -o "$RNVM_DIR/bin/rnvm"
    chmod +x "$RNVM_DIR/bin/rnvm"

    # Setup shell configuration
    setup_shell

    print_success "\nrnvm has been installed successfully! 🎉"
    print_info "\nTo complete the installation, please:\n"
    print_info "1. Restart your terminal"
    print_info "   OR"
    print_info "2. Run: source $SHELL_CONFIG"
    print_info "\nThen you can start using rnvm:"
    print_info "  rnvm install lts"
    print_info "  rnvm use lts"
    print_info "\nEnjoy! 🚀"
}

main