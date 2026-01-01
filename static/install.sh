#!/bin/sh
# Assay installer script
# Usage: curl -sSL https://assay.dev/install.sh | sh
#
# Environment variables:
#   ASSAY_VERSION - specific version to install (default: latest)
#   ASSAY_INSTALL_DIR - installation directory (default: ~/.assay/bin)

set -e

# Configuration
REPO="assay-dev/assay"
INSTALL_DIR="${ASSAY_INSTALL_DIR:-$HOME/.assay/bin}"
VERSION="${ASSAY_VERSION:-latest}"

# Colors (if terminal supports it)
if [ -t 1 ]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[0;33m'
    BLUE='\033[0;34m'
    NC='\033[0m' # No Color
else
    RED=''
    GREEN=''
    YELLOW=''
    BLUE=''
    NC=''
fi

info() {
    printf "${BLUE}==>${NC} %s\n" "$1"
}

success() {
    printf "${GREEN}✓${NC} %s\n" "$1"
}

warn() {
    printf "${YELLOW}⚠${NC} %s\n" "$1"
}

error() {
    printf "${RED}✗${NC} %s\n" "$1" >&2
    exit 1
}

# Detect OS and architecture
detect_platform() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)

    case "$ARCH" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            error "Unsupported architecture: $ARCH"
            ;;
    esac

    case "$OS" in
        linux)
            TARGET="${ARCH}-unknown-linux-gnu"
            ;;
        darwin)
            TARGET="${ARCH}-apple-darwin"
            ;;
        mingw*|msys*|cygwin*|windows*)
            error "Windows detected. Please use the PowerShell installer or download manually."
            ;;
        *)
            error "Unsupported OS: $OS"
            ;;
    esac

    echo "$TARGET"
}

# Get latest version from GitHub
get_latest_version() {
    if command -v curl >/dev/null 2>&1; then
        curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | cut -d'"' -f4
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | cut -d'"' -f4
    else
        error "Neither curl nor wget found. Please install one of them."
    fi
}

# Download file
download() {
    URL="$1"
    OUTPUT="$2"

    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$URL" -o "$OUTPUT"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "$URL" -O "$OUTPUT"
    else
        error "Neither curl nor wget found. Please install one of them."
    fi
}

# Main installation
main() {
    echo ""
    echo "  ${BLUE}Assay Installer${NC}"
    echo "  Deterministic policy enforcement for AI agents"
    echo ""

    # Detect platform
    info "Detecting platform..."
    TARGET=$(detect_platform)
    success "Platform: $TARGET"

    # Get version
    if [ "$VERSION" = "latest" ]; then
        info "Fetching latest version..."
        VERSION=$(get_latest_version)
        if [ -z "$VERSION" ]; then
            error "Could not determine latest version"
        fi
    fi
    success "Version: $VERSION"

    # Prepare installation directory
    info "Creating installation directory..."
    mkdir -p "$INSTALL_DIR"
    success "Install directory: $INSTALL_DIR"

    # Download
    ARCHIVE_NAME="assay-${VERSION}-${TARGET}.tar.gz"
    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARCHIVE_NAME}"
    
    info "Downloading $ARCHIVE_NAME..."
    TEMP_DIR=$(mktemp -d)
    trap 'rm -rf "$TEMP_DIR"' EXIT

    if ! download "$DOWNLOAD_URL" "$TEMP_DIR/$ARCHIVE_NAME"; then
        error "Download failed. Check if version $VERSION exists for platform $TARGET"
    fi
    success "Downloaded"

    # Extract
    info "Extracting..."
    tar -xzf "$TEMP_DIR/$ARCHIVE_NAME" -C "$TEMP_DIR"
    
    # Find and install binary
    BINARY=$(find "$TEMP_DIR" -name "assay" -type f | head -1)
    if [ -z "$BINARY" ]; then
        error "Binary not found in archive"
    fi
    
    cp "$BINARY" "$INSTALL_DIR/assay"
    chmod +x "$INSTALL_DIR/assay"
    success "Installed to $INSTALL_DIR/assay"

    # Verify installation
    info "Verifying installation..."
    if "$INSTALL_DIR/assay" --version >/dev/null 2>&1; then
        INSTALLED_VERSION=$("$INSTALL_DIR/assay" --version 2>&1 | head -1)
        success "Verified: $INSTALLED_VERSION"
    else
        warn "Could not verify installation"
    fi

    # Check PATH
    echo ""
    case ":$PATH:" in
        *":$INSTALL_DIR:"*)
            success "Assay is ready to use!"
            ;;
        *)
            warn "Add Assay to your PATH:"
            echo ""
            echo "  # Add to ~/.bashrc or ~/.zshrc:"
            echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
            echo ""
            echo "  # Then reload:"
            echo "  source ~/.bashrc  # or ~/.zshrc"
            ;;
    esac

    # Quick start
    echo ""
    echo "  ${GREEN}Quick Start${NC}"
    echo ""
    echo "  1. Create a policy:"
    echo "     ${BLUE}cat > policy.yaml << 'EOF'"
    echo "     version: \"1.1\""
    echo "     name: \"my-agent\""
    echo "     tools:"
    echo "       allow: [Search, CreateTicket]"
    echo "       deny: [DeleteAccount]"
    echo "     sequences:"
    echo "       - type: max_calls"
    echo "         tool: Search"
    echo "         max: 5"
    echo "     EOF${NC}"
    echo ""
    echo "  2. Create test traces:"
    echo "     ${BLUE}echo '{\"tools\": [\"Search\", \"CreateTicket\"]}' > traces.jsonl${NC}"
    echo ""
    echo "  3. Run coverage check:"
    echo "     ${BLUE}assay coverage --policy policy.yaml --traces traces.jsonl${NC}"
    echo ""
    echo "  Documentation: https://assay.dev/docs"
    echo ""
}

main "$@"
