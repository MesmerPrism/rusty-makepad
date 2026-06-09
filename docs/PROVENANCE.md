# Rusty Makepad Provenance

## Runtime Settings Resolver

The `rusty-makepad-settings` crate is a new AGPL Morphospace implementation of
the canonical Makepad app settings surface and resolver.

Reference pressure came from the old public runtime-config crate in the legacy
source repo, specifically these lessons:

- typed runtime values need source metadata;
- layered settings need deterministic precedence;
- same-precedence layers need deterministic later-wins behavior;
- readback/log markers need enough provenance to explain why a value won;
- Android properties and environment variables are input transports, not
  setting authority.

Rejected overreach:

- no dependency on the legacy source repo;
- no old compatibility aliases;
- no projection-only key registry as the generic Makepad authority;
- no ADB/property writes in this repo.

The active schemas are:

- `rusty.gui.makepad.app_settings_surface.v1`
- `rusty.gui.makepad.settings_profile.v1`
- `rusty.gui.makepad.effective_settings.v1`

