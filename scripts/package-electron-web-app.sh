#!/usr/bin/env bash
set -euo pipefail

PACKAGE_DIR="${1:?package directory is required}"
OUT_ROOT="${2:-dist/packager}"
PACKAGE_TARGET="${PACKAGE_TARGET:-electron}"
ELECTRON_PLATFORM="${ELECTRON_PLATFORM:-}"
PACKAGE_DIR="$(cd "${PACKAGE_DIR}" && pwd)"
mkdir -p "${OUT_ROOT}"
OUT_ROOT="$(cd "${OUT_ROOT}" && pwd)"
MANIFEST="${PACKAGE_DIR}/nexus-package-manifest.json"

META="$(python3 - "${MANIFEST}" <<'PY'
import json
import re
import sys
from pathlib import Path
payload = json.loads(Path(sys.argv[1]).read_text())
app_name = payload.get("appName") or "Nexus Packaged App"
slug = payload.get("slug") or "nexus-packaged-app"
bundle_name = re.sub(r"[^A-Za-z0-9]", "", app_name) or "NexusPackagedApp"
print(app_name)
print(slug)
print(bundle_name)
print(payload["webDir"])
PY
)"

APP_NAME="$(printf '%s\n' "${META}" | sed -n '1p')"
SLUG="$(printf '%s\n' "${META}" | sed -n '2p')"
BUNDLE_NAME="$(printf '%s\n' "${META}" | sed -n '3p')"
WEB_DIR="$(printf '%s\n' "${META}" | sed -n '4p')"

WORK_DIR="${OUT_ROOT}/work/${SLUG}/electron-host"
BUNDLE_OUT="${OUT_ROOT}/work/${SLUG}/electron-build"
ARTIFACT_DIR="${OUT_ROOT}/artifacts/${PACKAGE_TARGET}"
ZIP_PATH="${ARTIFACT_DIR}/${SLUG}-${PACKAGE_TARGET}.zip"

rm -rf "${WORK_DIR}" "${BUNDLE_OUT}" "${ZIP_PATH}"
mkdir -p "${WORK_DIR}" "${BUNDLE_OUT}" "${ARTIFACT_DIR}"
cp -R tools/electron-host/. "${WORK_DIR}/"
rm -rf "${WORK_DIR}/app"
mkdir -p "${WORK_DIR}/app"
cp -R "${WEB_DIR}/." "${WORK_DIR}/app/"

python3 - "${WORK_DIR}/package.json" "${APP_NAME}" "${SLUG}" <<'PY'
import json
import sys
from pathlib import Path
path = Path(sys.argv[1])
payload = json.loads(path.read_text())
payload["name"] = sys.argv[3]
payload["productName"] = sys.argv[2]
payload["version"] = "0.1.0"
path.write_text(json.dumps(payload, indent=2) + "\n")
PY

(cd "${WORK_DIR}" && npm install)

if [ -n "${ELECTRON_PLATFORM}" ]; then
  (cd "${WORK_DIR}" && npx electron-packager . "${BUNDLE_NAME}" --out "${BUNDLE_OUT}" --overwrite --asar=false --platform "${ELECTRON_PLATFORM}")
else
  (cd "${WORK_DIR}" && npx electron-packager . "${BUNDLE_NAME}" --out "${BUNDLE_OUT}" --overwrite --asar=false)
fi

BUNDLE_PATH="$(find "${BUNDLE_OUT}" -mindepth 1 -maxdepth 1 -type d | head -n 1)"
test -n "${BUNDLE_PATH}"
python3 - "${ZIP_PATH}" "${BUNDLE_PATH}" <<'PY'
import sys
import zipfile
from pathlib import Path
zip_path = Path(sys.argv[1])
bundle_path = Path(sys.argv[2])
with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as zf:
    for file in sorted(bundle_path.rglob("*")):
        if file.is_file():
            zf.write(file, file.relative_to(bundle_path.parent).as_posix())
PY

python3 scripts/record-package-output.py \
  --manifest "${MANIFEST}" \
  --target "${PACKAGE_TARGET}" \
  --artifact "${ZIP_PATH}" \
  --out-root "${OUT_ROOT}"

echo "Built ${ZIP_PATH}"
