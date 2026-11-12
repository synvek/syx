#!/usr/bin/env python3
"""R-49 scoped markdown relative-link check (entry + high-traffic docs).

Exit 0 when broken==0 for the scoped set. Does NOT scan architecture-decisions.md.
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

LINK_RE = re.compile(r"\[([^\]]*)\]\(([^)]+)\)")

SCOPED = [
    "docs/README.md",
    "docs/zh/README.zh-CN.md",
    "docs/adr/README.md",
    "docs/adr/current.md",
    "docs/adr/capabilities-summary.md",
    "books/README.md",
    "docs/9_UsageGuide.md",
    "docs/zh/9_UsageGuide.zh-CN.md",
]


def check_file(path: Path) -> list[str]:
    text = path.read_text(encoding="utf-8", errors="replace")
    broken: list[str] = []
    for m in LINK_RE.finditer(text):
        url = m.group(2).strip()
        if url.startswith(("http://", "https://", "mailto:", "#")):
            continue
        url0 = url.split("#", 1)[0].split("?", 1)[0]
        if not url0:
            continue
        target = (path.parent / url0).resolve()
        if not target.exists():
            broken.append(url)
    return broken


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", type=Path, default=Path("."))
    args = ap.parse_args()
    root = args.root.resolve()
    total = 0
    for rel in SCOPED:
        path = root / rel
        if not path.is_file():
            print(f"MISSING {rel}")
            total += 1
            continue
        broken = check_file(path)
        print(f"{rel}: broken={len(broken)}")
        for u in broken:
            print(f"  - {u}")
            total += 1
    print(f"scoped_broken_total={total}")
    return 0 if total == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
