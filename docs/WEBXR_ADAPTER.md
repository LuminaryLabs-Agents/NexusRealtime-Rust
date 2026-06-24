# Native WebXR Adapter Model

NexusRealtime-Rust is not the game engine. It is a native host adapter that lets a NexusRealtime project ask for WebXR-shaped capabilities while the APK fulfills those requests with Android, OpenXR, and a GPU backend.

## Ownership boundary

Kits own domain meaning:

- XR session intent
- XR frame semantics
- XR input sources
- XR layer descriptors
- render descriptors
- grab and throw rules
- physics rules
- toon material and sky descriptors

The Rust host owns device capabilities:

- Android lifecycle
- native file access
- OpenXR loader, instance, session, spaces, actions, swapchains, and frame submission
- Vulkan or GLES backend setup
- APK packaging
- crash-safe logging

## WebXR-shaped contract

The adapter exposes concepts that mirror WebXR:

- `WebXrSessionRequest`
- `WebXrFramePacket`
- `WebXrViewPacket`
- `WebXrInputSourcePacket`
- `WebXrLayerPacket`
- `WebXrHostCapabilities`

The OpenXR backend maps those packets to native calls such as session creation, frame wait/begin/end, view location, action sync, and swapchain presentation.

## Renderer position

Vulkan and GLES are backend implementations. They do not own house logic, grab behavior, sky meaning, or toon material meaning. They consume descriptors emitted by kits and render them into host-provided frame targets.

Preferred production target: Vulkan.
Fallback/debug target: GLES.
Non-immersive fallback: Android canvas.

## Build artifact upload

The APK workflow uploads the debug APK as a GitHub artifact. If a Discord webhook is configured with `DISCORD_WEBHOOK_URL` as either a repository secret or variable, the workflow also posts the APK file to that webhook.
