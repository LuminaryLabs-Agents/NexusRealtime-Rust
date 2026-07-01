# Workflow

Status: active

## Purpose

Use branch-push GitHub Actions as the main artifact builder and deployer.

## Notes

- `make macos-app` should build the local macOS `.app`.
- `make host-ffi` should compile the Rust dynamic-library tool-call surface for the current target.
- GitHub Actions should upload artifacts and deploy a Pages download surface.
- Keep build validation command-oriented; no gameplay or human-view run loop is required for this repo.
