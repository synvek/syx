/* Bridge CRT `_start` → SYX entry.
 *
 * Bare `-e syx_rt_start` skips glibc `__libc_start_main`, so a normal `ret`
 * from the process entry is undefined on Linux ELF and typically segfaults.
 * Inject this object only when linking user natives (not into libsyx_rt.a /
 * the syx CLI) so CRT can call `main` as usual.
 */
extern int syx_rt_start(int argc, const char **argv);

int main(int argc, char **argv) {
  return syx_rt_start(argc, (const char **)argv);
}
