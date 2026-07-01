# Goal

Status: active

## Purpose

Build and publish native artifacts from this build-only repo.

## Notes

- Success requires a branch-push workflow that builds artifacts online.
- Success requires a small macOS `.app` generated from Rust backend code.
- Success requires dynamic-library artifacts for platform lanes where the workspace can compile them.
- Success requires README download links that point to deployed workflow output.
