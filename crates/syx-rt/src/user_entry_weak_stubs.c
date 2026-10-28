/* Weak defaults for host / staticlib links on ELF and Mach-O.
 *
 * `syx_rt_start` references these; real native AOT objects provide strong
 * definitions that override the weaks at final link. Do not compile this on
 * MSVC (COFF weak replacement differs); Windows host tools use syx-driver stubs.
 */

#include <stdint.h>

__attribute__((weak)) void syx_user_init(void) {}

__attribute__((weak)) int32_t syx_user_main(void) { return 0; }
