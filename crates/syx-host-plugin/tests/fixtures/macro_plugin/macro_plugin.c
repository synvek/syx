#include "../../../include/syx_plugin.h"
#include <stdio.h>
#include <string.h>

static char macro_resp_buf[4096];

SYX_PLUGIN_EXPORT const char* syx_macro_expand(const char* request_json) {
    if (!request_json) {
        return NULL;
    }
    if (strstr(request_json, "\"role\":\"accessor\"") != NULL
        || strstr(request_json, "\"role\": \"accessor\"") != NULL) {
        snprintf(
            macro_resp_buf,
            sizeof macro_resp_buf,
            "{\"stmts\":[],\"decls\":[\"func get() -> Int { return self.value }\",\"func set(newValue: Int) { self.value = newValue }\"],\"expr\":null}");
        return macro_resp_buf;
    }
    if (strstr(request_json, "\"role\":\"deinit\"") != NULL
        || strstr(request_json, "\"role\": \"deinit\"") != NULL) {
        snprintf(
            macro_resp_buf,
            sizeof macro_resp_buf,
            "{\"stmts\":[\"print(\\\"plugin:deinit\\\")\"],\"decls\":[],\"expr\":null}");
        return macro_resp_buf;
    }
    if (strstr(request_json, "\"role\":\"extension\"") != NULL
        || strstr(request_json, "\"role\": \"extension\"") != NULL) {
        snprintf(
            macro_resp_buf,
            sizeof macro_resp_buf,
            "{\"stmts\":[],\"decls\":[\"func label() -> String { return \\\"plugin:label\\\" }\"],\"expr\":null}");
        return macro_resp_buf;
    }
    if (strstr(request_json, "SmokeMemberMacro") != NULL) {
        snprintf(
            macro_resp_buf,
            sizeof macro_resp_buf,
            "{\"stmts\":[\"func externalMember() { print(\\\"plugin:Point\\\") }\"],\"decls\":[],\"expr\":null}");
        return macro_resp_buf;
    }
    /* Phase 119: production JSON-protocol cdylib type (not Smoke/Codable-only). */
    if (strstr(request_json, "JsonPluginMacro") != NULL
        || strstr(request_json, "ProdMemberMacro") != NULL) {
        snprintf(
            macro_resp_buf,
            sizeof macro_resp_buf,
            "{\"stmts\":[],\"decls\":[\"func pluginOk() -> String { return \\\"plugin-prod:ok\\\" }\"],\"expr\":null}");
        return macro_resp_buf;
    }
    snprintf(
        macro_resp_buf,
        sizeof macro_resp_buf,
        "{\"stmts\":[],\"decls\":[],\"expr\":null,\"error\":\"unknown macro\"}");
    return macro_resp_buf;
}

SYX_PLUGIN_EXPORT syx_plugin_info* syx_plugin_init(void) {
    return NULL;
}
