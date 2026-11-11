#!/usr/bin/env bash
# R-6 Channel.handle census (post-SA residual).
# KPIs: r5 hinge still green · handle ≠ Object · create→Int · id accepts Int|bag.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-6 Channel.handle census =="

python3 - <<'PY'
import re, subprocess, sys
from pathlib import Path

root = Path(".")
fail = 0

def die(msg: str) -> None:
    global fail
    print(f"FAIL: {msg}", file=sys.stderr)
    fail = 1

def ok(key: str, val: object = 1) -> None:
    print(f"{key}={val}")

# --- Roll up R-5 (includes r4…Mutex/Semaphore) ---
r = subprocess.run(
    ["bash", str(root / "scripts/r5_mutex_sem_census.sh")],
    cwd=root,
    capture_output=True,
    text=True,
)
print(r.stdout, end="")
if r.stderr:
    print(r.stderr, end="", file=sys.stderr)
if r.returncode != 0:
    die("r5_mutex_sem_census failed (R-6 requires R-5 hinge stay green)")
else:
    ok("r5_hinge_still_green", 1)

sync = root / "packages/sync/mod.sy"
text = sync.read_text(encoding="utf-8")

m = re.search(
    r"public class Channel\s*\{(.*?)\n\}",
    text,
    flags=re.S,
)
if not m:
    die("packages/sync/mod.sy: public class Channel not found")
    channel_body = ""
else:
    channel_body = m.group(1)

# handle must be Int, not Object
handle_obj = re.search(r"var\s+handle\s*:\s*Object\b", channel_body)
handle_int = re.search(r"var\s+handle\s*:\s*Int\b", channel_body)
if handle_obj or not handle_int:
    die("Channel.handle must be Int (not Object)")
ok("channel_handle_object_residual", 0)

# SyxHost.channel_create → Int
host = (root / "packages/internal/system_host.sy").read_text(encoding="utf-8")
if not re.search(r"func\s+channel_create\s*\(\s*n0:\s*Int\s*\)\s*->\s*Int\b", host):
    die("SyxHost.channel_create must return Int")
ok("channel_create_returns_int_syxhost", 1)

# registry ChannelCreate => "Int"
reg = (root / "crates/syx-stdlib/src/registry.rs").read_text(encoding="utf-8")
# Look near ChannelCreate return-type arm
if not re.search(
    r'HostFnId::ChannelCreate\s*=>\s*"Int"',
    reg,
):
    die('registry ChannelCreate must map to "Int"')
ok("channel_create_returns_int_registry", 1)

# VM: host_channel_create pushes platform_int (no bag push at create)
vm = (root / "crates/syx-vm/src/channel_hosts.rs").read_text(encoding="utf-8")
# Extract host_channel_create function body
cm = re.search(
    r"pub fn host_channel_create\(stack: &mut Vec<Value>\) -> SyxResult<\(\)> \{(.*?)^\}",
    vm,
    flags=re.S | re.M,
)
if not cm:
    die("host_channel_create not found")
    create_body = ""
else:
    create_body = cm.group(1)

# Ignore comments: fail only if code still builds a bag with __channel_id.
code_lines = [
    ln for ln in create_body.splitlines()
    if not ln.strip().startswith("//")
]
code_body = "\n".join(code_lines)
if re.search(r'insert\s*\(\s*"__channel_id"', code_body) or (
    "__channel_id" in code_body and "Value::Object" in code_body
):
    die("host_channel_create must not push {__channel_id} bag")
if "platform_int_value(id)" not in create_body and "Value::I64(id)" not in create_body:
    die("host_channel_create must push bare Int id")
ok("channel_create_returns_int", 1)

# channel_id_from_value accepts bare I64/I32 before Object bag
depth_fn = re.search(
    r"fn channel_id_from_value_depth\(v: &Value, depth: u8\) -> SyxResult<i64> \{(.*?)^\}",
    vm,
    flags=re.S | re.M,
)
if not depth_fn:
    die("channel_id_from_value_depth not found")
    depth_body = ""
else:
    depth_body = depth_fn.group(1)

head = depth_body[:1500]
obj_pos = head.find("Value::Object")
i64_ok = "Value::I64(n) => return Ok(*n)" in head or re.search(
    r"Value::I64\([^\)]*\)\s*=>\s*return Ok", head
)
i32_ok = "Value::I32(n) => return Ok(*n as i64)" in head or re.search(
    r"Value::I32\([^\)]*\)\s*=>\s*return Ok", head
)
i64_pos = head.find("Value::I64")
if not (i64_ok and i32_ok and obj_pos >= 0 and 0 <= i64_pos < obj_pos):
    die("channel_id_from_value_depth must accept bare I64/I32 before Object bag")
ok("channel_id_accepts_int", 1)

# bag dual-track still present (Object __channel_id path kept)
if "__channel_id" not in depth_body:
    die("channel_id_from_value_depth must keep __channel_id bag dual-track")
ok("channel_id_bag_dual_track", 1)

print("current=R-6-exited")
print("next=R-7-plus-residual")
print("r6_exit_hinge=met")
sys.exit(fail)
PY
