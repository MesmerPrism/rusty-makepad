# Rusty Makepad

Rusty Makepad is the Morphospace lane for generic Makepad adapters, desktop and
Android 2D Makepad app-shell code, and canonical Makepad app settings.

The first source slice provides:

- `rusty.gui.makepad.app_settings_surface.v1`
- `rusty.gui.makepad.settings_profile.v1`
- `rusty.gui.makepad.effective_settings.v1`
- a resolver that produces deterministic effective settings with provenance.

Quest-specific profile bundles and headset app behavior belong in
`rusty-quest-makepad`; platform write/readback transports belong in
`rusty-quest`.

## Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

