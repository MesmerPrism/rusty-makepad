# Rusty Makepad Architecture

Rusty Makepad owns generic Makepad adapters and Makepad app settings contracts.

## Settings Authority

Every app declares one settings surface:

```text
rusty.gui.makepad.app_settings_surface.v1
```

Profiles are value bundles over that surface:

```text
rusty.gui.makepad.settings_profile.v1
```

The resolver emits:

```text
rusty.gui.makepad.effective_settings.v1
```

The effective report records final values, winning source layers, rejected
lower layers, setting revisions, generated time, and source ids.

## Non-Ownership

Rusty Makepad does not write ADB properties, own Quest/OpenXR runtime behavior,
or own command authority. Platform writers consume the exposure map and produce
evidence in their own repos.

