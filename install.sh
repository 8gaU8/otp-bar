#! /bin/bash
set -eu -o pipefail
cd "$(dirname "$0")"


# Params
LATEST_VERSION='1.0.0'

VERSION="${APP_VERSION:-${LATEST_VERSION}}"
TAG="app-v${VERSION}"
ARCH='aarch64"'

TMP_DMG='/tmp/otp-bar.dmg'

# Install APP
echo 'Downloading App DMG...'
wget "https://github.com/8gaU8/otp-bar/releases/download/${TAG}/OTP.Bar_${VERSION}_${ARCH}.dmg" -O "${TMP_DMG}"
xattr -d com.apple.quarantine "${TMP_DMG}" || true
hdiutil attach "${TMP_DMG}"
cp -R "/Volumes/OTP Bar/OTP Bar.app" /Applications/
hdiutil detach "/Volumes/OTP Bar"
rm "${TMP_DMG}"

echo "Installed OTP Bar ${VERSION}"

