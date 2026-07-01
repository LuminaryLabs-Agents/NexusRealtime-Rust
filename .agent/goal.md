# Goal

Status: active

## Purpose

Build and publish native artifacts and packaged NexusRealtime apps from this build-only repo.

## Notes

- Success requires a branch-push workflow that builds artifacts online.
- Success requires a small macOS `.app` generated from Rust backend code.
- Success requires dynamic-library artifacts for platform lanes where the workspace can compile them.
- Success requires README download links that point to deployed workflow output.
- Current priority: Universal NexusRealtime Packager.
- Success requires packaging a recognized app folder without modifying it, starting with `/Users/crimsonwheeler/Documents/GitHub/NexusEngine-GoldRush`.
- Packager outputs should include normalized static web zips, macOS `.app`, Android debug APK, iOS simulator `.app`, Windows `.exe` via Electron, and Electron desktop bundles.
