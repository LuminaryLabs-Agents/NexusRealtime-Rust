# NexusRealtime-Rust

Lightweight native Rust host surface for NexusRealtime with Android APK build workflow.

## Current implementation status

This repository is a native host scaffold for NexusRealtime. It is intended to host NexusRealtime-authored experiences across native targets such as Android, Quest/OpenXR, headless replay, and renderer-specific presentation backends.

It should not become a separate gameplay engine. Gameplay truth remains in the NexusRealtime core runtime, kits, DSKs, and sequences.

## Local domain kit staging

The `local-kits/` folder contains the current local staging contracts for the native host/domain kit plan.

These kits model the host architecture as domain boundaries:

```txt
NexusRealtime Core Runtime
  kits / DSKs / sequences
        ↓
Rust Host Kernel Domain
  project loading
  host profile selection
  input routing
  command buffer output
  diagnostics / logs
        ↓
Host / presentation / artifact domains
  Android lifecycle
  OpenXR session
  XR input
  GLES renderer
  stereo renderer
  headless replay
  build artifact logs
```

Important local kit files:

```txt
local-kits/README.md
local-kits/ARCHITECTURE.md
local-kits/PROMOTION_MAP.md
local-kits/native-host-domain-kits.mjs
local-kits/examples/native-host-composition.mjs
local-kits/tests/native-host-domain-kits.test.mjs
local-kits/targets/README.md
```

Run the local kit smoke test with:

```bash
node local-kits/tests/native-host-domain-kits.test.mjs
```

Run the composition example with:

```bash
node local-kits/examples/native-host-composition.mjs
```

## Downloads

Branch pushes build deployable artifacts through GitHub Actions and publish the latest download page here:

- Downloads page: [https://luminarylabs-agents.github.io/NexusEngine-Rust/downloads/](https://luminarylabs-agents.github.io/NexusEngine-Rust/downloads/)
- Artifact manifest: [https://luminarylabs-agents.github.io/NexusEngine-Rust/downloads/index.json](https://luminarylabs-agents.github.io/NexusEngine-Rust/downloads/index.json)

Expected latest files after a successful workflow run:

- `nexusengine-rust-macos-app.zip`
- `nexus-host-ffi-macos-aarch64.zip`
- `nexus-host-ffi-linux-x64.zip`
- `nexus-host-ffi-windows-x64.zip`
- `nexusrealtime-rust-debug-apk.zip`
- `nexusengine-goldrush-web-static.zip`
- `nexusengine-goldrush-macos-app.zip`
- `nexusengine-goldrush-android-debug.apk`
- `nexusengine-goldrush-ios-simulator-app.zip`
- `nexusengine-goldrush-windows-exe.zip`
- `nexusengine-goldrush-electron-mac.zip`
- `nexusengine-goldrush-electron-win.zip`
- `nexusengine-goldrush-electron-linux.zip`
- `nexusengine-goldrush-nexus-package-manifest.json`

Local build commands:

```bash
make macos-app
make host-ffi
make package-nexus PROJECT=/path/to/nexus-project
make package-goldrush
make package-downloads
```

Direct packager commands:

```bash
cargo run -p nexus-packager -- inspect /path/to/repo --json
cargo run -p nexus-packager -- build /path/to/repo --target web-static --out dist/packager
cargo run -p nexus-packager -- package /path/to/repo --targets macos-app,android-apk,ios-sim,windows-exe,electron,web-static
```

`nexus-packager` never builds inside the input project. It stages a copy in
`dist/packager/work/<slug>/source`, excluding `.git`, `node_modules`, `dist`,
`target`, `output`, and `.playwright-cli`, then writes a normalized web bundle
and `nexus-package-manifest.json` under `dist/packager/packages/<slug>/`.

Optional project config can be supplied in `nexus.pack.json`:

```json
{
  "name": "NexusEngine GoldRush",
  "appId": "dev.luminarylabs.nexuspackaged.goldrush",
  "entry": "index.html",
  "kind": "nexusrealtime-vite",
  "buildCommand": "npm run build",
  "webOutDir": "dist",
  "targets": ["web-static", "macos-app"],
  "assetBase": "./"
}
```

## Build branch policy

```txt
main  = normal development branch
build = controlled full-build trigger branch
```

The long-term build pipeline should run every supported platform lane from `build`, collect all logs/artifact manifests, write a full build report into the repository, and send only a compact summary to Discord.
