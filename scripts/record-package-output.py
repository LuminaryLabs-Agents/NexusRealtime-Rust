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


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", required=True)
    parser.add_argument("--target", required=True)
    parser.add_argument("--artifact", required=True)
    parser.add_argument("--out-root", default="dist/packager")
    args = parser.parse_args()

    manifest_path = Path(args.manifest)
    artifact_path = Path(args.artifact)
    out_root = Path(args.out_root).resolve()
    payload = json.loads(manifest_path.read_text(encoding="utf-8"))
    rel_path = artifact_path.resolve()
    try:
        display_path = rel_path.relative_to(out_root).as_posix()
    except ValueError:
        display_path = rel_path.as_posix()

    output = {
        "target": args.target,
        "path": display_path,
        "bytes": artifact_path.stat().st_size,
        "sha256": sha256(artifact_path),
    }

    existing = payload.setdefault("targetOutputs", [])
    existing[:] = [
        item for item in existing
        if not (item.get("target") == args.target and item.get("path") == display_path)
    ]
    existing.append(output)
    manifest_path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")

    web_dir = payload.get("webDir")
    if web_dir:
        web_manifest = Path(web_dir) / "nexus-package-manifest.json"
        if web_manifest.parent.exists():
            web_manifest.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
