# Memory

Status: active

## Purpose

Repo-local operating memory for NexusEngine-Rust build and packager work.

## Notes

- NexusEngine-Rust is build-only: validate build/package outputs, not gameplay.
- Universal packager inputs must be staged under `dist/packager/work/<slug>/source`; never run package builds inside dirty source app repos.
- GoldRush is the first proof input and should remain read-only unless the user explicitly changes direction.
- Native app wrappers consume `dist/packager/packages/<slug>/web` and its `nexus-package-manifest.json`.
