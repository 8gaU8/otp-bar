#! /bin/bash
set -eu pipefail
cd "$(dirname "$0")"


# Params
VERSION="0.3.0"
TAG="app-v${VERSION}"
ARCH="aarch64"

TMP_DMG='/tmp/otp-bar.dmg'

# Install APP
echo 'Downloading App DMG...'
wget "https://github.com/8gaU8/otp-bar/releases/download/${TAG}/OTP.Bar_${VERSION}_${ARCH}.dmg" -O "${TMP_DMG}"
xattr -d com.apple.quarantine "${TMP_DMG}" || true
hdiutil attach "${TMP_DMG}"
cp -R "/Volumes/OTP Bar/OTP Bar.app" /Applications/
hdiutil detach "/Volumes/OTP Bar"
rm "${TMP_DMG}"

# Install Requirements
echo 'Installing Requirements...'
brew install coreutils oath-toolkit


# Initialize config
echo 'Initializing config...'
CONFIG_DIR="${HOME}/.config/otp-bar";
CONFIG_FILE="config.json";

if [ ! -d "${CONFIG_DIR}" ]; then
    echo 'Creating config files'
    mkdir -p "${CONFIG_DIR}";
    echo '{ "oathtoolExecutablePath": "/opt/homebrew/bin/oathtool" }'> "${CONFIG_DIR}/${CONFIG_FILE}";
fi

echo "Installed OTP Bar ${VERSION}"

