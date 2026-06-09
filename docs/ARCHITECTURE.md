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

## Hotload Decisions

Hotload is not a separate settings authority. A hotload file, UI proposal, or
session command submits a `rusty.gui.makepad.hotload_proposal.v1` payload with
canonical setting ids. The settings crate accepts only values whose
`writer_policy` and `hotload_policy` allow runtime change, rejects the rest with
reasons, and emits `rusty.gui.makepad.hotload_decision.v1` plus a new
`rusty.gui.makepad.effective_settings.v1` report.

## Non-Ownership

Rusty Makepad does not write ADB properties, own Quest/OpenXR runtime behavior,
or own command authority. Platform writers consume the exposure map and produce
evidence in their own repos.
