#!/usr/bin/env python3
"""Generate MirOp / HostFn capability tables (M1-B04/B05 / ADR-136).

Rust enums are the authoritative inventory (parsed from source).
TOML overrides annotate backends/status; missing override ⇒ status=unsupported
and all backends=unsupported (fail-closed; every inventory name gets an explicit row).

Usage:
  python3 scripts/gen_ir_capabilities.py
  python3 scripts/gen_ir_capabilities.py --check
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MIR_RS = ROOT / "crates" / "syx-mir" / "src" / "lib.rs"
HOST_RS = ROOT / "crates" / "syx-stdlib" / "src" / "registry.rs"
MIR_TOML = ROOT / "packages" / "internal" / "mir_op_capabilities.toml"
HOST_TOML = ROOT / "packages" / "internal" / "host_fn_capabilities.toml"
OUT_MIR_MD = ROOT / "books" / "shared" / "mir-op-capabilities.md"
OUT_HOST_MD = ROOT / "books" / "shared" / "host-fn-capabilities.md"
OUT_MIR_JSON = ROOT / "packages" / "internal" / "mir_op_capabilities.generated.json"
OUT_HOST_JSON = ROOT / "packages" / "internal" / "host_fn_capabilities.generated.json"

STR_RE = re.compile(r'^(\w+)\s*=\s*"([^"]*)"\s*$')
BACKENDS_RE = re.compile(r"^backends\s*=\s*\{([^}]*)\}\s*$")
VARIANT_RE = re.compile(r"^([A-Za-z_][A-Za-z0-9_]*)\s*(?:\(|\{|,|$)")

BACKEND_KEYS = ("vm", "mir", "llvm", "wasm", "cranelift")
DEFAULT_BACKENDS = {k: "unsupported" for k in BACKEND_KEYS}
FEATURE_STATUSES = {
    "implemented",
    "partial",
    "experimental",
    "experimental-real-subset",
    "preview",
    "unsupported",
    "stub",
    "frozen",
}
BACKEND_STATUSES = {
    "real",
    "partial",
    "host-assisted",
    "stub",
    "unsupported",
    "noop",
}
WEAK_BACKENDS = {"noop", "stub", "unsupported"}


def parse_enum_variants(path: Path, enum_name: str) -> list[str]:
    text = path.read_text(encoding="utf-8")
    m = re.search(rf"pub enum {enum_name} \{{([\s\S]*?)\n\}}", text)
    if not m:
        raise SystemExit(f"enum {enum_name} not found in {path}")
    names: list[str] = []
    seen: set[str] = set()
    for raw in m.group(1).splitlines():
        line = raw.strip()
        if not line or line.startswith("/") or line.startswith("#"):
            continue
        mm = VARIANT_RE.match(line)
        if not mm:
            continue
        name = mm.group(1)
        if name in seen:
            continue
        seen.add(name)
        names.append(name)
    if not names:
        raise SystemExit(f"no variants parsed for {enum_name}")
    return names


def parse_backends(inner: str) -> dict[str, str]:
    out = {}
    for part in inner.split(","):
        part = part.strip()
        if not part:
            continue
        m = re.match(r'(\w+)\s*=\s*"([^"]*)"', part)
        if m:
            out[m.group(1)] = m.group(2)
    return out


def load_overrides(path: Path, row_key: str) -> tuple[dict, dict[str, dict]]:
    """Return (meta, name -> override dict)."""
    meta: dict = {}
    rows: dict[str, dict] = {}
    section = None
    current: dict | None = None

    if not path.exists():
        return meta, rows

    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.split("#", 1)[0].strip()
        if not line:
            continue
        if line == "[meta]":
            if current is not None and "name" in current:
                rows[current["name"]] = current
                current = None
            section = "meta"
            continue
        if line == f"[[{row_key}]]":
            if current is not None and "name" in current:
                rows[current["name"]] = current
            current = {}
            section = "row"
            continue
        target = meta if section == "meta" else current
        if target is None:
            continue
        m = BACKENDS_RE.match(line)
        if m:
            target["backends"] = parse_backends(m.group(1))
            continue
        m = STR_RE.match(line)
        if m:
            target[m.group(1)] = m.group(2)
            continue
    if current is not None and "name" in current:
        rows[current["name"]] = current
    return meta, rows


def build_table(
    kind: str,
    inventory: list[str],
    meta: dict,
    overrides: dict[str, dict],
) -> dict:
    unknown_override = sorted(set(overrides) - set(inventory))
    if unknown_override:
        raise SystemExit(
            f"{kind}: override names not in inventory ({len(unknown_override)}): "
            f"{unknown_override[:10]}"
        )
    rows = []
    errors: list[str] = []
    for name in inventory:
        ov = overrides.get(name, {})
        backends = dict(DEFAULT_BACKENDS)
        backends.update(ov.get("backends") or {})
        # Every row always has a complete backend map (product requirement).
        for key in BACKEND_KEYS:
            backends.setdefault(key, "unsupported")
        status = ov.get("status") or "unsupported"
        if status not in FEATURE_STATUSES:
            errors.append(f"{kind}.{name}: unknown status {status!r}")
        for key, val in backends.items():
            if val not in BACKEND_STATUSES:
                errors.append(f"{kind}.{name}: unknown backend {key}={val!r}")
        if status == "implemented":
            # Cranelift is the JIT backend axis (B-08); HostFn/MirOp rows may keep
            # cranelift=unsupported while marking inventory status=implemented when
            # vm/mir/llvm/wasm are production-ready (W441 / ADR-785 first HostFn raise).
            weak = {
                k: v
                for k, v in backends.items()
                if v in WEAK_BACKENDS and k != "cranelift"
            }
            if weak:
                errors.append(
                    f"{kind}.{name}: status=implemented but weak backends {weak}"
                )
        # Overrides that set a non-default status must include an explicit backends map.
        if name in overrides and "backends" not in overrides[name]:
            errors.append(
                f"{kind}.{name}: override missing backends={{...}} "
                f"(backend-level status required for every HostFn/MirOp override)"
            )
        rows.append(
            {
                "name": name,
                "status": status,
                "backends": backends,
                "notes": ov.get("notes") or "",
                "pilot": ov.get("pilot") or "",
            }
        )
    if errors:
        for e in errors:
            print(f"INVALID: {e}", file=sys.stderr)
        raise SystemExit(f"{kind}: {len(errors)} capability honesty error(s)")
    return {
        "meta": {
            "version": meta.get("version", "0.1"),
            "syx_version": meta.get("syx_version", "117.0"),
            "phase": int(meta.get("phase", 139)) if str(meta.get("phase", "139")).isdigit() else 139,
            "schema": meta.get("schema", f"syx-{kind}-capability-v0"),
            "kind": kind,
            "default": "unsupported",
            "count": len(rows),
            "backend_keys": list(BACKEND_KEYS),
        },
        "ops" if kind == "mir_op" else "hosts": rows,
    }


def render_md(data: dict, title: str, row_key: str) -> str:
    meta = data["meta"]
    rows = data[row_key]
    lines = [
        f"# {title}",
        "",
        f"> Generated by `scripts/gen_ir_capabilities.py` (ADR-136 / M1-B). Do not edit by hand.",
        "",
        f"- schema: `{meta['schema']}`",
        f"- SYX {meta['syx_version']} · phase {meta['phase']}",
        f"- rows: **{meta['count']}** (inventory complete; default = `{meta['default']}`)",
        "",
        "| Name | Status | VM | MIR | LLVM | WASM | Cranelift | Notes |",
        "|------|--------|----|-----|------|------|-----------|-------|",
    ]
    for r in rows:
        b = r["backends"]
        lines.append(
            "| {name} | {status} | {vm} | {mir} | {llvm} | {wasm} | {cranelift} | {notes} |".format(
                name=r["name"],
                status=r["status"],
                vm=b.get("vm", ""),
                mir=b.get("mir", ""),
                llvm=b.get("llvm", ""),
                wasm=b.get("wasm", ""),
                cranelift=b.get("cranelift", ""),
                notes=(r.get("notes") or "").replace("|", "\\|"),
            )
        )
    lines.append("")
    return "\n".join(lines)


def write_or_check(path: Path, content: str, check: bool) -> bool:
    if check:
        if not path.exists():
            print(f"missing {path}", file=sys.stderr)
            return False
        existing = path.read_text(encoding="utf-8")
        if existing != content:
            print(f"stale {path}", file=sys.stderr)
            return False
        return True
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    return True


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()

    mir_names = parse_enum_variants(MIR_RS, "MirOp")
    host_names = parse_enum_variants(HOST_RS, "HostFnId")
    mir_meta, mir_ov = load_overrides(MIR_TOML, "op")
    host_meta, host_ov = load_overrides(HOST_TOML, "host")

    mir_data = build_table("mir_op", mir_names, mir_meta, mir_ov)
    host_data = build_table("host_fn", host_names, host_meta, host_ov)

    ok = True
    ok &= write_or_check(
        OUT_MIR_JSON, json.dumps(mir_data, indent=2, ensure_ascii=False) + "\n", args.check
    )
    ok &= write_or_check(
        OUT_HOST_JSON, json.dumps(host_data, indent=2, ensure_ascii=False) + "\n", args.check
    )
    ok &= write_or_check(
        OUT_MIR_MD, render_md(mir_data, "MirOp capability table", "ops"), args.check
    )
    ok &= write_or_check(
        OUT_HOST_MD, render_md(host_data, "HostFn capability table", "hosts"), args.check
    )

    if args.check:
        if not ok:
            print("gen_ir_capabilities.py --check FAILED", file=sys.stderr)
            return 1
        print(
            f"gen_ir_capabilities.py --check OK ({len(mir_names)} MirOp, {len(host_names)} HostFn)"
        )
        return 0

    print(f"wrote {OUT_MIR_JSON.relative_to(ROOT)} ({len(mir_names)} ops)")
    print(f"wrote {OUT_HOST_JSON.relative_to(ROOT)} ({len(host_names)} hosts)")
    print(f"wrote {OUT_MIR_MD.relative_to(ROOT)}")
    print(f"wrote {OUT_HOST_MD.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
