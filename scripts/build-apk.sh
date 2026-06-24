#!/usr/bin/env bash
set -euo pipefail

cargo test --workspace
cargo install cargo-ndk --locked
cargo ndk -t arm64-v8a -o app/android/app/src/main/jniLibs build --release -p nexus-android-bridge
(cd app/android && gradle --no-daemon assembleDebug)
