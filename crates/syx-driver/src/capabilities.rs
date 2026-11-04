//! M0-C04 / ADR-133: `syx doctor --capabilities` report from embedded capability JSON.
//! M1-B04/B05 / ADR-136: `syx doctor --ir-capabilities` MirOp/HostFn tables.

use serde_json::Value;

const CAPABILITIES_JSON: &str =
    include_str!("../../../packages/internal/capabilities.generated.json");
const MIR_OP_CAPABILITIES_JSON: &str =
    include_str!("../../../packages/internal/mir_op_capabilities.generated.json");
const HOST_FN_CAPABILITIES_JSON: &str =
    include_str!("../../../packages/internal/host_fn_capabilities.generated.json");

#[derive(Debug, Clone)]
pub struct CapabilitiesReport {
    pub schema: String,
    pub syx_version: String,
    pub phase: u64,
    pub rows: Vec<CapabilityRow>,
}

#[derive(Debug, Clone)]
pub struct CapabilityRow {
    pub id: String,
    pub feature: String,
    pub status: String,
    pub vm: String,
    pub mir: String,
    pub native: String,
    pub wasm: String,
    pub notes: String,
}

#[derive(Debug, Clone)]
pub struct IrCapabilitiesReport {
    pub mir_schema: String,
    pub host_schema: String,
    pub mir_count: usize,
    pub host_count: usize,
    pub mir_non_unsupported: usize,
    pub host_non_unsupported: usize,
    pub mir_pilot: usize,
    pub host_pilot: usize,
}

pub fn run_capabilities_report() -> CapabilitiesReport {
    let v: Value = serde_json::from_str(CAPABILITIES_JSON).unwrap_or(Value::Null);
    let meta = v.get("meta").cloned().unwrap_or(Value::Null);
    let mut rows = Vec::new();
    if let Some(arr) = v.get("capability").and_then(|c| c.as_array()) {
        for cap in arr {
            let b = cap.get("backends").cloned().unwrap_or(Value::Null);
            rows.push(CapabilityRow {
                id: str_field(cap, "id"),
                feature: str_field(cap, "feature"),
                status: str_field(cap, "status"),
                vm: str_field(&b, "vm"),
                mir: str_field(&b, "mir"),
                native: str_field(&b, "native"),
                wasm: str_field(&b, "wasm"),
                notes: str_field(cap, "notes"),
            });
        }
    }
    CapabilitiesReport {
        schema: str_field(&meta, "schema"),
        syx_version: str_field(&meta, "syx_version"),
        phase: meta.get("phase").and_then(|p| p.as_u64()).unwrap_or(0),
        rows,
    }
}

fn str_field(v: &Value, key: &str) -> String {
    v.get(key)
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string()
}

fn count_table(json: &str, arr_key: &str) -> (String, usize, usize, usize) {
    let v: Value = serde_json::from_str(json).unwrap_or(Value::Null);
    let schema = str_field(v.get("meta").unwrap_or(&Value::Null), "schema");
    let rows = v
        .get(arr_key)
        .and_then(|a| a.as_array())
        .cloned()
        .unwrap_or_default();
    let total = rows.len();
    let mut non_unsup = 0usize;
    let mut pilot = 0usize;
    for r in &rows {
        if str_field(r, "status") != "unsupported" {
            non_unsup += 1;
        }
        if !str_field(r, "pilot").is_empty() {
            pilot += 1;
        }
    }
    (schema, total, non_unsup, pilot)
}

pub fn run_ir_capabilities_report() -> IrCapabilitiesReport {
    let (mir_schema, mir_count, mir_non_unsupported, mir_pilot) =
        count_table(MIR_OP_CAPABILITIES_JSON, "ops");
    let (host_schema, host_count, host_non_unsupported, host_pilot) =
        count_table(HOST_FN_CAPABILITIES_JSON, "hosts");
    IrCapabilitiesReport {
        mir_schema,
        host_schema,
        mir_count,
        host_count,
        mir_non_unsupported,
        host_non_unsupported,
        mir_pilot,
        host_pilot,
    }
}

pub fn print_capabilities_report(report: &CapabilitiesReport) {
    println!(
        "syx doctor --capabilities · {} · SYX {} · phase {}",
        report.schema, report.syx_version, report.phase
    );
    println!(
        "{:<28} {:<26} {:<12} {:<12} {:<12} {:<12}",
        "ID", "STATUS", "VM", "MIR", "NATIVE", "WASM"
    );
    for row in &report.rows {
        println!(
            "{:<28} {:<26} {:<12} {:<12} {:<12} {:<12}",
            truncate(&row.id, 28),
            truncate(&row.status, 26),
            truncate(&row.vm, 12),
            truncate(&row.mir, 12),
            truncate(&row.native, 12),
            truncate(&row.wasm, 12),
        );
    }
    println!("{} capabilities", report.rows.len());
}

pub fn print_ir_capabilities_report(report: &IrCapabilitiesReport) {
    println!("syx doctor --ir-capabilities · ADR-136 / M1-B04/B05");
    println!(
        "MirOp  · {} · rows={} annotated={} pilot={}",
        report.mir_schema, report.mir_count, report.mir_non_unsupported, report.mir_pilot
    );
    println!(
        "HostFn · {} · rows={} annotated={} pilot={}",
        report.host_schema, report.host_count, report.host_non_unsupported, report.host_pilot
    );
    println!(
        "backends=vm,mir,llvm,wasm,cranelift (every row); default=unsupported; \
         see books/shared/mir-op-capabilities.md and host-fn-capabilities.md"
    );
}

fn truncate(s: &str, n: usize) -> String {
    if s.len() <= n {
        s.to_string()
    } else {
        format!("{}…", &s[..n.saturating_sub(1)])
    }
}
