#!/usr/bin/env bash
set -euo pipefail

PACKAGE_DIR="${1:?package directory is required}"
OUT_ROOT="${2:-dist/packager}"
MANIFEST="${PACKAGE_DIR}/nexus-package-manifest.json"

if [ "$(uname -s)" != "Darwin" ]; then
  echo "iOS simulator packaging requires Darwin/macOS." >&2
  exit 1
fi

command -v xcrun >/dev/null 2>&1 || {
  echo "xcrun is required for iOS simulator packaging." >&2
  exit 1
}

META="$(python3 - "${MANIFEST}" <<'PY'
import json
import re
import sys
from pathlib import Path
payload = json.loads(Path(sys.argv[1]).read_text())
app_name = payload.get("appName") or "Nexus Packaged App"
slug = payload.get("slug") or "nexus-packaged-app"
app_id = payload.get("appId") or "dev.luminarylabs.nexuspackaged.app"
bundle_name = re.sub(r"[^A-Za-z0-9]", "", app_name) or slug
print(app_name)
print(slug)
print(app_id)
print(bundle_name)
print(payload["webDir"])
PY
)"

APP_NAME="$(printf '%s\n' "${META}" | sed -n '1p')"
SLUG="$(printf '%s\n' "${META}" | sed -n '2p')"
APP_ID="$(printf '%s\n' "${META}" | sed -n '3p')"
BUNDLE_NAME="$(printf '%s\n' "${META}" | sed -n '4p')"
WEB_DIR="$(printf '%s\n' "${META}" | sed -n '5p')"

APP_DIR="${OUT_ROOT}/artifacts/ios-sim/${BUNDLE_NAME}.app"
ZIP_PATH="${OUT_ROOT}/artifacts/ios-sim/${SLUG}-ios-simulator-app.zip"
SDK_PATH="$(xcrun --sdk iphonesimulator --show-sdk-path)"
ARCH="$(uname -m)"
TARGET="${ARCH}-apple-ios17.0-simulator"

rm -rf "${APP_DIR}" "${ZIP_PATH}"
mkdir -p "${APP_DIR}/app"
cp -R "${WEB_DIR}/." "${APP_DIR}/app/"

xcrun swiftc \
  -sdk "${SDK_PATH}" \
  -target "${TARGET}" \
  -parse-as-library \
  -O \
  app/ios-webview/NexusPackagedWebApp/AppDelegate.swift \
  -o "${APP_DIR}/NexusPackagedWebApp"

python3 - "${APP_DIR}/Info.plist" "${APP_NAME}" "${APP_ID}" <<'PY'
import plistlib
import sys
from pathlib import Path
path = Path(sys.argv[1])
payload = {
    "CFBundleExecutable": "NexusPackagedWebApp",
    "CFBundleIdentifier": sys.argv[3],
    "CFBundleName": sys.argv[2],
    "CFBundleDisplayName": sys.argv[2],
    "CFBundlePackageType": "APPL",
    "CFBundleShortVersionString": "0.1.0",
    "CFBundleVersion": "1",
    "MinimumOSVersion": "17.0",
    "UILaunchStoryboardName": "",
    "UISupportedInterfaceOrientations": [
        "UIInterfaceOrientationLandscapeLeft",
        "UIInterfaceOrientationLandscapeRight",
        "UIInterfaceOrientationPortrait"
    ],
}
path.write_bytes(plistlib.dumps(payload))
PY

codesign --force --deep --sign - "${APP_DIR}" >/dev/null 2>&1 || true
ditto -c -k --keepParent "${APP_DIR}" "${ZIP_PATH}"
python3 scripts/record-package-output.py \
  --manifest "${MANIFEST}" \
  --target ios-sim \
  --artifact "${ZIP_PATH}" \
  --out-root "${OUT_ROOT}"

echo "Built ${ZIP_PATH}"
