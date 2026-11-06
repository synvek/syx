//! SYX MIR codec / dual-blob / sidecar / AOT-embed framing.
//! Extracted in S2-I167 (zero semantics; re-exported from crate root).

use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

use crate::{MirFunction, MirInstr, MirModule, MirOp};

pub fn encode_module(module: &MirModule) -> Vec<u8> {
    bincode::serialize(module).expect("MIR module encode failed")
}

pub fn decode_module(bytes: &[u8]) -> SyxResult<MirModule> {
    bincode::deserialize(bytes).map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))
}

/// Whole-module MIR blob inventory stats (B-05 encode/load honesty floor; ≠ codec split).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirModuleBlobStats {
    pub fn_count: usize,
    pub encoded_bytes: usize,
}

/// Report `fn_count` and whole-module `encode_module` byte length.
///
/// Used for the B-05 encode/load honesty floor (W309). Does **not** split codec layers
/// or omit instruction streams.
pub fn mir_module_blob_stats(module: &MirModule) -> MirModuleBlobStats {
    MirModuleBlobStats {
        fn_count: module.functions.len(),
        encoded_bytes: encode_module(module).len(),
    }
}

/// Metadata sidecar view for Medium dual-blob / AOT embed wire.
///
/// Serialized via bincode as the sidecar segment. **Not** a `MirModule` subset wire format.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MirModuleMetadataSidecarView {
    pub fn_names: Vec<String>,
    pub string_literals: Vec<String>,
    pub local_names: Vec<String>,
    pub local_types: Vec<String>,
}

/// Extract sidecar tables (fn names, `ConstStr` literals, debug locals).
///
/// Does **not** mutate the module.
pub fn mir_module_metadata_sidecar_view(module: &MirModule) -> MirModuleMetadataSidecarView {
    let mut fn_names = Vec::with_capacity(module.functions.len());
    let mut string_literals = Vec::new();
    let mut local_names = Vec::new();
    let mut local_types = Vec::new();
    for func in &module.functions {
        fn_names.push(func.name.to_string());
        for instr in &func.instructions {
            if let MirOp::ConstStr(s) = &instr.op {
                string_literals.push(s.clone());
            }
        }
        for block in &func.blocks {
            for instr in &block.instructions {
                if let MirOp::ConstStr(s) = &instr.op {
                    string_literals.push(s.clone());
                }
            }
        }
        for name in &func.local_names {
            local_names.push(name.to_string());
        }
        for ty in &func.local_types {
            local_types.push(ty.to_string());
        }
    }
    MirModuleMetadataSidecarView {
        fn_names,
        string_literals,
        local_names,
        local_types,
    }
}

/// Production Medium real-codec first-slice: encode metadata sidecar bytes.
///
/// Wires the former inventory-only sidecar view into a public encode API (W403).
/// Does **not** change generic [`encode_module`], AOT compact blob, or claim dual-blob
/// production embed / B-05 `[x]`.
pub fn encode_module_metadata_sidecar(module: &MirModule) -> Vec<u8> {
    let view = mir_module_metadata_sidecar_view(module);
    bincode::serialize(&view).expect("MIR metadata sidecar encode failed")
}

/// Production Medium real-codec first-slice: decode metadata sidecar bytes.
///
/// Round-trip partner of [`encode_module_metadata_sidecar`]. Does **not** replace
/// [`decode_module`] / `syx_rt_load_mir_blob` whole-module load.
pub fn decode_module_metadata_sidecar(bytes: &[u8]) -> SyxResult<MirModuleMetadataSidecarView> {
    bincode::deserialize(bytes).map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))
}

/// Production Medium dual-blob package: sidecar view + whole-module execution bytes.
///
/// Framed by [`encode_module_dual_blob_package`]. Inventory wire uses whole-module
/// execution; default AOT embed uses [`encode_module_aot_embed_blob`] (compact execution).
/// Does **not** claim sidecar→RT dual-load / B-05 `[x]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirModuleDualBlobPackage {
    pub sidecar: MirModuleMetadataSidecarView,
    /// Whole-module [`encode_module`] bytes carried as the execution/whole segment.
    pub execution_bytes: Vec<u8>,
}

const DUAL_BLOB_PACKAGE_MAGIC: &[u8; 4] = b"SYXD";
const DUAL_BLOB_PACKAGE_VERSION: u8 = 1;

fn encode_dual_blob_framed(sidecar_bytes: &[u8], execution_bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(
        16 + sidecar_bytes.len().saturating_add(execution_bytes.len()),
    );
    out.extend_from_slice(DUAL_BLOB_PACKAGE_MAGIC);
    out.push(DUAL_BLOB_PACKAGE_VERSION);
    out.extend_from_slice(&[0u8; 3]);
    out.extend_from_slice(&(sidecar_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(&(execution_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(sidecar_bytes);
    out.extend_from_slice(execution_bytes);
    out
}

/// Production Medium dual-blob package encode (sidecar + whole-module execution).
///
/// Wire format: magic `SYXD` + version + reserved(3) + sidecar_len(u32 LE) +
/// execution_len(u32 LE) + sidecar bytes + execution bytes.
/// Inventory/wire package uses whole-module [`encode_module`] as execution.
/// Default AOT embed uses [`encode_module_aot_embed_blob`] (compact-for-aot execution).
pub fn encode_module_dual_blob_package(module: &MirModule) -> Vec<u8> {
    encode_dual_blob_framed(
        &encode_module_metadata_sidecar(module),
        &encode_module(module),
    )
}

/// Production Medium dual-blob package decode.
pub fn decode_module_dual_blob_package(bytes: &[u8]) -> SyxResult<MirModuleDualBlobPackage> {
    let execution = peel_dual_blob_package_execution(bytes)?;
    let sidecar_len = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let sidecar = decode_module_metadata_sidecar(&bytes[16..16 + sidecar_len])?;
    Ok(MirModuleDualBlobPackage {
        sidecar,
        execution_bytes: execution.to_vec(),
    })
}

/// Peel execution segment from a `SYXD` dual-blob / AOT-embed package.
///
/// Returns `Ok(execution_slice)` without decoding MIR. Used by `syx_rt_load_mir_blob`
/// (W406+). Pair with [`peel_dual_blob_package_sidecar`] for dual-load runtime (W407).
pub fn peel_dual_blob_package_execution(bytes: &[u8]) -> SyxResult<&[u8]> {
    let (sidecar_len, _execution_len, body) = dual_blob_package_segments(bytes)?;
    Ok(&body[sidecar_len..])
}

/// Peel sidecar segment from a `SYXD` dual-blob / AOT-embed package.
///
/// Returns `Ok(sidecar_slice)` without decoding. Used by `syx_rt_load_mir_blob`
/// dual-load runtime first-slice (W407) before [`decode_module_metadata_sidecar`].
pub fn peel_dual_blob_package_sidecar(bytes: &[u8]) -> SyxResult<&[u8]> {
    let (sidecar_len, _execution_len, body) = dual_blob_package_segments(bytes)?;
    Ok(&body[..sidecar_len])
}

fn dual_blob_package_segments(bytes: &[u8]) -> SyxResult<(usize, usize, &[u8])> {
    if bytes.len() < 16 {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "dual-blob package too short",
        ));
    }
    if &bytes[0..4] != DUAL_BLOB_PACKAGE_MAGIC {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "dual-blob package bad magic",
        ));
    }
    if bytes[4] != DUAL_BLOB_PACKAGE_VERSION {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "dual-blob package unsupported version",
        ));
    }
    let sidecar_len = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let execution_len = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    let body = &bytes[16..];
    if body.len() != sidecar_len.saturating_add(execution_len) {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "dual-blob package length mismatch",
        ));
    }
    Ok((sidecar_len, execution_len, body))
}

/// Default AOT embed blob (W406/W409): SYXD framing with sidecar + **omitted** execution.
///
/// W409: execution segment is empty (`execution_len=0`). Sidecar is still encoded from
/// the **original** module. RT loads via sidecar-only (fn stubs + string pool).
/// Does **not** delete `@syx_mir_blob` / claim MIR-free / B-05 `[x]`.
pub fn encode_module_aot_embed_blob(module: &MirModule) -> Vec<u8> {
    encode_dual_blob_framed(&encode_module_metadata_sidecar(module), &[])
}

/// AOT-embed execution bytes helper (W409: always empty — execution segment omitted).
pub fn encode_module_aot_embed_execution_bytes(_module: &MirModule) -> Vec<u8> {
    Vec::new()
}

/// Honesty: default AOT embed omits the execution segment (W409).
pub fn aot_embed_execution_omitted() -> bool {
    true
}

/// Build a minimal MIR module of Return stubs from sidecar `fn_names` (index alignment).
///
/// Used by RT sidecar-only load when AOT embed execution is empty (W409).
pub fn mir_module_from_sidecar_fn_stubs(
    view: &MirModuleMetadataSidecarView,
) -> MirModule {
    MirModule {
        functions: view
            .fn_names
            .iter()
            .map(|name| MirFunction {
                name: SmolStr::new(name.as_str()),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![],
                instructions: vec![MirInstr {
                    dest: None,
                    op: MirOp::Return,
                    span: None,
                }],
                local_names: vec![],
                local_types: vec![],
                init_body_pc: 0,
                init_wrap_pc: 0,
            })
            .collect(),
        ..Default::default()
    }
}

/// True when `bytes` look like a `SYXD` dual-blob / AOT-embed package header.
pub fn is_dual_blob_package_bytes(bytes: &[u8]) -> bool {
    bytes.len() >= 5
        && &bytes[0..4] == DUAL_BLOB_PACKAGE_MAGIC
        && bytes[4] == DUAL_BLOB_PACKAGE_VERSION
}

/// Sidecar segment length helper for dual-blob package assertions.
pub fn dual_blob_package_sidecar_bytes(module: &MirModule) -> Vec<u8> {
    encode_module_metadata_sidecar(module)
}

/// Execution/whole segment helper for dual-blob package assertions.
pub fn dual_blob_package_execution_bytes(module: &MirModule) -> Vec<u8> {
    encode_module(module)
}
