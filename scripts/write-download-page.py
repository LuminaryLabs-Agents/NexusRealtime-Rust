#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def infer_target(name: str) -> str:
    checks = [
        ("macos-app", "macos-app"),
        ("ios-simulator", "ios-sim"),
        ("ios-sim", "ios-sim"),
        ("android", "android-apk"),
        ("windows-exe", "windows-exe"),
        ("electron-mac", "electron-mac"),
        ("electron-win", "electron-win"),
        ("electron-linux", "electron-linux"),
        ("web-static", "web-static"),
        ("host-ffi", "host-ffi"),
        ("debug-apk", "android-apk"),
    ]
    for needle, target in checks:
        if needle in name:
            return target
    if name.endswith("manifest.json"):
        return "manifest"
    return "artifact"


def infer_platform(target: str, name: str) -> str:
    if "macos" in name or target in {"macos-app", "electron-mac"}:
        return "macos"
    if "windows" in name or target in {"windows-exe", "electron-win"}:
        return "windows"
    if "linux" in name or target == "electron-linux":
        return "linux"
    if "android" in name or target == "android-apk":
        return "android"
    if "ios" in name or target == "ios-sim":
        return "ios-simulator"
    if target == "web-static":
        return "web"
    return "generic"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", default="dist/pages")
    parser.add_argument("--repo", default="LuminaryLabs-Agents/NexusEngine-Rust")
    parser.add_argument("--branch", default="unknown")
    parser.add_argument("--sha", default="unknown")
    args = parser.parse_args()

    root = Path(args.output)
    downloads = root / "downloads"
    downloads.mkdir(parents=True, exist_ok=True)

    artifacts = []
    for path in sorted(downloads.iterdir()):
        if path.name in {"index.html", "index.json"} or not path.is_file():
            continue
        target = infer_target(path.name)
        artifacts.append({
            "name": path.name,
            "url": f"./{path.name}",
            "bytes": path.stat().st_size,
            "sha256": sha256(path),
            "target": target,
            "platform": infer_platform(target, path.name),
        })

    payload = {
        "repo": args.repo,
        "branch": args.branch,
        "sha": args.sha,
        "artifacts": artifacts,
    }
    (downloads / "index.json").write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")

    rows = "\n".join(
        f'<li><a href="./{item["name"]}">{item["name"]}</a> '
        f'<span>{item["target"]} / {item["platform"]} / {item["bytes"]} bytes</span></li>'
        for item in artifacts
    ) or "<li>No artifacts were staged by this workflow run.</li>"

    html = f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>NexusEngine-Rust Downloads</title>
  <style>
    body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; margin: 32px; max-width: 920px; line-height: 1.45; }}
    code {{ background: #f2f4f7; padding: 2px 5px; border-radius: 4px; }}
    li {{ margin: 10px 0; }}
    span {{ color: #57606a; margin-left: 8px; }}
  </style>
</head>
<body>
  <h1>NexusEngine-Rust Downloads</h1>
  <p>Latest deployed artifacts for <code>{args.repo}</code>.</p>
  <p>Branch: <code>{args.branch}</code><br>Commit: <code>{args.sha}</code></p>
  <ul>
    {rows}
  </ul>
  <p><a href="./index.json">Artifact manifest JSON</a></p>
</body>
</html>
"""
    (downloads / "index.html").write_text(html, encoding="utf-8")
    (root / "index.html").write_text('<meta http-equiv="refresh" content="0; url=downloads/">\n', encoding="utf-8")


if __name__ == "__main__":
    main()
