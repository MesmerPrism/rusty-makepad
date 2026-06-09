# Rusty Makepad Agent Notes

This is the clean source repository for Rusty Makepad. Keep committed content
self-contained and free of local-only planning paths, downstream app names,
platform-specific runtime handles, and historical naming drift.

Rusty Morphospace is the top-level project/platform umbrella. This repo remains
the Makepad adapter lane inside that umbrella: generic Makepad app shells,
Makepad widget adapters for Rusty GUI descriptors, and the canonical Makepad
app settings surface/resolver. Do not introduce `rusty.morphospace.*` schemas
here; use `rusty.gui.makepad.*` for generic Makepad GUI adapter contracts.

Project-owned source in this repo is licensed `AGPL-3.0-or-later`. The
upstream Makepad fork remains an upstream-derived toolkit dependency under its
own license and provenance.

## Purpose

Rusty Makepad owns generic Makepad adapters and Makepad app parameter surfaces.
It should remain usable without Quest/OpenXR platform policy, headset camera
behavior, Matter simulation truth, Manifold command authority, or product
workflow ownership.

## Read Order

1. `README.md`
2. `docs/ARCHITECTURE.md`
3. `docs/VALIDATION.md`
4. `fixtures/README.md`

## Architecture Rules

- Makepad app behavior is controlled through a canonical settings surface.
- Profiles, CLI flags, environment variables, Android properties, UI controls,
  and hotload requests are entry points into the same resolver.
- A profile is a value bundle over canonical setting ids, not a second place to
  define behavior.
- ADB and platform property writers belong in `rusty-quest` or another platform
  adapter. This repo may describe exposure names but does not write them.
- Quest/OpenXR Makepad apps belong in `rusty-quest-makepad`.
- Use `rusty.gui.makepad.*` schema ids for default Makepad adapter contracts.

## Validation

Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

