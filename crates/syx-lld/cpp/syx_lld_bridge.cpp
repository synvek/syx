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
#include <vector>

LLD_HAS_DRIVER(coff)
LLD_HAS_DRIVER(elf)
LLD_HAS_DRIVER(mingw)
LLD_HAS_DRIVER(macho)
LLD_HAS_DRIVER(wasm)

int syx_lld_link(int argc, const char **argv) {
  std::vector<const char *> args(argv, argv + argc);
  lld::Result r =
      lld::lldMain(args, llvm::outs(), llvm::errs(), LLD_ALL_DRIVERS);
  return r.retCode;
}
#else
int syx_lld_link(int argc, const char **argv) {
  (void)argc;
  (void)argv;
  return 1;
}
#endif
