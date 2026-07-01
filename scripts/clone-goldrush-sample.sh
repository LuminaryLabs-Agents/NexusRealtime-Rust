#!/usr/bin/env bash
set -euo pipefail

DEST="${1:-sample/NexusEngine-GoldRush}"
REPO="${NEXUS_PACKAGER_SAMPLE_REPO:-https://github.com/LuminaryLabs-Dev/NexusEngine-GoldRush.git}"
BRANCH="${NEXUS_PACKAGER_SAMPLE_BRANCH:-development}"

rm -rf "${DEST}"
mkdir -p "$(dirname "${DEST}")"
git config --global core.longpaths true || true
git clone --no-checkout --filter=blob:none --branch "${BRANCH}" --depth 1 "${REPO}" "${DEST}"

(
  cd "${DEST}"
  git sparse-checkout init --no-cone
  cat > .git/info/sparse-checkout <<'PATTERNS'
/*
!/.playwright-cli/
!/output/
!/screenshots/
!/reports/
!/docs/
PATTERNS
  git checkout
)
