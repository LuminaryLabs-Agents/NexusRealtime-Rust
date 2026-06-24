#!/usr/bin/env bash
set -euo pipefail

if command -v gradle >/dev/null 2>&1; then
  exit 0
fi

VERSION="8.10.2"
HOME_DIR="${HOME}/.nexusrealtime-tools"
mkdir -p "${HOME_DIR}"
ZIP="${HOME_DIR}/gradle-${VERSION}-bin.zip"
DIR="${HOME_DIR}/gradle-${VERSION}"

if [ ! -d "${DIR}" ]; then
  curl -L "https://services.gradle.org/distributions/gradle-${VERSION}-bin.zip" -o "${ZIP}"
  unzip -q "${ZIP}" -d "${HOME_DIR}"
fi

export PATH="${DIR}/bin:${PATH}"
gradle --version
