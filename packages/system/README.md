# System (SA-3-01)

Official companion module: `import System`.

Merges former import names (destructive; no aliases):

- `Os` · `Process` · `Env` · `Path` · `IO` · `Time` · `DateTime`

Sources remain under `packages/os`, `packages/process`, `packages/io`, `packages/core/{env,path,time}.sy`, `packages/datetime` and are concatenated by `embedded_std::SYSTEM_SOURCE`. API Swiftify (`ProcessInfo` / `FilePath` / hide `__syx_*`) is **SA-4**.
