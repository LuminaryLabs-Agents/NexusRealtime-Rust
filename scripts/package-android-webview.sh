#!/usr/bin/env bash
set -euo pipefail

PACKAGE_DIR="${1:?package directory is required}"
OUT_ROOT="${2:-dist/packager}"
MANIFEST="${PACKAGE_DIR}/nexus-package-manifest.json"

META="$(python3 - "${MANIFEST}" <<'PY'
import json
import sys
from pathlib import Path
payload = json.loads(Path(sys.argv[1]).read_text())
print(payload.get("slug") or "nexus-packaged-app")
print(payload["webDir"])
PY
)"

SLUG="$(printf '%s\n' "${META}" | sed -n '1p')"
WEB_DIR="$(printf '%s\n' "${META}" | sed -n '2p')"
WORK_DIR="${OUT_ROOT}/work/${SLUG}/android-webview"
ARTIFACT_DIR="${OUT_ROOT}/artifacts/android-apk"
APK_PATH="${ARTIFACT_DIR}/${SLUG}-android-debug.apk"

rm -rf "${WORK_DIR}"
mkdir -p "${WORK_DIR}" "${ARTIFACT_DIR}"
cp -R app/android-webview/. "${WORK_DIR}/"
rm -rf "${WORK_DIR}/app/src/main/assets/app"
mkdir -p "${WORK_DIR}/app/src/main/assets/app"
cp -R "${WEB_DIR}/." "${WORK_DIR}/app/src/main/assets/app/"

if command -v sdkmanager >/dev/null 2>&1; then
  yes | sdkmanager "platform-tools" "platforms;android-35" "build-tools;35.0.0" || true
fi

if ! command -v gradle >/dev/null 2>&1; then
  bash scripts/ensure-gradle.sh
  export PATH="${HOME}/.nexusrealtime-tools/gradle-8.10.2/bin:${PATH}"
fi

(cd "${WORK_DIR}" && gradle --no-daemon assembleDebug)
FOUND_APK="$(find "${WORK_DIR}/app/build/outputs/apk/debug" -name '*.apk' | head -n 1)"
test -n "${FOUND_APK}"
cp "${FOUND_APK}" "${APK_PATH}"

python3 scripts/record-package-output.py \
  --manifest "${MANIFEST}" \
  --target android-apk \
  --artifact "${APK_PATH}" \
  --out-root "${OUT_ROOT}"

echo "Built ${APK_PATH}"
