// Phase 123–124: PKCS#11 shared-library stub (symbol table + OpenSession/Login; not hardware GA).
#include <stdint.h>

void* C_GetFunctionList(void) { return (void*)0; }
uint32_t C_Initialize(void* init_args) {
    (void)init_args;
    return 0;
}
uint32_t C_Finalize(void* reserved) {
    (void)reserved;
    return 0;
}
/* Phase 133: expose ≥2 slots for hsm-multi-slot MVP (not hardware GA). */
uint32_t C_GetSlotList(unsigned char token_present, void* slot_list, unsigned long* count) {
    (void)token_present;
    if (count == 0) {
        return 1;
    }
    if (slot_list == 0) {
        *count = 2;
        return 0;
    }
    if (*count < 2) {
        *count = 2;
        return 1;
    }
    {
        unsigned long* slots = (unsigned long*)slot_list;
        slots[0] = 0;
        slots[1] = 1;
    }
    *count = 2;
    return 0;
}

/* OpenSession stub: per-slot session handle (not hardware GA). */
uint32_t C_OpenSession(
    unsigned long slot_id,
    unsigned long flags,
    void* application,
    void* notify,
    unsigned long* session) {
    (void)flags;
    (void)application;
    (void)notify;
    if (session == 0) {
        return 1;
    }
    *session = 0x70010002UL + slot_id;
    return 0;
}

/* Login stub: accepts any pin for session MVP (not hardware GA). */
uint32_t C_Login(
    unsigned long session,
    unsigned long user_type,
    const unsigned char* pin,
    unsigned long pin_len) {
    (void)session;
    (void)user_type;
    (void)pin;
    (void)pin_len;
    return 0;
}

uint32_t C_Logout(unsigned long session) {
    (void)session;
    return 0;
}

uint32_t C_CloseSession(unsigned long session) {
    (void)session;
    return 0;
}

uint32_t C_Sign(
    unsigned long session,
    unsigned char* data,
    unsigned long data_len,
    unsigned char* signature,
    unsigned long* signature_len) {
    (void)session;
    if (data == 0 || data_len == 0 || signature == 0 || signature_len == 0) {
        return 1;
    }
    signature[0] = 0xAB;
    *signature_len = 1;
    return 0;
}

/* Phase 130: C_Verify stub accepts C_Sign stub signature (0xAB) for same payload (not hardware GA). */
uint32_t C_Verify(
    unsigned long session,
    unsigned char* data,
    unsigned long data_len,
    unsigned char* signature,
    unsigned long signature_len) {
    (void)session;
    if (data == 0 || data_len == 0 || signature == 0 || signature_len == 0) {
        return 1;
    }
    if (signature[0] != 0xAB) {
        return 1;
    }
    return 0;
}

/* Phase 131: C_Digest stub binds payload into a digest prefix 0xD1 (not hardware GA). */
uint32_t C_Digest(
    unsigned long session,
    unsigned char* data,
    unsigned long data_len,
    unsigned char* digest,
    unsigned long* digest_len) {
    (void)session;
    if (data == 0 || data_len == 0 || digest == 0 || digest_len == 0) {
        return 1;
    }
    if (*digest_len < data_len + 1) {
        *digest_len = data_len + 1;
        return 1;
    }
    digest[0] = 0xD1;
    {
        unsigned long i;
        for (i = 0; i < data_len; i++) {
            digest[i + 1] = data[i];
        }
    }
    *digest_len = data_len + 1;
    return 0;
}

/* Phase 132: streaming digest Init/Update/Final (stub; not hardware GA). */
#define SYX_DIGEST_STREAM_CAP 256
static unsigned char g_digest_stream[SYX_DIGEST_STREAM_CAP];
static unsigned long g_digest_stream_len = 0;
static unsigned char g_digest_stream_active = 0;

uint32_t C_DigestInit(unsigned long session, void* mechanism) {
    (void)session;
    (void)mechanism;
    g_digest_stream_len = 0;
    g_digest_stream_active = 1;
    return 0;
}

uint32_t C_DigestUpdate(
    unsigned long session,
    unsigned char* part,
    unsigned long part_len) {
    (void)session;
    if (!g_digest_stream_active || part == 0 || part_len == 0) {
        return 1;
    }
    if (g_digest_stream_len + part_len > SYX_DIGEST_STREAM_CAP) {
        return 1;
    }
    {
        unsigned long i;
        for (i = 0; i < part_len; i++) {
            g_digest_stream[g_digest_stream_len + i] = part[i];
        }
    }
    g_digest_stream_len += part_len;
    return 0;
}

uint32_t C_DigestFinal(
    unsigned long session,
    unsigned char* digest,
    unsigned long* digest_len) {
    (void)session;
    if (!g_digest_stream_active || digest == 0 || digest_len == 0) {
        return 1;
    }
    if (g_digest_stream_len == 0) {
        return 1;
    }
    if (*digest_len < g_digest_stream_len + 1) {
        *digest_len = g_digest_stream_len + 1;
        return 1;
    }
    digest[0] = 0xD1;
    {
        unsigned long i;
        for (i = 0; i < g_digest_stream_len; i++) {
            digest[i + 1] = g_digest_stream[i];
        }
    }
    *digest_len = g_digest_stream_len + 1;
    g_digest_stream_len = 0;
    g_digest_stream_active = 0;
    return 0;
}

/* Phase 128: C_GetAttributeValue stub returns bound HSM-ref (not hardware GA). */
static const char kHsmRefBound[] = "pkcs11-so-hsm-ref-bound-p128";

uint32_t C_GetAttributeValue(
    unsigned long session,
    unsigned long object,
    void* template_ptr,
    unsigned long count) {
    (void)session;
    (void)object;
    if (template_ptr == 0 || count == 0) {
        return 1;
    }
    /* Simplified attribute template: first entry is (type, pValue, ulValueLen). */
    typedef struct {
        unsigned long type;
        void* value;
        unsigned long value_len;
    } Attr;
    Attr* attrs = (Attr*)template_ptr;
    if (attrs[0].value == 0) {
        attrs[0].value_len = (unsigned long)(sizeof(kHsmRefBound) - 1);
        return 0;
    }
    if (attrs[0].value_len < (unsigned long)(sizeof(kHsmRefBound) - 1)) {
        attrs[0].value_len = (unsigned long)(sizeof(kHsmRefBound) - 1);
        return 1;
    }
    {
        unsigned long i;
        char* out = (char*)attrs[0].value;
        for (i = 0; i < sizeof(kHsmRefBound) - 1; i++) {
            out[i] = kHsmRefBound[i];
        }
        attrs[0].value_len = (unsigned long)(sizeof(kHsmRefBound) - 1);
    }
    return 0;
}

static const char kSlotLabel[] = "pkcs11-so-stub-p124-session-adapter";

const char* syx_pkcs11_slot_label(void) {
    return kSlotLabel;
}
