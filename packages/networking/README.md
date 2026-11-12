# Networking (SA-3-02)

`import Networking` — former `Net` import name retired (destructive; no alias).

## Domain sources (build-time gen only)

`packages/net/` and `packages/io/types/http_response.sy` are **build-time domain sources** concatenated into `packages/networking/mod.sy` (see `scripts/gen_networking_mod.py`). Users should `import Networking` — domain dirs are not a supported public import surface (todo-005 F3 honesty; trees not deleted this window).

R-47 does **not** Swiftify `Http` → `HTTPClient` or wipe `SyxHost` from Networking.
