# A-07 Crypto Vectors (Wave A W346 thin-open; W347–W349 RFC/tamper partial deepen; W350 floor; W351 exit)

> **Status:** **A-07 thin floor signed** (ADR-690 / W346) + **W347–W349 RFC/tamper partial deepen** (ADR-691 / ADR-692 / ADR-693) + **W350 Q-S3 floor signed** (ADR-694 / [`gate-a07-qs3-floor.md`](../../plan/archive/gates/gate-a07-qs3-floor.md)；F-10 W581 archived) + **W351 Q-S3 / A-07 exit signed** (ADR-695 / [`gate-a07-qs3-exit.md`](../../plan/archive/gates/gate-a07-qs3-exit.md)；F-10 W580 archived) — inventory indexed; six exact nails landed; floor + exit charters met.  
> Roadmap A-07 is `[x]`; Q-S3 is checked. **Not** all A-* `[x]` (A-08 / A-10 remain open).  
> **W345 cross-index:** A-06 / Q-S2 exit signed ([`gate-a06-qs2-exit.md`](../../plan/archive/gates/gate-a06-qs2-exit.md) / ADR-689；F-10 archived).

## Scope

| Cluster | Status | Evidence (existing nails) |
|---------|--------|---------------------------|
| **ed25519 sign/verify** | **in scope** | `crates/syx-driver/tests/platform/driver_gate.rs` (`p98_driver_ed25519_sign_host`); `p98_driver_parity_crypto_ed25519_exact` + `tests/fixtures/parity/stdlib/crypto_ed25519.sy` |
| **host registry / import surface** | **in scope** | `p98_driver_host_registry_ed25519`; `p98_driver_import_crypto_ed25519` |
| **pbkdf2 smoke** | **in scope** | `crates/syx-driver/tests/platform/crypto_pbkdf2.rs` (c=1) + `gate_a07_rfc_pbkdf2_c2_vector` (c=2; ADR-693) + `packages/internal/capabilities.toml` `crypto.pbkdf2` (**status still experimental**) |
| **package surface** | **in scope** | `packages/crypto/mod.sy` (sha256/512、hmacSha256、aesGcmEncrypt、ed25519Sign/Verify、pbkdf2、randomBytes) |
| **RFC known-answer / tamper negatives** | **partial met** (W347–W349 / ADR-691 / ADR-692 / ADR-693) | `gate_a07_rfc_sha256_empty_vector`; `gate_a07_tamper_ed25519_verify_false`; `gate_a07_rfc_sha512_empty_vector`; `gate_a07_rfc_hmac_sha256_rfc4231_jefe`; `gate_a07_rfc_pbkdf2_c2_vector`; `gate_a07_tamper_ed25519_sig_byte_flip` — **Not** full RFC vector table GA |

## Thin-open + deepen + floor + exit definition

Q-S3 **thin floor** = W346 indexes pre-existing nails（thin meta retired F-10 W581）. W347–W349 add **exact HostFn behavioral nails**. W350 signs the **Q-S3 floor charter**（[`gate-a07-qs3-floor.md`](../../plan/archive/gates/gate-a07-qs3-floor.md)；F-10 W581 archived）. W351 signs **Q-S3 / A-07 exit** ([`gate-a07-qs3-exit.md`](../../plan/archive/gates/gate-a07-qs3-exit.md) / ADR-695；F-10 W580 archived). Exit ≠ full RFC table GA and ≠ pbkdf2 status raise.

**Evidence nails:** `crates/syx-driver/tests/stdlib/crypto_rfc_vectors.rs` + platform nails（driver_gate / crypto_pbkdf2 / packages/crypto） + roadmap A-07 `[x]`（F-10 W580–W581：qs3_exit + thin/floor meta retired）.

## Frozen

- Claiming all A-* rows `[x]` (A-08 / A-10 still open)
- Full RFC vector table GA claim
- `crypto.pbkdf2` experimental→implemented status raise
- Residual `native=partial→real` (listener / pool / row / deque)
- B-04 / B-05 / B-06 `[x]`; Wave B production delivery

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
