#include "syx_plugin.h"
#include <string.h>

/* W1045 / ADR-1390: sum packed f64-le; write one f64. ≠ faer. */
static int v2probe_sum(const syx_plugin_buf* in, syx_plugin_buf* out, size_t out_cap) {
    if (!in || !out || !out->ptr) {
        return 1;
    }
    if (in->kind != SYX_PLUGIN_BUF_F64LE || (in->len % 8) != 0) {
        return 1;
    }
    if (out_cap < 8) {
        return 1;
    }
    double acc = 0.0;
    const unsigned char* p = in->ptr;
    for (size_t i = 0; i + 8 <= in->len; i += 8) {
        double x;
        memcpy(&x, p + i, 8);
        acc += x;
    }
    memcpy(out->ptr, &acc, 8);
    out->kind = SYX_PLUGIN_BUF_F64LE;
    out->len = 8;
    return 0;
}

static syx_host_fn_desc_v2 v2_desc = {
    "__syx_plugin_v2probe_sum",
    "Array",
    1,
    v2probe_sum,
};

static syx_plugin_info_v2 v2_info = {
    SYX_PLUGIN_API_VERSION_V2,
    "v2probe",
    0,
    0,
    1,
    &v2_desc,
};

SYX_PLUGIN_EXPORT syx_plugin_info* syx_plugin_init(void) {
    return (syx_plugin_info*)&v2_info;
}
