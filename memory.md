# NexusEngine-Rust Memory

## Purpose

NexusEngine-Rust is a build-only native Rust host surface for NexusRealtime artifacts. It should compile host libraries, app bundles, APKs, and deployable download artifacts; it should not be treated as a playtest/gameplay repo.

## Architecture Shape

- Rust workspace crates hold host/runtime/domain boundaries.
- `crates/nexus-host-ffi` exposes a C ABI dynamic-library surface for device/app wrappers.
- `app/macos/NexusEngineRustDemo` is the first small desktop wrapper that loads the Rust backend through `libnexus_host_ffi.dylib`.
- `crates/nexus-packager` is the Universal NexusRealtime Packager CLI. It recognizes NexusRealtime/Vite/static/Rust project shapes, stages source copies, normalizes web bundles, and writes `nexus-package-manifest.json`.
- `crates/nexus-static-server` serves packaged web bundles on localhost for native WebView hosts.
- `app/macos/NexusPackagedWebApp`, `app/android-webview`, `app/ios-webview`, and `tools/electron-host` are generic host templates consumed by packager scripts.
- `.github/workflows/build-apk.yml` is the branch-push artifact workflow for tests, dynamic libraries, macOS app packaging, Android APK packaging, and Pages downloads.

## Conventions

- Prefer build/package validation over runtime play validation.
- Keep platform outputs as downloadable artifacts from GitHub Actions and GitHub Pages.
- Track active build goals in `.agent/goal.md` and `.agent/goal-packets/`.
- Track durable repo decisions here only when they affect future work.
- Do not build in source app repos. Package inputs are copied into `dist/packager/work/<slug>/source` with `.git`, `node_modules`, `dist`, `target`, `output`, and `.playwright-cli` excluded.
- GoldRush is the first read-only sample input for universal packaging.
