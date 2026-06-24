# Quest OpenXR One-Go Implementation Plan

## Goal

Take NexusRealtime-Rust from a Quest-runnable WebXR-adapter fallback APK into a fully immersive Quest OpenXR APK in one coordinated implementation sprint.

The target output is a build-branch APK artifact that:

- installs on Quest 2, Quest 3, and Quest Pro
- launches through the immersive HMD OpenXR path
- creates a real OpenXR runtime session
- creates stereo projection swapchains
- renders per-eye frames through a renderer backend
- streams controller actions into WebXR-shaped input packets
- routes interaction through kits, not through the host shell
- supports grab, hold, release, and throw for at least one object
- keeps visual scene meaning in descriptors and kits
- keeps Rust as the native WebXR-compatible adapter/backend

## Architecture rule

NexusRealtime and ProtoKits own domain meaning.

NexusRealtime-Rust owns native host capabilities.

The APK packages both.

```text
NexusRealtime / ProtoKits / DSKs
  -> WebXR-shaped session, frame, input, layer, render, grab, physics, and visual descriptors

NexusRealtime-Rust
  -> native adapter that satisfies those WebXR-shaped requests through Android + OpenXR + renderer backend

Quest APK
  -> packaged host adapter + bundled project descriptors + native Rust library + Android manifest
```

## Non-negotiable boundary

Do not move these into Android Activity or OpenXR host logic:

- house scene rules
- grabbable object semantics
- throw rules
- toon material meaning
- sky/lighting meaning
- physics intent
- XR session intent
- render packet meaning

They belong in kits and descriptors.

The host can own only:

- Android lifecycle
- native asset access
- OpenXR loader/init/session/spaces/actions/swapchains/frame submission
- Vulkan or GLES device setup
- native logging
- APK packaging
- haptics execution
- GPU resource upload

## Implementation ledger

| Step | Workstream | Exact change | Done when |
|---:|---|---|---|
| 1 | Build config | Add root `build.json` and make Gradle/build script read it. | Changing `versionName` or `versionCode` in `build.json` changes APK metadata without editing Gradle. |
| 2 | Build validation | Keep `make apk` as the single build command and validate APK contents from `build.json`. | Workflow logs show required entries from `validation.requiredApkEntries`. |
| 3 | WebXR adapter | Promote `nexus-webxr-adapter` as the contract layer for sessions, frames, views, input sources, and layers. | `cargo test -p nexus-webxr-adapter` passes and supports immersive VR local-floor. |
| 4 | OpenXR backend | Implement `nexus-openxr-host` as a backend for the WebXR adapter, not as a game engine. | It maps `WebXrSessionRequest` to OpenXR session creation. |
| 5 | Android bridge | Add JNI calls for lifecycle: `nativeStartOpenXr`, `nativeOnResume`, `nativeOnPause`, `nativeShutdown`. | Activity can initialize and shut down native runtime without crash. |
| 6 | Loader/init | Implement Android OpenXR loader initialization and instance creation. | Logcat shows `LoaderReady` and `InstanceReady`. |
| 7 | Session/spaces | Implement system selection, session creation, and local-floor reference space. | Logcat shows `SessionReady` and `ReferenceSpaceReady`. |
| 8 | Swapchains | Create two-eye projection swapchains and depth targets. | Logcat shows `SwapchainsReady`; no crash on launch. |
| 9 | Blank stereo | Render a left/right color clear into swapchain images and submit frames. | Quest displays stable immersive stereo clear for 30 seconds. |
| 10 | Input actions | Add OpenXR action set for grip, trigger/select, thumbstick, pose, and haptics. | Logcat shows controller active state and changing button values. |
| 11 | Frame packets | Convert each OpenXR frame into `WebXrFramePacket`. | Frame packet contains predicted display time, two views, input sources, and projection layer packet. |
| 12 | Kit bridge | Feed WebXR-shaped input/frame packets to Nexus/kit layer before tick. | Same grab kit consumes synthetic and OpenXR input. |
| 13 | Renderer core | Add/solidify shared render descriptors independent of GLES/Vulkan. | Renderer backend receives descriptor packets, not domain objects. |
| 14 | GLES fallback | Use GLES for debug renderer path if Vulkan path is incomplete. | GLES path can clear/render two eyes from same descriptor packet. |
| 15 | Vulkan target | Add Vulkan backend as production target. | Vulkan backend can clear stereo swapchain images on Quest. |
| 16 | Sky render | Render gradient horizon sky from `sky-gradient-kit` descriptor. | Sky appears stable in both eyes. |
| 17 | Scene render | Render floor, house, cube, ball, and mug from project descriptors. | Objects appear with sane scale and reachable placement. |
| 18 | Toon shader | Implement 4-band toon material rendering. | House and props have visible four-band lighting. |
| 19 | Outline v1 | Implement first outline pass, preferably inverted hull initially. | House and cube have stable silhouettes. |
| 20 | Grab hover | Use `xr-grab-throw-kit` descriptors to determine hover. | Pointing at cube highlights it. |
| 21 | Grab hold | Attach selected object to controller pose while grip/select is held. | Cube follows the controller. |
| 22 | Throw release | Use recent pose history to compute release velocity. | Fast release throws farther than slow release. |
| 23 | Physics minimum | Add gravity, floor collision, friction, restitution for thrown props. | Cube lands and settles; ball bounces lightly. |
| 24 | Haptics | Map kit haptic requests to OpenXR haptic output. | Controller vibrates on grab start. |
| 25 | Performance profile | Implement Quest 2 baseline: 72 Hz, conservative render scale, MSAA 2x. | App runs for two minutes without obvious frame instability. |
| 26 | Artifact | Upload APK to GitHub artifact and optional Discord webhook. | Artifact exists and Discord upload skips safely when hook is missing. |

## Required root build.json

Add this file at the repository root and make Gradle/build scripts read it:

```json
{
  "schema": "nexusrealtime-rust.build.v1",
  "app": {
    "name": "NexusRealtime XR House",
    "namespace": "dev.luminarylabs.nexusrealtime",
    "applicationId": "dev.luminarylabs.nexusrealtime.rust",
    "versionCode": 1,
    "versionName": "0.1.0",
    "debuggable": true
  },
  "android": {
    "compileSdk": 35,
    "minSdk": 29,
    "targetSdk": 35,
    "buildTools": "35.0.0",
    "ndkVersion": "27.0.12077973",
    "abiFilters": ["arm64-v8a"]
  },
  "openxr": {
    "enabled": true,
    "immersiveHmd": true,
    "headtrackingRequired": false,
    "loaderDependency": "org.khronos.openxr:openxr_loader_for_android:1.0.34",
    "sessionMode": "immersive-vr",
    "referenceSpace": "local-floor",
    "preferredBackend": "vulkan",
    "fallbackBackend": "gles"
  },
  "quest": {
    "targets": ["quest-2", "quest-3", "quest-pro"],
    "baselineDevice": "quest-2",
    "targetRefreshRate": 72,
    "renderScale": 1.0,
    "msaa": 2
  },
  "project": {
    "id": "xr-house-demo",
    "assetRoot": "assets/nexus/xr-house-demo",
    "entrySequence": "scene.sequence.json",
    "runtime": "nexus-webxr-adapter"
  },
  "repos": {
    "core": "https://github.com/LuminaryLabs-Dev/NexusRealtime.git",
    "protokits": "https://github.com/LuminaryLabs-Agents/NexusRealtime-ProtoKits.git"
  },
  "artifacts": {
    "githubArtifactName": "nexusrealtime-rust-debug-apk",
    "apkFileName": "app-debug.apk",
    "discordFileName": "nexusrealtime-rust-debug.apk"
  },
  "discord": {
    "enabled": true,
    "env": "DISCORD_WEBHOOK_URL",
    "message": "NexusRealtime Rust APK build"
  },
  "validation": {
    "requiredApkEntries": [
      "AndroidManifest.xml",
      "lib/arm64-v8a/libnexus_android_bridge.so",
      "assets/nexus/xr-house-demo/project.json",
      "assets/nexus/xr-house-demo/host.adaptive.json",
      "assets/nexus/xr-house-demo/interaction.grab.json"
    ]
  }
}
```

## OpenXR runtime sequence

The implementation must execute this runtime order:

```text
Android Activity created
  -> nativeInit(project assets)
  -> nativeStartOpenXr(android context/activity)
  -> initialize loader
  -> create instance
  -> get HMD system
  -> create session
  -> create local-floor reference space
  -> create action set and bindings
  -> create projection swapchains
  -> enter frame loop

Each frame:
  -> xrWaitFrame
  -> xrBeginFrame
  -> xrLocateViews
  -> xrSyncActions
  -> build WebXrFramePacket
  -> feed Nexus/kit tick
  -> renderer consumes descriptors
  -> acquire swapchain image
  -> render each eye
  -> release swapchain image
  -> xrEndFrame
```

## Renderer backend policy

Use Vulkan as the production target, but keep GLES as fallback/debug until Vulkan is validated.

```text
Vulkan
  -> production Quest renderer
  -> preferredBackend in descriptors
  -> final target for performance

GLES
  -> fallback renderer
  -> faster bring-up and debug
  -> useful if Vulkan is not ready

Android Canvas
  -> non-immersive fallback shell only
  -> not a real Quest immersive path
```

## Acceptance criteria

The one-go implementation is acceptable only when all of these are true:

```text
1. Build branch workflow is green.
2. APK artifact exists.
3. APK installs with adb.
4. App launches on Quest.
5. OpenXR reaches SessionReady.
6. OpenXR reaches SwapchainsReady.
7. OpenXR reaches FrameLoopReady.
8. Both eyes receive rendered frames.
9. Controller input is visible in logs.
10. At least one cube can be selected, held, released, and thrown.
11. App runs for two minutes without crash.
12. GitHub artifact upload works.
13. Discord upload skips safely when DISCORD_WEBHOOK_URL is unset.
14. Discord upload posts APK when DISCORD_WEBHOOK_URL is set.
```

## Guardrails

Do not complete the work by moving domain behavior into Rust host code.

Do not make the OpenXR backend own scene rules.

Do not make Vulkan own toon or grab semantics.

Do not remove Android Canvas fallback until OpenXR is stable.

Do not claim Quest immersive success until a headset install shows real OpenXR session + swapchain + submitted frames.
