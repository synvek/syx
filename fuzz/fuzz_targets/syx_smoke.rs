//! C-7a / W948 lineage: `syx_smoke` fuzz target.
//! Consumes input via `syx-lexer` (UTF-8 → `scan_all`).
//! ≠ language-guided fuzz / ≠ unbounded fuzz GA / ≠ multi-target suite.

#![no_main]

use libfuzzer_sys::fuzz_target;
use syx_lexer::Lexer;

fuzz_target!(|data: &[u8]| {
    let Ok(s) = std::str::from_utf8(data) else {
        return;
    };
    // Lexer returns SyxResult on lexical errors — do not unwrap/panic on Err.
    let _ = Lexer::from_str("<fuzz>", s).scan_all();
});
