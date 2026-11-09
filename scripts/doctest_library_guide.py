#!/usr/bin/env python3
"""SA-5-05: library-guide doctest gate (thin wrapper around book_pass_rate.py).

Default: en books/library-guide, --strict (exit 1 on any fail).
"""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    script = root / "scripts" / "book_pass_rate.py"
    syx = None
    args = sys.argv[1:]
    # Forward remaining args; always force library-guide + strict unless caller overrides.
    cmd = [sys.executable, str(script), "--library-guide", "--strict", *args]
    return subprocess.call(cmd)


if __name__ == "__main__":
    raise SystemExit(main())
