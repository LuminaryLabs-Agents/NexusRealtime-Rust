#!/usr/bin/env bash
set -euo pipefail

PACKAGE_DIR="${1:?package directory is required}"
OUT_ROOT="${2:-dist/packager}"
MANIFEST="${PACKAGE_DIR}/nexus-package-manifest.json"

if [ "$(uname -s)" != "Darwin" ]; then
  echo "macOS app packaging requires Darwin/macOS." >&2
  exit 1
fi

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

APP_DIR="${OUT_ROOT}/artifacts/macos-app/${BUNDLE_NAME}.app"
CONTENTS="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS}/MacOS"
RESOURCES_DIR="${CONTENTS}/Resources"
ZIP_PATH="${OUT_ROOT}/artifacts/macos-app/${SLUG}-macos-app.zip"

rm -rf "${APP_DIR}" "${ZIP_PATH}"
mkdir -p "${MACOS_DIR}" "${RESOURCES_DIR}/app"

cargo build --release -p nexus-static-server
cp "target/release/nexus-static-server" "${MACOS_DIR}/nexus-static-server"
cp -R "${WEB_DIR}/." "${RESOURCES_DIR}/app/"

swiftc \
  -O \
  -framework AppKit \
  -framework WebKit \
  app/macos/NexusPackagedWebApp/Sources/main.swift \
  -o "${MACOS_DIR}/NexusPackagedWebApp"

python3 - "${CONTENTS}/Info.plist" "${APP_NAME}" "${APP_ID}" <<'PY'
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
    "LSMinimumSystemVersion": "13.0",
    "NSHighResolutionCapable": True,
}
path.write_bytes(plistlib.dumps(payload))
PY

plutil -lint "${CONTENTS}/Info.plist"
codesign --force --deep --sign - "${APP_DIR}" >/dev/null 2>&1 || true
ditto -c -k --keepParent "${APP_DIR}" "${ZIP_PATH}"
python3 scripts/record-package-output.py \
  --manifest "${MANIFEST}" \
  --target macos-app \
  --artifact "${ZIP_PATH}" \
  --out-root "${OUT_ROOT}"

echo "Built ${ZIP_PATH}"
