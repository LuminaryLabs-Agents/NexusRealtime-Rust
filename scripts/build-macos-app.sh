#!/usr/bin/env bash
set -euo pipefail

APP_NAME="NexusEngineRustDemo"
APP_DIR="dist/macos/${APP_NAME}.app"
CONTENTS="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS}/MacOS"
FRAMEWORKS_DIR="${CONTENTS}/Frameworks"
RESOURCES_DIR="${CONTENTS}/Resources"
LIB_NAME="libnexus_host_ffi.dylib"

if [ "$(uname -s)" != "Darwin" ]; then
  echo "macOS app packaging requires Darwin/macOS." >&2
  exit 1
fi

rm -rf "${APP_DIR}"
mkdir -p "${MACOS_DIR}" "${FRAMEWORKS_DIR}" "${RESOURCES_DIR}"

cargo build --release -p nexus-host-ffi
cp "target/release/${LIB_NAME}" "${FRAMEWORKS_DIR}/${LIB_NAME}"

swiftc \
  -O \
  app/macos/NexusEngineRustDemo/Sources/main.swift \
  -L "${FRAMEWORKS_DIR}" \
  -l nexus_host_ffi \
  -Xlinker -rpath \
  -Xlinker "@executable_path/../Frameworks" \
  -o "${MACOS_DIR}/${APP_NAME}"

cat > "${CONTENTS}/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>NexusEngineRustDemo</string>
  <key>CFBundleIdentifier</key>
  <string>dev.luminarylabs.nexusengine.rust.demo</string>
  <key>CFBundleName</key>
  <string>NexusEngine Rust Demo</string>
  <key>CFBundleDisplayName</key>
  <string>NexusEngine Rust Demo</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>0.1.0</string>
  <key>CFBundleVersion</key>
  <string>1</string>
  <key>LSMinimumSystemVersion</key>
  <string>13.0</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
PLIST

plutil -lint "${CONTENTS}/Info.plist"
echo "Built ${APP_DIR}"
