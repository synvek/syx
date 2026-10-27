#include "syx_plugin.h"
#include <string.h>

/* W1047 / ADR-1392: HANDLE kind demo (8-byte LE u64). Not compiled in CI. ≠ faer. */
static int encode_handle(syx_plugin_buf* out, unsigned long long id) {
    unsigned char bytes[8];
    for (int i = 0; i < 8; i++) {
        bytes[i] = (unsigned char)((id >> (8 * i)) & 0xffu);
    }
    if (!out || !out->ptr) {
        return 1;
    }
    memcpy(out->ptr, bytes, 8);
    out->kind = SYX_PLUGIN_BUF_HANDLE;
    out->len = 8;
    return 0;
}

static int unused_pin_demo(const syx_plugin_iov* in, syx_plugin_iov* out, syx_plugin_alloc* alloc) {
    (void)in;
    (void)alloc;
    if (!out || out->count < 1) {
        return 1;
    }
    return encode_handle(&out->bufs[0], 1ull);
}
