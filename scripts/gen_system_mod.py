#!/usr/bin/env python3
"""Regenerate packages/system/mod.sy from domain sources (SA-3-01 / SA-4-04a)."""
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
PARTS = [
    "packages/core/env.sy",
    "packages/core/path.sy",
    "packages/core/time.sy",
    "packages/datetime/mod.sy",
    "packages/os/mod.sy",
    "packages/process/mod.sy",
    "packages/process/process_builder.sy",
    "packages/io/mod.sy",
    "packages/io/file_handle.sy",
    "packages/io/file_stream.sy",
    "packages/io/file_watcher.sy",
]
hdr = (
    "/// SA-3-01: `import System` aggregate.\n"
    "/// Edit domain sources listed in scripts/gen_system_mod.py; re-run this script.\n"
    "/// Runtime/workspace load prepends packages/internal/system_host.sy (SA-4-04a).\n"
    "/// Keep crates/syx-driver/src/embedded_std.rs SYSTEM_SOURCE file list in sync.\n\n"
)
out = ROOT / "packages/system/mod.sy"
body = "\n".join((ROOT / p).read_text() for p in PARTS)
assert "__syx_" not in body, "public System domain still contains __syx_"
out.write_text(hdr + body + "\n")
print(f"wrote {out.relative_to(ROOT)} ({out.stat().st_size} bytes)")
