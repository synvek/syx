# Appendix: Implementation Status Matrix

This appendix mirrors [shared/implementation-status.md](../shared/implementation-status.md) for readers of this reference.

Please consult: **[SYX Language Feature Implementation Status Master Table](../shared/implementation-status.md)** (SYX **117.0** · Phase **139**)

That table lists the **Implemented** / **Partially implemented** / **Planned** / **Not implemented** / **GA** status by domain, and takes tests and source as ground truth.

Update policy: `shared/implementation-status.md` is revised with the current Phase; the appendices of both books keep only a link, to avoid duplicated maintenance.

> **Key changes in the current Phase 139:**
> - Language surface: multiline / tuple / nested path / closed range / Character / call labels
> - SIMD16 i8x16 all_true/any_true (guest-v22) plus the existing guest surface
> - SynUI deferred; Farm/Dist freeze v19
> - audit **2600/3050**; cache **`syx-cache-v1`**; native AOT **LLVM only**
> - Details in [language-guide Appendix A](../language-guide/appendix-a-implementation-matrix.md)

---

**See also** → [Language Guide Appendix A](../language-guide/appendix-a-implementation-matrix.md) · [Implementation Status Master Table](../shared/implementation-status.md) · [architecture-decisions.md](../../docs/adr/architecture-decisions.md)
