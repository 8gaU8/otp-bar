#! /bin/bash
set -eu -o pipefail
cd "$(dirname "$0")"


# Params
VERSION="0.4.0"
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

# Initialize config directory
echo 'Initializing config directory...'
CONFIG_DIR="${HOME}/.config/otp-bar";

if [ ! -d "${CONFIG_DIR}" ]; then
    echo 'Creating config directory'
    mkdir -p "${CONFIG_DIR}";
fi

echo "Installed OTP Bar ${VERSION}"

