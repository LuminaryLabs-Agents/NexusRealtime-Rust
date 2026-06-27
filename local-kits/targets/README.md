# Local Target Profiles

The local kit catalog exposes four target compositions through `createNativeHostComposition(target)`.

## Targets

| Target | Purpose | Required external capability |
|---|---|---|
| `android-gles` | Android/JNI lifecycle plus GLES presentation | `n:platform.android` |
| `quest-openxr` | OpenXR session plus XR input and stereo presentation | `n:platform.openxr` |
| `headless` | CI/test/replay plus build artifact logging | `n:platform.headless`, `n:ci.runner` |
| `all` | Full local contract validation | all known external capabilities |

## Required command

```bash
node local-kits/tests/native-host-domain-kits.test.mjs
```

## Promotion gate

A target is not promotion-ready until:

```txt
[ ] composition validates
[ ] install order resolves
[ ] no missing provides/requires token
[ ] target has a host domain
[ ] target has an output domain
[ ] target exposes diagnostics
[ ] target can be tested headlessly
```
