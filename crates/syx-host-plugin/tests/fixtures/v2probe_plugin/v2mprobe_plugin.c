#include "syx_plugin.h"
#include <string.h>

/* W1046 / ADR-1391: api_version 3 iov + realloc. ≠ faer. */

static int v2probe_dot(const syx_plugin_iov* in, syx_plugin_iov* out, syx_plugin_alloc* alloc) {
    if (!in || in->count < 2 || !in->bufs || !out || out->count < 1 || !out->bufs) {
        return 1;
    }
    if (!alloc || !alloc->realloc_fn) {
        return 1;
    }
    syx_plugin_buf* a = &in->bufs[0];
    syx_plugin_buf* b = &in->bufs[1];
    if (a->kind != SYX_PLUGIN_BUF_F64LE || b->kind != SYX_PLUGIN_BUF_F64LE) {
        return 1;
    }
    if (a->len != b->len || (a->len % 8) != 0) {
        return 1;
    }
    double acc = 0.0;
    for (size_t i = 0; i + 8 <= a->len; i += 8) {
        double x, y;
        memcpy(&x, a->ptr + i, 8);
        memcpy(&y, b->ptr + i, 8);
        acc += x * y;
    }
    unsigned char* p = alloc->realloc_fn(alloc->ctx, out->bufs[0].ptr, 8);
    if (!p) {
        return 1;
    }
    memcpy(p, &acc, 8);
    out->bufs[0].ptr = p;
    out->bufs[0].kind = SYX_PLUGIN_BUF_F64LE;
    out->bufs[0].len = 8;
    return 0;
}

static int v2probe_grow(const syx_plugin_iov* in, syx_plugin_iov* out, syx_plugin_alloc* alloc) {
    if (!in || in->count < 1 || !in->bufs || !out || out->count < 1 || !out->bufs) {
        return 1;
    }
    if (!alloc || !alloc->realloc_fn) {
        return 1;
    }
    size_t need = in->bufs[0].len;
    unsigned char* p = alloc->realloc_fn(alloc->ctx, out->bufs[0].ptr, need);
    if (!p) {
        return 1;
    }
    memcpy(p, in->bufs[0].ptr, need);
    out->bufs[0].ptr = p;
    out->bufs[0].kind = SYX_PLUGIN_BUF_F64LE;
    out->bufs[0].len = need;
    return 0;
}

static syx_host_fn_desc_v2m v2m_desc[2] = {
    {"__syx_plugin_v2probe_dot", "Array", 2, v2probe_dot},
    {"__syx_plugin_v2probe_grow", "Array", 1, v2probe_grow},
};

static syx_plugin_info_v2m v2m_info = {
    SYX_PLUGIN_API_VERSION_V2M,
    "v2probe",
    0,
    0,
    0,
    0,
    2,
    v2m_desc,
};

SYX_PLUGIN_EXPORT syx_plugin_info* syx_plugin_init(void) {
    return (syx_plugin_info*)&v2m_info;
}
