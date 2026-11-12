#!/usr/bin/env python3
"""R-46: Ch17 en↔zh syx fence body sync nail (≠ full-tree F5)."""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EN = ROOT / "books/language-guide/17-error-handling.md"
ZH = ROOT / "books/zh-cn/language-guide/17-error-handling.md"

ANTI = [
    (r'throw\s+"', "throw_string"),
    (r"catch\s*\(", "catch_paren"),
    (r"if\s*\(!", "if_bang_paren"),
]


def extract_fences(path: Path) -> list[str]:
    text = path.read_text(encoding="utf-8")
    return [m.group(1) for m in re.finditer(r"```syx[^\n]*\n(.*?)```", text, re.S)]


def main() -> int:
    en = extract_fences(EN)
    zh = extract_fences(ZH)
    if len(en) != len(zh):
        print(f"FAIL: fence count en={len(en)} zh={len(zh)}", file=sys.stderr)
        return 1
    mism = [i for i, (a, b) in enumerate(zip(en, zh)) if a != b]
    if mism:
        print(f"FAIL: fence body mismatches at indices {mism}", file=sys.stderr)
        return 1
    body = "\n".join(en)
    for pat, name in ANTI:
        n = len(re.findall(pat, body))
        if n != 0:
            print(f"FAIL: Ch17 fence residual {name}={n}", file=sys.stderr)
            return 1
        print(f"ch17_fence_{name}=0")
    print(f"ch17_fence_count={len(en)}")
    print("ch17_en_zh_fence_sync=1")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
