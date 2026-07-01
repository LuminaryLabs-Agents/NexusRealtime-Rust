#!/usr/bin/env bash
set -euo pipefail

OUT_ROOT="${OUT_ROOT:-dist/packager}"
PAGES_ROOT="${PAGES_ROOT:-dist/pages}"
REPO_NAME="${REPO_NAME:-LuminaryLabs-Agents/NexusEngine-Rust}"
BRANCH_NAME="${BRANCH_NAME:-local}"
COMMIT_SHA="${COMMIT_SHA:-local}"

rm -rf "${PAGES_ROOT}/downloads"
mkdir -p "${PAGES_ROOT}/downloads"

if [ -d "${OUT_ROOT}/artifacts" ]; then
  find "${OUT_ROOT}/artifacts" -mindepth 2 -maxdepth 2 -type f | while read -r artifact; do
    cp "${artifact}" "${PAGES_ROOT}/downloads/"
  done
fi

if [ -d "${OUT_ROOT}/packages" ]; then
  find "${OUT_ROOT}/packages" -name nexus-package-manifest.json -type f | while read -r manifest; do
    slug="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("slug","nexus-package"))' "${manifest}")"
    cp "${manifest}" "${PAGES_ROOT}/downloads/${slug}-nexus-package-manifest.json"
  done
fi

python3 scripts/write-download-page.py \
  --output "${PAGES_ROOT}" \
  --repo "${REPO_NAME}" \
  --branch "${BRANCH_NAME}" \
  --sha "${COMMIT_SHA}"

echo "Wrote ${PAGES_ROOT}/downloads/index.html"
