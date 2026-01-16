#!/bin/bash
set -eu -o pipefail

# Params
LATEST_VERSION='1.1.0'
VERSION="${CLI_VERSION:-${LATEST_VERSION}}"
TAG="app-v${VERSION}"
ARCH="${ARCH:-aarch64}"

# Default installation path
DEFAULT_INSTALL_PATH="$HOME/.local/bin"
INSTALL_PATH="${INSTALL_PATH:-${DEFAULT_INSTALL_PATH}}"

# Ensure installation directory exists
mkdir -p "${INSTALL_PATH}"

# Temporary file
TMP_CLI="/tmp/otp-cli-${ARCH}"

echo "Installing OTP CLI v${VERSION} to ${INSTALL_PATH}"

# Download the CLI binary from release
echo "Downloading OTP CLI binary..."
wget "https://github.com/8gaU8/otp-bar/releases/download/${TAG}/otp-cli-${ARCH}" -O "${TMP_CLI}" || {
    echo "ERROR: Failed to download CLI binary. The binary may not be available for this release."
    echo "You can build it locally by running: cd src-tauri && cargo build --release --bin otp-cli"
    exit 1
}

# Make it executable
chmod +x "${TMP_CLI}"

# Move to installation path
mv "${TMP_CLI}" "${INSTALL_PATH}/otp-cli"

echo "✓ OTP CLI installed successfully to ${INSTALL_PATH}/otp-cli"
echo ""
echo "Usage:"
echo "  otp-cli show [token_id]    # Display OTP with remaining time"
echo "  otp-cli clip [token_id]    # Copy OTP to clipboard"
echo ""
echo "If token_id is not specified, the highest priority token will be used."
echo ""
echo "Note: Make sure ${INSTALL_PATH} is in your PATH."
echo "Add this to your shell profile if needed:"
echo "  export PATH=\"\$PATH:${INSTALL_PATH}\""
