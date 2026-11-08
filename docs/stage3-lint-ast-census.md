# Stage-3 lint-ast census (S3-05)

**ADR:** [open](adr/stage3-s3-05-open.md) · [l007-l012-false-positive](adr/stage3-s3-05-l007-l012-false-positive.md) · [struct-extension-type-member](adr/stage3-s3-05-struct-extension-type-member.md) · [claim-prep](adr/stage3-s3-05-claim-prep.md) · [residual-scope](adr/stage3-s3-05-residual-scope.md) · [**claim**](adr/stage3-s3-05-claim.md) · **Window:** S3-I77 · **Current:** S3-I77 · **Claimed:** scoped `[x]`

**Goal (S3-05 scoped `[x]`):** Primary 误报轴清零 + I73/I74 铰链 + I75/I76 脚手架；L010–L011 / L012 Stmt / 全 CST lint = **deferred supersede-only**。  
**This census:** S3-I77 scoped claim — S3-05 **`[x]`**；Stage-3 仍 **`[~]`**（**5/7**）；**≠** Stage-3 exit / **≠** 本窗 S3-06 施工。

## Command

```bash
bash scripts/s3_05_lint_ast_census.sh
```

## Classification (hard)

| Class | Counts as S3-05 debt? | Notes |
|-------|----------------------|-------|
| L007 / L012 false positives | **Claimed** | Primary = 0 |
| Struct / Extension context | **Claimed** | I74 |
| Claim-prep / residual-scope | **Claimed** | I75–I76 |
| L010–L011 text→AST | **Deferred** | supersede-only |
| L012 Stmt AST | **Deferred** | supersede-only |
| Full CST lint / S2-04 Frontend | **Deferred** | supersede-only |
| S3-06 LSP | **Next workstream** | I78 open |
| S3-07 init / HostFn / Wave / Stage-3 exit | **No（OUT）** | Hard OUT |

## KPIs（S3-I77 · Claim · 2026-11-08）

Machine-local（prefer release `syx`）。Claimed under scoped meaning（见 claim ADR）。

| KPI | Value | Notes |
|-----|------:|-------|
| `s3_05_claim_helper` | **≥1** | I77 hinge |
| `residual_scope_helper` | **≥1** | I76 |
| `claim_prep_helper` | **≥1** | I75 |
| `scoped_fixture_helper` | **≥1** | fixtures |
| `l007_l012_knife_helper` | **≥1** | I73 |
| `struct_type_member_helper` | **≥1** | I74 |
| `stage3_lint_census_helper` | **≥1** | ADR anchors |
| `lint_rules_ast_only` | **9** | L001–L009 |
| `lint_rules_text_hybrid` | **3** | deferred OK |
| `class_demo_l007_count` | **0** | Primary claimed |
| `class_demo_l012_count` | **0** | Primary claimed |
| `struct_book_point_l007_count` | **0** | Primary claimed |

## Gap（诚实）

| 轴 | 状态 |
|----|------|
| S3-05 | **`[x]`** scoped；hybrid deferred |
| Stage-3 | 仍 **`[~]`**（**5/7**；余 S3-06…07） |
| Next | **S3-I78** S3-06 open — ≠ Stage-3 exit；**≠** 本窗施工 |

## Ladder (locked)

1. **S3-I72:** S3-05 open — landed.
2. **S3-I73:** L007/L012 false-positive knife — landed.
3. **S3-I74:** Struct/Extension type-member — landed.
4. **S3-I75:** claim-prep — landed.
5. **S3-I76:** residual-scope — landed.
6. **S3-I77:** S3-05 scoped claim — **this window**.
7. **S3-I78:** S3-06 open — next（≠ Stage-3 exit）。

## Non-claims

- **≠** Stage-3 exit  
- **≠** S3-06 / S3-07 construction this window  
- **≠** restore hybrid as claim hard gate  
- **≠** reopen S3-04 fib&lt;0.5 / VM·JIT knife  
- **≠** HostFn+1 / F-08 / Wave  

## References

- [stage3-s3-05-claim](adr/stage3-s3-05-claim.md) · S3-I77
- [stage3-s3-05-residual-scope](adr/stage3-s3-05-residual-scope.md) · S3-I76
- [plan/02](../plan/02-improvement-roadmap.md) S3-05
