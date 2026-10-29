//! Thin C ABI around `lld::lldMain` for static embedding into SYX.

#ifdef __cplusplus
extern "C" {
#endif

int syx_lld_link(int argc, const char **argv);

#ifdef __cplusplus
}
#endif

#if defined(SYX_LLD_HAS_DRIVER)
#include "lld/Common/Driver.h"
#include "llvm/Support/raw_ostream.h"
#include <string>
#include <vector>

LLD_HAS_DRIVER(coff)
LLD_HAS_DRIVER(elf)
LLD_HAS_DRIVER(mingw)
LLD_HAS_DRIVER(macho)
LLD_HAS_DRIVER(wasm)

int syx_lld_link(int argc, const char **argv) {
  std::vector<const char *> args(argv, argv + argc);
  // Buffer diagnostics: `/force:multiple` may warn on CRT GS symbol overlap
  // (msvcrt vs vcruntime). Only surface output when the link actually fails.
  std::string diag;
  llvm::raw_string_ostream diag_os(diag);
  lld::Result r = lld::lldMain(args, llvm::nulls(), diag_os, LLD_ALL_DRIVERS);
  if (r.retCode != 0 && !diag.empty()) {
    llvm::errs() << diag;
  }
  return r.retCode;
}
#else
int syx_lld_link(int argc, const char **argv) {
  (void)argc;
  (void)argv;
  return 1;
}
#endif
