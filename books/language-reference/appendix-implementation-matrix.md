# 附录：实现状态矩阵

本附录与 [shared/implementation-status.md](../shared/implementation-status.md) 内容一致，供语言参考读者查阅。

请参阅：**[SYX 语言特性实现状态总表](../shared/implementation-status.md)**（SYX **117.0** · Phase **139**）

该表按领域列出 **已实现** / **部分实现** / **计划中** / **未实现** / **GA** 状态，并以测试用例与源码为 ground truth。

更新策略：随当前 Phase 修订 `shared/implementation-status.md`，两书附录仅保留链接避免重复维护。

> **当前 Phase 139 关键变化：**
> - 语言面：multiline / tuple / nested path / closed range / Character / call labels
> - SIMD16 i8x16 all_true/any_true（guest-v22）及既有 guest 表面
> - SynUI deferred；Farm/Dist freeze v19
> - audit **2600/3050**；cache **`syx-cache-v1`**；原生 AOT **LLVM sole**
> - 细节见 [language-guide 附录 A](../language-guide/appendix-a-implementation-matrix.md)

---

**参见** → [语言指南附录 A](../language-guide/appendix-a-implementation-matrix.md) · [实现状态总表](../shared/implementation-status.md) · [architecture-decisions.md](../../docs/adr/architecture-decisions.md)
