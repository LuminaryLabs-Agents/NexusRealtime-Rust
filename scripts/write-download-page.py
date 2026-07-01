#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path


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
        artifacts.append({
            "name": path.name,
            "url": f"./{path.name}",
            "bytes": path.stat().st_size,
        })

    payload = {
        "repo": args.repo,
        "branch": args.branch,
        "sha": args.sha,
        "artifacts": artifacts,
    }
    (downloads / "index.json").write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")

    rows = "\n".join(
        f'<li><a href="./{item["name"]}">{item["name"]}</a> <span>{item["bytes"]} bytes</span></li>'
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
