#ifndef SYX_PLUGIN_H
#define SYX_PLUGIN_H

#define SYX_PLUGIN_API_VERSION 1
#define SYX_PLUGIN_API_VERSION_V2 2
#define SYX_PLUGIN_API_VERSION_V2M 3

#define SYX_PLUGIN_BUF_UTF8 0u
#define SYX_PLUGIN_BUF_F64LE 1u
#define SYX_PLUGIN_BUF_HANDLE 2u /* W1047 / ADR-1392: 8-byte LE u64 host handle; not f64 */
#define SYX_PLUGIN_V2_OUT_CAP 65536u
#define SYX_PLUGIN_V2M_GROW_INITIAL_CAP 8u

#ifdef _WIN32
#define SYX_PLUGIN_EXPORT __declspec(dllexport)
#else
#define SYX_PLUGIN_EXPORT __attribute__((visibility("default")))
#endif

#include <stddef.h>

/* Host passes UTF-8 input; plugin writes NUL-terminated UTF-8 into out (max cap bytes). */
typedef void (*syx_host_fn)(const char* in, char* out, size_t out_cap);

typedef struct {
    const char* name;
    const char* return_type;
    unsigned int arity;
    syx_host_fn fn;
} syx_host_fn_desc;

typedef struct {
    unsigned int api_version;
    const char* plugin_id;
    unsigned int host_count;
    const syx_host_fn_desc* hosts;
} syx_plugin_info;

/* W1045 / ADR-1390: v2 single-slot typed buffer. Host owns out.ptr[out_cap].
   Plugin sets out->len; return 0 on success, nonzero on overflow (no silent truncate). */
typedef struct {
    unsigned int kind; /* 0 UTF-8, 1 f64-le, 2 HANDLE (W1047 / ADR-1392; 8-byte LE u64, not f64) */
    unsigned char* ptr;
    size_t len;
} syx_plugin_buf;

typedef int (*syx_host_fn_v2)(const syx_plugin_buf* in, syx_plugin_buf* out, size_t out_cap);

typedef struct {
    const char* name;
    const char* return_type;
    unsigned int arity;
    syx_host_fn_v2 fn;
} syx_host_fn_desc_v2;

/* v2 plugins only. v1 plugins MUST keep syx_plugin_info (do not append fields). */
typedef struct {
    unsigned int api_version;
    const char* plugin_id;
    unsigned int host_count;
    const syx_host_fn_desc* hosts;
    unsigned int v2_host_count;
    const syx_host_fn_desc_v2* v2_hosts;
} syx_plugin_info_v2;

/* W1046 / ADR-1391: multi-slot iov + host realloc. Read only when api_version==3. */
typedef struct {
    unsigned int count;
    syx_plugin_buf* bufs;
} syx_plugin_iov;

typedef unsigned char* (*syx_plugin_realloc_fn)(void* ctx, unsigned char* ptr, size_t new_cap);

/* W1051 / ADR-1396: host borrow/pin. Call-duration read-only; plugin must not
   drop/realloc the borrowed id until the host fn returns. Trailing fields —
   v2m plugins that only use ctx+realloc_fn remain valid. */
typedef int (*syx_plugin_borrow_fn)(
    void* ctx,
    unsigned long long id,
    syx_plugin_buf* buf,
    size_t* nrows,
    size_t* ncols);
typedef int (*syx_plugin_pin_fn)(
    void* ctx,
    size_t nrows,
    size_t ncols,
    const unsigned char* ptr,
    size_t len,
    unsigned long long* out_id);
/* W1055 / ADR-1400: explicit host-table drop. Trailing field — plugins that
   only use borrow_fn/pin_fn remain valid. */
typedef int (*syx_plugin_drop_fn)(void* ctx, unsigned long long id);

typedef struct {
    void* ctx;
    syx_plugin_realloc_fn realloc_fn;
    syx_plugin_borrow_fn borrow_fn; /* W1051 */
    syx_plugin_pin_fn pin_fn;       /* W1051 */
    syx_plugin_drop_fn drop_fn;     /* W1055 */
} syx_plugin_alloc;

typedef int (*syx_host_fn_v2m)(const syx_plugin_iov* in, syx_plugin_iov* out, syx_plugin_alloc* alloc);

typedef struct {
    const char* name;
    const char* return_type;
    unsigned int arity;
    syx_host_fn_v2m fn;
} syx_host_fn_desc_v2m;

typedef struct {
    unsigned int api_version;
    const char* plugin_id;
    unsigned int host_count;
    const syx_host_fn_desc* hosts;
    unsigned int v2_host_count;
    const syx_host_fn_desc_v2* v2_hosts;
    unsigned int v2m_host_count;
    const syx_host_fn_desc_v2m* v2m_hosts;
} syx_plugin_info_v2m;

typedef syx_plugin_info* (*syx_plugin_init_fn)(void);

#endif
