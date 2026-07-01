# macOS App And Platform Dynamic Libraries

Status: active

## Purpose

Make this build-only repo produce downloadable native artifacts from branch-push workflows.

## Success Criteria

- Add a tiny Rust-backed desktop app target.
- Package the desktop target as `NexusEngineRustDemo.app` on macOS.
- Build a dynamic-library tool-call surface as `.dylib`, `.so`, and `.dll` artifacts through GitHub Actions lanes.
- Keep Android APK generation intact as a separate artifact lane.
- Deploy a downloads page from the workflow so the README can link to current artifacts after pushes.
- Validate locally with compile/package commands where the current Mac can run them.

## Notes

- Do not treat this as a play/run repo.
- The first runnable target is the macOS `.app`; additional platform artifacts follow after that.
- GitHub-hosted artifacts should be produced on pushes to branches, not only manual dispatch.
