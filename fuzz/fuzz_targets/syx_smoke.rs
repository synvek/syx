//! W948 / ADR-1292: C-09 fuzz scaffold first-slice — minimal `syx_smoke` target.
//! Accepts bytes and never panics. ≠ language-guided fuzz / ≠ CI fuzz / ≠ scaffold GA.

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|_data: &[u8]| {
    // Smoke-only: accept input, no crash. Full compiler/driver fuzz is a later window.
});
