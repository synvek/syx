#include "syx_plugin.h"
#include <string.h>

static void echo_host(const char* in, char* out, size_t out_cap) {
    if (out_cap == 0) {
        return;
    }
    strncpy(out, in ? in : "", out_cap - 1);
    out[out_cap - 1] = '\0';
}

static syx_host_fn_desc echo_host_desc = {
    "__syx_plugin_echo_echo",
    "String",
    1,
    echo_host,
};

static syx_plugin_info echo_info = {
    SYX_PLUGIN_API_VERSION,
    "echo",
    1,
    &echo_host_desc,
};

SYX_PLUGIN_EXPORT syx_plugin_info* syx_plugin_init(void) {
    return &echo_info;
}
