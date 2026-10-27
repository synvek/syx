//! MIR opcode tags shared across WASM and native runtimes.

pub const CONST_I32: u32 = 0;
pub const CONST_STR: u32 = 1;
pub const CONST_BOOL: u32 = 2;
pub const CONST_NIL: u32 = 3;
pub const LOAD_LOCAL: u32 = 4;
pub const STORE_LOCAL: u32 = 5;
pub const ADD: u32 = 10;
pub const SUB: u32 = 11;
pub const MUL: u32 = 12;
pub const DIV: u32 = 13;
pub const MOD: u32 = 14;
pub const EQ: u32 = 15;
pub const NE: u32 = 16;
pub const LT: u32 = 17;
pub const GT: u32 = 18;
pub const AND: u32 = 19;
pub const OR: u32 = 20;
pub const NOT: u32 = 21;
pub const NEG: u32 = 22;
pub const PRINT: u32 = 30;
pub const POP_BOOL: u32 = 31;
pub const CONCAT_STR: u32 = 32;
pub const TO_STR: u32 = 33;
pub const POP: u32 = 34;
pub const DUP: u32 = 35;
pub const CALL_HOST: u32 = 40;
pub const METHOD_CALL: u32 = 41;
pub const MAKE_ITERATOR: u32 = 42;
pub const ITERATOR_NEXT: u32 = 43;
pub const ARRAY_BUILD: u32 = 44;
pub const ARRAY_GET: u32 = 45;
pub const OBJECT_GET: u32 = 46;
pub const BEGIN_FN: u32 = 50;
pub const GET_RETURN: u32 = 51;
pub const CALL_FN: u32 = 52;
pub const PEEK_IS_NIL: u32 = 53;
pub const AWAIT_POLL: u32 = 54;
pub const ACTOR_CALL: u32 = 55;
pub const CALL_WITNESS: u32 = 56;
pub const DICT_BUILD: u32 = 57;
pub const SET_BUILD: u32 = 58;
pub const OBJECT_SET: u32 = 59;
pub const AWAIT_SUSPEND: u32 = 60;
pub const ACTOR_SEND: u32 = 61;
pub const TASK_SPAWN: u32 = 62;
pub const TASK_JOIN: u32 = 63;
pub const MAKE_CLOSURE: u32 = 64;
pub const CALL_CLOSURE: u32 = 65;
pub const CONST_NUMERIC: u32 = 66;
pub const JUMP_IF_NIL: u32 = 67;
pub const TO_DESCRIPTION: u32 = 68;
pub const TO_DEBUG_DESCRIPTION: u32 = 69;
pub const VALUE_EQ: u32 = 70;
pub const VALUE_NE: u32 = 71;
pub const REF_EQ: u32 = 72;
pub const REF_NE: u32 = 73;
pub const BEGIN_TRY: u32 = 74;
pub const END_TRY: u32 = 75;
pub const CATCH: u32 = 76;
pub const THROW: u32 = 77;
pub const BIND_RESULT: u32 = 84;
pub const RETURN_ERR: u32 = 85;
pub const LE: u32 = 78;
pub const GE: u32 = 79;
pub const LOAD_GLOBAL: u32 = 80;
pub const STORE_GLOBAL: u32 = 81;
pub const SET_FN_INDEX: u32 = 82;
pub const ACTOR_REGISTER: u32 = 83;
/// Phase 39: 128-bit numeric literal (`kind` in `a`, `lo` in `b`, `hi` in exec `c`).
pub const CONST_NUMERIC128: u32 = 86;
pub const SIMD_ADD: u32 = 87;
pub const SIMD_SUB: u32 = 88;
pub const SIMD_MUL: u32 = 89;
pub const SIMD_DIV: u32 = 90;
pub const SIMD_EQ: u32 = 91;
pub const SIMD_LT: u32 = 92;
pub const SIMD_LOAD: u32 = 93;
pub const SIMD_STORE: u32 = 94;
pub const SET_CONST128_HI: u32 = 95;
pub const SET_CONST128_LO: u32 = 96;
pub const ARRAY_SET: u32 = 97;
pub const SIMD_SHUFFLE: u32 = 98;
/// Phase 41: spill top SIMD value to WASM linear memory for guest simd128 pilot.
pub const SIMD_WASM_SPILL: u32 = 99;
/// Phase 41: load SIMD value from WASM linear memory after guest simd128 op.
pub const SIMD_WASM_FILL: u32 = 100;
/// Phase 68: homogeneous array sort via Comparable witness compare loop.
pub const ARRAY_SORTED: u32 = 102;
/// Phase 75: compact unified managed heap after collect.
pub const HEAP_COMPACT: u32 = 105;
/// Phase 108 Track B: simd128 Gt / bitwise And/Or/Xor.
pub const SIMD_GT: u32 = 106;
pub const SIMD_AND: u32 = 107;
pub const SIMD_OR: u32 = 108;
pub const SIMD_XOR: u32 = 109;
/// Phase 109 Track B: simd128 Ne/Le/Ge/Not/Shl.
pub const SIMD_NE: u32 = 110;
pub const SIMD_LE: u32 = 111;
pub const SIMD_GE: u32 = 112;
pub const SIMD_NOT: u32 = 113;
pub const SIMD_SHL: u32 = 114;
/// Phase 110 Track B: simd128 arithmetic right shift guest.
pub const SIMD_SHR: u32 = 115;
/// Phase 111 Track B: simd128 lane-wise min/max + f32 add guest pilot.
pub const SIMD_MIN: u32 = 116;
pub const SIMD_MAX: u32 = 117;
pub const SIMD_F32_ADD: u32 = 118;
/// Phase 112 Track B: simd128 lane-wise `f32x4` sub/mul guest.
pub const SIMD_F32_SUB: u32 = 119;
pub const SIMD_F32_MUL: u32 = 120;
/// Phase 113 Track B: simd128 lane-wise `f32x4` div + `f64x2` add guest.
pub const SIMD_F32_DIV: u32 = 121;
pub const SIMD_F64_ADD: u32 = 122;
pub const SIMD_I16_ADD: u32 = 130;
pub const SIMD_I8_ADD: u32 = 132;
pub const SIMD_I8_SUB: u32 = 133;
pub const SIMD_I8_MUL: u32 = 136;
pub const SIMD_I8_AND: u32 = 137;
pub const SIMD_I8_OR: u32 = 140;
pub const SIMD_I8_XOR: u32 = 141;
pub const SIMD_I8_NOT: u32 = 142;
/// Phase 124: lane-wise `i8x16` min/max guest pilot (SIMD16 lane-min-max MVP).
pub const SIMD_I8_MIN: u32 = 143;
pub const SIMD_I8_MAX: u32 = 144;
/// Phase 130: lane-wise `i8x16` unsigned min guest pilot.
pub const SIMD_I8_UMIN: u32 = 153;
/// Phase 130: lane-wise `i8x16` unsigned max guest pilot.
pub const SIMD_I8_UMAX: u32 = 154;
/// Phase 131: lane-wise `i8x16` abs guest pilot.
pub const SIMD_I8_ABS: u32 = 155;
/// Phase 132: lane-wise `i8x16` neg guest pilot.
pub const SIMD_I8_NEG: u32 = 156;
/// Phase 125: `i8x16` extract/replace lane guest pilot (SIMD16 lane-extract-replace MVP).
pub const SIMD_I8_EXTRACT: u32 = 145;
pub const SIMD_I8_REPLACE: u32 = 146;
pub const SIMD_I8_EQ: u32 = 147;
pub const SIMD_I8_NE: u32 = 157;
/// Phase 134: lane-wise `i8x16` le guest pilot.
pub const SIMD_I8_LE: u32 = 158;
/// Phase 135: lane-wise `i8x16` ge guest pilot.
pub const SIMD_I8_GE: u32 = 159;
/// Phase 136: lane-wise `i8x16` avgr_u guest pilot.
pub const SIMD_I8_AVGR_U: u32 = 160;
/// Phase 137: lane-wise `i8x16` popcnt guest pilot.
pub const SIMD_I8_POPCNT: u32 = 161;
/// Phase 138: MSB bitmask (`i8x16.bitmask`) guest pilot.
pub const SIMD_I8_BITMASK: u32 = 162;
/// Phase 139: all lanes non-zero (`i8x16.all_true`) guest pilot.
pub const SIMD_I8_ALL_TRUE: u32 = 163;
/// Phase 139: any lane non-zero (`v128.any_true`) guest pilot.
pub const SIMD_I8_ANY_TRUE: u32 = 164;
pub const SIMD_I8_LT: u32 = 148;
pub const SIMD_I8_GT: u32 = 149;
/// Phase 127: lane-wise `i8x16` signed saturating add guest pilot.
pub const SIMD_I8_ADD_SAT: u32 = 150;
/// Phase 127: lane-wise `i8x16` signed saturating sub guest pilot.
pub const SIMD_I8_SUB_SAT: u32 = 151;
/// Phase 128: fixed-mode `i8x16` shuffle MVP (mode in `a`: 0=reverse, 1=swap pairs).
pub const SIMD_I8_SHUFFLE: u32 = 152;
pub const SIMD_I8_SHL: u32 = 138;
pub const SIMD_I8_SHR: u32 = 139;
pub const SIMD_I16_SUB: u32 = 134;
pub const SIMD_I16_MUL: u32 = 135;
pub const SIMD_I64_ADD: u32 = 131;
/// Phase 114: lane-wise `f64x2` sub guest pilot.
pub const SIMD_F64_SUB: u32 = 123;
/// Phase 114 guest-v7: push v128 from linear memory (Load GA; no SimdWasmFill).
pub const SIMD_GUEST_PUSH_V128: u32 = 124;
/// Phase 114 guest-v7: pop v128 to linear memory (Store GA; no SimdWasmSpill(0)).
pub const SIMD_GUEST_POP_V128: u32 = 125;
/// Phase 115: lane-wise `f64x2` mul guest pilot.
pub const SIMD_F64_MUL: u32 = 126;
/// Phase 115: lane-wise `f64x2` div guest pilot.
pub const SIMD_F64_DIV: u32 = 127;
/// Scalar bitwise ops (integer `& | ^ ~ << >>`).
pub const BIT_AND: u32 = 165;
pub const BIT_OR: u32 = 166;
pub const BIT_XOR: u32 = 167;
pub const BIT_NOT: u32 = 168;
pub const SHL: u32 = 169;
pub const SHR: u32 = 170;
/// Integer narrowing / numeric conversion (`from` in `a`, `to` in `b`).
pub const NUMERIC_CAST: u32 = 171;
