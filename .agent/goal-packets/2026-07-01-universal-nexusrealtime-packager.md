# Universal NexusRealtime Packager

Status: active

## Purpose

Turn `NexusEngine-Rust` into a build-only packager that can take a recognized NexusRealtime/Vite/static app folder, build it in isolation, and emit downloadable cross-device packages.

## Proof Input

- `/Users/crimsonwheeler/Documents/GitHub/NexusEngine-GoldRush`
- Treat GoldRush as read-only. Do not edit or clean its dirty worktree.

## Required Interfaces

- `nexus-packager inspect <path> --json`
- `nexus-packager build <path> --target web-static --out dist/packager`
- `nexus-packager package <path> --targets macos-app,android-apk,ios-sim,windows-exe,electron,web-static`
- `make package-nexus PROJECT=/path/to/repo`
- `make package-goldrush`
- `make package-downloads`

## Target Outputs

- Static HTML bundle zip
- macOS `.app` zip using `WKWebView` plus `nexus-static-server`
- Android WebView debug APK
- iOS simulator `.app` zip
- Windows `.exe` via Electron on Windows runner
- Electron app bundles for macOS, Windows, and Linux
- `nexus-package-manifest.json` with output hashes and warnings

## Validation

- `cargo test --workspace`
- `nexus-packager inspect` on GoldRush
- `make package-goldrush`
- Launch generated GoldRush macOS `.app`
- Verify web-static bundle through a local static server
- Push workflow and verify Pages links after deploy
