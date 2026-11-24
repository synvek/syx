"""C-6b: Cargo workspace version as capability syx_version SSOT helpers."""

from __future__ import annotations

import re
import sys
from pathlib import Path

_WS_VER_RE = re.compile(
    r'\[workspace\.package\][\s\S]*?^version\s*=\s*"([^"]+)"',
    re.M,
)
_SYX_VER_LINE_RE = re.compile(
    r'^(\s*syx_version\s*=\s*")([^"]*)("\s*(?:#.*)?)$',
    re.M,
)


def read_workspace_semver(root: Path) -> str:
    cargo = (root / "Cargo.toml").read_text(encoding="utf-8")
    m = _WS_VER_RE.search(cargo)
    if not m:
        print("FAIL: missing [workspace.package] version in Cargo.toml", file=sys.stderr)
        raise SystemExit(2)
    return m.group(1)


def read_toml_syx_version(path: Path) -> str | None:
    if not path.is_file():
        return None
    text = path.read_text(encoding="utf-8")
    m = re.search(r'^syx_version\s*=\s*"([^"]*)"', text, re.M)
    return m.group(1) if m else None


def rewrite_toml_syx_version(path: Path, version: str) -> bool:
    """Rewrite syx_version = \"...\" in place. Returns True if file changed."""
    text = path.read_text(encoding="utf-8")

    def _repl(m: re.Match[str]) -> str:
        return f"{m.group(1)}{version}{m.group(3)}"

    new_text, n = _SYX_VER_LINE_RE.subn(_repl, text, count=1)
    if n == 0:
        print(f"FAIL: {path}: missing syx_version = \"...\" line", file=sys.stderr)
        raise SystemExit(2)
    if new_text == text:
        return False
    path.write_text(new_text, encoding="utf-8")
    return True
