#!/usr/bin/env bash
set -euo pipefail

TARGET="${1:-}"
if [ -z "${TARGET}" ]; then
  TARGET="$(rustc -vV | awk '/host:/ {print $2}')"
fi

case "${TARGET}" in
  *apple-darwin) LIB_FILE="libnexus_host_ffi.dylib" ;;
  *windows*) LIB_FILE="nexus_host_ffi.dll" ;;
  *) LIB_FILE="libnexus_host_ffi.so" ;;
esac

cargo build --release -p nexus-host-ffi --target "${TARGET}"

OUT_DIR="dist/host-ffi/${TARGET}"
rm -rf "${OUT_DIR}"
mkdir -p "${OUT_DIR}"
cp "target/${TARGET}/release/${LIB_FILE}" "${OUT_DIR}/${LIB_FILE}"
cat > "${OUT_DIR}/README.txt" <<TXT
NexusEngine Rust host FFI artifact
target=${TARGET}
library=${LIB_FILE}

Exported C ABI:
- nexus_host_demo_status() -> char*
- nexus_host_tick_summary(sequence_json: *const char, manifest_json: *const char) -> char*
- nexus_host_string_free(value: char*)
TXT

echo "Built ${OUT_DIR}/${LIB_FILE}"
