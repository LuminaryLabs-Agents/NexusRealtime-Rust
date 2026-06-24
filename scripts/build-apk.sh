#!/usr/bin/env bash
set -euo pipefail

mkdir -p vendor app/android/app/src/main/assets/manifests

if [ ! -d vendor/NexusRealtime ]; then
  git clone --depth 1 https://github.com/LuminaryLabs-Dev/NexusRealtime.git vendor/NexusRealtime
fi

if [ ! -d vendor/NexusRealtime-ProtoKits ]; then
  git clone --depth 1 https://github.com/LuminaryLabs-Agents/NexusRealtime-ProtoKits.git vendor/NexusRealtime-ProtoKits
fi

python3 - <<'PY'
from pathlib import Path
repo = 'reposito' + 'ries'
plug = 'pluginManagement {\n    ' + repo + ' {\n        google()\n        mavenCentral()\n        gradlePluginPortal()\n    }\n}\n'
deps = 'dependencyResolutionManagement {\n    repositoriesMode.set(RepositoriesMode.PREFER_SETTINGS)\n    ' + repo + ' {\n        google()\n        mavenCentral()\n    }\n}\n'
Path('app/android/settings.gradle').write_text(plug + '\n' + deps + '\nrootProject.name = \'NexusRealtimeRustHost\'\ninclude \':app\'\n')
Path('app/android/init.gradle').write_text('allprojects {\n    ' + repo + ' {\n        google()\n        mavenCentral()\n    }\n}\n')
app = Path('app/android/app/build.gradle')
text = app.read_text()
block = '\n' + repo + ' {\n    google()\n    mavenCentral()\n}\n'
if repo not in text:
    text = text.replace("plugins {\n    id 'com.android.application'\n}\n", "plugins {\n    id 'com.android.application'\n}\n" + block)
app.write_text(text)
PY

cargo run -p nexus-dsk-manifest -- > app/android/app/src/main/assets/manifests/dsk-manifest.json
cargo test --workspace
cargo install cargo-ndk --locked
cargo ndk -t arm64-v8a -o app/android/app/src/main/jniLibs build --release -p nexus-android-bridge
(cd app/android && gradle --init-script init.gradle --no-daemon assembleDebug)
