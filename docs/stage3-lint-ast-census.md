# Stage-3 lint-ast census (S3-05)

**ADR:** [open](adr/stage3-s3-05-open.md) · [l007-l012-false-positive](adr/stage3-s3-05-l007-l012-false-positive.md) · [struct-extension-type-member](adr/stage3-s3-05-struct-extension-type-member.md) · [claim-prep](adr/stage3-s3-05-claim-prep.md) · [residual-scope](adr/stage3-s3-05-residual-scope.md) · **Window:** S3-I76 · **Current:** S3-I76 · **Claimed:** no（residual-scope；≠ `[x]`）

**Goal (S3-05 scoped):** Primary 误报轴清零 + I73/I74 铰链；L010–L011 / L012 Stmt / 全 CST lint = **deferred supersede-only**。  
**This census:** S3-I76 residual-scope — scoped acceptance 成文；S3-05 仍 **`[~]`**；Stage-3 仍 **`[~]`**（**4/7**）；**≠** Stage-3 exit / **≠** S3-05 `[x]` 本窗。

## Command

```bash
bash scripts/s3_05_lint_ast_census.sh
```

## Classification (hard)

| Class | Counts as S3-05 debt? | Notes |
|-------|----------------------|-------|
| L007 class-field false positive | **Hinged (I73)** | Primary |
| L012 nested `print(` false positive | **Hinged (I73)** | column-0 · Primary |
| Struct / Extension member context | **Hinged (I74)** | Primary structural |
| Claim-prep scaffolding | **Hinged (I75)** | fixtures + draft |
| Scoped acceptance / deferred hybrid | **Hinged (I76)** | this window |
| L010–L011 text→AST | **Deferred** | supersede-only |
| L012 Stmt AST | **Deferred** | supersede-only |
| Full CST lint / S2-04 Frontend | **Deferred** | supersede-only |
| S3-06 LSP / S3-07 init | **No（OUT）** | next workstreams |
| S3-04 fib / VM·JIT | **No（OUT）** | claimed; ≠ reopen |
| HostFn / Wave / F-08 / Stage-3 exit | **No（OUT）** | Hard OUT |

## KPIs（S3-I76 · Residual-scope · 2026-11-08）

Machine-local（prefer release `syx`）。**≠** claim S3-05 `[x]`。

| KPI | Value | Notes |
|-----|------:|-------|
| `residual_scope_helper` | **≥1** | I76 hinge |
| `claim_prep_helper` | **≥1** | I75 |
| `scoped_fixture_helper` | **≥1** | fixtures |
| `stage3_lint_census_helper` | **≥1** | ADR anchors |
| `l007_l012_knife_helper` | **≥1** | I73 |
| `struct_type_member_helper` | **≥1** | I74 |
| `syx_lint_lib_loc` | **≈380** | |
| `lint_source_text_helper` | **≥1** | hybrid residual visible |
| `lint_rules_ast_only` | **9** | L001–L009 |
| `lint_rules_text_hybrid` | **3** | L010–L012 deferred OK |
| `class_demo_l007_count` | **0** | Primary |
| `class_demo_l012_count` | **0** | Primary |
| `struct_book_point_l007_count` | **0** | Primary |
| `class_demo_fixture` | **≥1** | |
| `struct_demo_fixture` | **≥1** | |

## Gap（诚实）

| 轴 | 状态 |
|----|------|
| S3-05 | **`[~]`**；scoped acceptance 已写；**未** claim |
| Stage-3 | 仍 **`[~]`**（**4/7**） |
| Deferred | L010–L011 / L012 Stmt / full CST — supersede-only |
| Next | **S3-I77** scoped claim — ≠ Stage-3 exit |

## Ladder (locked)

1. **S3-I72:** S3-05 open — landed.
2. **S3-I73:** L007/L012 false-positive knife — landed.
3. **S3-I74:** Struct/Extension type-member — landed.
4. **S3-I75:** claim-prep — landed.
5. **S3-I76:** residual-scope — **this window**.
6. **S3-I77:** S3-05 scoped `[x]` claim — next.
7. **≠** Stage-3 exit until S3-05…07 claimed under exit ADR.

## Non-claims

- **≠** Stage-3 exit  
- **≠** S3-05 `[x]` this window  
- **≠** S3-06 / S3-07 construction  
- **≠** reopen S3-04 fib&lt;0.5 / VM·JIT knife  
- **≠** HostFn+1 / F-08 / Wave  
- **≠** CST lint rewrite / AST-ize L010–L012 this window  

## References

- [stage3-s3-05-residual-scope](adr/stage3-s3-05-residual-scope.md) · S3-I76
- [stage3-s3-05-claim-prep](adr/stage3-s3-05-claim-prep.md) · S3-I75
- [plan/02](../plan/02-improvement-roadmap.md) S3-05
