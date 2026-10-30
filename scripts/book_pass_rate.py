#!/usr/bin/env python3
"""S1-I1: book code-block pass-rate baseline (language-guide).

Extracts fenced ```sy / ```syx / ```swift blocks from books/language-guide
(and optional --zh), runs `syx check` on each, prints totals.

Annotations (S1-09 draft; see docs/doctest-annotations.md):
  ```sy                 — full check (default)
  ```sy,check           — same as sy (explicit)
  ```sy,fragment        — wrap in `func main() { ... }` when no main present
  ```sy,error=CODE      — expect check failure containing CODE (counts as pass if matched)

Usage:
  python3 scripts/book_pass_rate.py [--syx PATH] [--zh] [--limit N] [--json OUT]
"""
from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
from dataclasses import asdict, dataclass
from pathlib import Path

FENCE_RE = re.compile(
    r"^```(?P<tag>sy(?:x)?(?P<ann>(?:,[\w.=-]+)*))?\s*\n(?P<body>.*?)^```\s*$",
    re.MULTILINE | re.DOTALL,
)
# Also accept ```swift as book historical fences (treated as sy,check).
SWIFT_RE = re.compile(
    r"^```swift\s*\n(?P<body>.*?)^```\s*$",
    re.MULTILINE | re.DOTALL,
)


@dataclass
class BlockResult:
    file: str
    index: int
    tag: str
    ok: bool
    kind: str  # pass | fail | expect_error_pass | expect_error_miss | skip
    detail: str


def parse_annotations(tag: str) -> set[str]:
    if not tag:
        return {"check"}
    parts = [p for p in tag.split(",") if p]
    # first token is language (sy / syx)
    return set(parts[1:] or ["check"])


def extract_blocks(text: str, path: Path) -> list[tuple[str, str, str]]:
    """Return list of (tag, body, source_label)."""
    out: list[tuple[str, str, str]] = []
    for i, m in enumerate(FENCE_RE.finditer(text), 1):
        tag = m.group("tag") or "sy"
        body = m.group("body")
        out.append((tag, body, f"{path.name}#{i}"))
    for i, m in enumerate(SWIFT_RE.finditer(text), 1):
        out.append(("sy,check", m.group("body"), f"{path.name}#swift{i}"))
    return out


def wrap_fragment(body: str) -> str:
    if re.search(r"\bfunc\s+main\s*\(", body):
        return body
    return f"func main() {{\n{body}\n}}\n"


def run_check(syx: str, source: str, timeout: float) -> tuple[bool, str]:
    with tempfile.TemporaryDirectory(prefix="syx-book-") as td:
        path = Path(td) / "snippet.sy"
        path.write_text(source, encoding="utf-8")
        try:
            proc = subprocess.run(
                [syx, "check", str(path)],
                capture_output=True,
                text=True,
                timeout=timeout,
            )
        except subprocess.TimeoutExpired:
            return False, "timeout"
        out = (proc.stdout or "") + (proc.stderr or "")
        # syx check may exit 0 even with diagnostics historically; treat diagnostic markers as fail.
        failed = proc.returncode != 0 or bool(
            re.search(r"×|error:|\[P\d{3}\]|\[C\d{3}\]|\[S\d{3}\]|\[M\d{3}\]", out)
        )
        return (not failed), out.strip()[:500]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--root",
        type=Path,
        default=Path(__file__).resolve().parents[1],
        help="repo root",
    )
    ap.add_argument("--syx", default=None, help="path to syx binary")
    ap.add_argument("--zh", action="store_true", help="also scan books/zh-cn/language-guide")
    ap.add_argument("--limit", type=int, default=0, help="max blocks (0 = all)")
    ap.add_argument("--timeout", type=float, default=30.0)
    ap.add_argument("--json", type=Path, default=None, help="write JSON report")
    args = ap.parse_args()

    root: Path = args.root
    syx = args.syx or str(root / "target" / "debug" / "syx")
    if not Path(syx).exists():
        # fall back to PATH
        syx = args.syx or "syx"

    guides = [root / "books" / "language-guide"]
    if args.zh:
        guides.append(root / "books" / "zh-cn" / "language-guide")

    md_files: list[Path] = []
    for g in guides:
        if g.is_dir():
            md_files.extend(sorted(g.glob("*.md")))

    results: list[BlockResult] = []
    total = 0
    for md in md_files:
        text = md.read_text(encoding="utf-8")
        for tag, body, label in extract_blocks(text, md):
            total += 1
            if args.limit and len(results) >= args.limit:
                break
            anns = parse_annotations(tag)
            expect_error = None
            for a in anns:
                if a.startswith("error="):
                    expect_error = a.split("=", 1)[1]
            src = wrap_fragment(body) if "fragment" in anns else body
            ok, detail = run_check(syx, src, args.timeout)
            if expect_error is not None:
                matched = expect_error in detail
                results.append(
                    BlockResult(
                        file=str(md.relative_to(root)),
                        index=total,
                        tag=tag,
                        ok=matched,
                        kind="expect_error_pass" if matched else "expect_error_miss",
                        detail=detail[:200],
                    )
                )
            else:
                results.append(
                    BlockResult(
                        file=str(md.relative_to(root)),
                        index=total,
                        tag=tag,
                        ok=ok,
                        kind="pass" if ok else "fail",
                        detail="" if ok else detail[:200],
                    )
                )
        if args.limit and len(results) >= args.limit:
            break

    passed = sum(1 for r in results if r.ok)
    n = len(results)
    rate = (100.0 * passed / n) if n else 0.0
    parse_failish = sum(
        1
        for r in results
        if (not r.ok) and re.search(r"\[P\d{3}\]|Expected ", r.detail)
    )

    print(f"book_pass_rate: {passed}/{n} = {rate:.1f}%")
    print(f"parse_fail_approx: {parse_failish}")
    print(f"syx: {syx}")
    print(f"guides: {', '.join(str(g.relative_to(root)) for g in guides if g.is_dir())}")

    # Per-chapter summary
    by_file: dict[str, list[BlockResult]] = {}
    for r in results:
        by_file.setdefault(r.file, []).append(r)
    print("--- by file ---")
    for f, rs in sorted(by_file.items()):
        p = sum(1 for x in rs if x.ok)
        print(f"{p}/{len(rs)}\t{f}")

    if args.json:
        payload = {
            "passed": passed,
            "total": n,
            "rate_pct": rate,
            "parse_fail_approx": parse_failish,
            "results": [asdict(r) for r in results],
        }
        args.json.write_text(json.dumps(payload, indent=2), encoding="utf-8")
        print(f"wrote {args.json}")

    return 0 if n > 0 else 1


if __name__ == "__main__":
    sys.exit(main())
