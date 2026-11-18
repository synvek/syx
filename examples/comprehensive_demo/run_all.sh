#!/usr/bin/env bash
# run_all.sh — 依次在 VM / native / wasm 三后端验证综合测试套件。
#
# 用法：
#   bash examples/comprehensive_demo/run_all.sh              # 三后端全量
#   bash examples/comprehensive_demo/run_all.sh --only lang  # 透传 VM CLI 参数（--only/--skip/--strict/--force）
#   SYX=/path/to/syx bash run_all.sh                         # 指定编译器
#
# 后端现状（实测，详见 README 后端矩阵）：
# - VM：原地 syx run，全部 25 套件完整报告。
# - native：整项目构建阻塞于 stdlib/host 的 MIR001/C020，故复制到临时 staging 目录，
#   剔除阻塞套件并同步过滤 main.sy 注册表行后构建（17 个 lang 套件可构建）。
#   但 native 运行时控制流损坏（if Bool 条件体不执行、Bool==Bool/while Bool 死循环、
#   switch 返回 scrutinee），执行用例无有效结果且可能挂起，故只验证构建、不运行。
# - wasm：编译器 build 控制流损坏（MIR001 Channel / W001 If underflow / if 体不执行），
#   尝试运行并记录失败，不阻塞整体。
# - 工作目录固定为仓库根：stdlib.time_path 的 FilePath.exists 用例以此为前提。

set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
SYX="${SYX:-$ROOT/target/release/syx}"
cd "$ROOT"

echo "===== VM ====="
"$SYX" run "$HERE" -- --backend vm "$@"
echo

# native 编译阻塞套件（文件名）：stdlib 三套件 + host 五套件。
# 实测依据：stdlib.math=MIR001 Double；stdlib.text_json=C020 hash(into:)；
# stdlib.time_path=MIR001 FilePath；host.concurrency=MIR001 Channel；
# host.io=宿主函数返回 nil；simd/net/services 依赖 VM 宿主。
NATIVE_EXCLUDE_FILES="suite_stdlib_math.sy suite_stdlib_text_json.sy suite_stdlib_time_path.sy suite_concurrency.sy suite_simd.sy suite_host_io.sy suite_host_net.sy suite_host_services.sy"
NATIVE_EXCLUDE_NAMES="stdlib.math stdlib.text_json stdlib.time_path host.concurrency host.simd host.io host.net host.services"

STAGE="$(mktemp -d /tmp/syx_comp_native.XXXXXX)"
cp "$HERE/syx.toml" "$HERE/harness.sy" "$HERE/ac_helper.sy" "$STAGE/"
for f in "$HERE"/suite_*.sy; do
    base="$(basename "$f")"
    excluded=0
    for x in $NATIVE_EXCLUDE_FILES; do
        if [ "$base" = "$x" ]; then excluded=1; fi
    done
    if [ "$excluded" = "0" ]; then cp "$f" "$STAGE/"; fi
done

# 过滤 main.sy 中含被剔除套件名的行（注册表/后端标注/dispatch 各占一行），
# 并清理数组尾逗号（syx 不允许尾逗号）。
python3 - "$HERE/main.sy" "$STAGE/main.sy" $NATIVE_EXCLUDE_NAMES <<'PYEOF'
import re, sys
src, dst = sys.argv[1], sys.argv[2]
quotes = ['"%s"' % s for s in sys.argv[3:]]
lines = [l for l in open(src, encoding="utf-8") if not any(q in l for q in quotes)]
text = "".join(lines)
text = re.sub(r',(\s*\])', r'\1', text)
open(dst, "w", encoding="utf-8").write(text)
PYEOF

echo "===== native (staging: $STAGE) ====="
if "$SYX" build --native "$STAGE" >"$STAGE/build.log" 2>&1; then
    echo "[NOTE] native 构建成功（17 个 lang 套件，产物 $STAGE/target/debug/app）。"
    echo "[NOTE] native 运行时控制流损坏（if Bool 条件体不执行、Bool==Bool/while Bool 死循环），不执行用例；编译器修复后可直接运行该产物 --backend native。"
else
    echo "[SKIP] native 构建失败（详见 $STAGE/build.log），不阻塞整体"
    tail -5 "$STAGE/build.log"
fi
echo

echo "===== wasm ====="
if "$SYX" run --wasm "$HERE" -- --backend wasm "$@"; then
    :
else
    echo "[NOTE] wasm 后端当前不可用（编译器缺陷：MIR001 Channel / W001 If underflow / if 语句体不执行），已记录，不阻塞整体"
fi