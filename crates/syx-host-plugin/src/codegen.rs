//! Generate SYX import wrappers from `plugin.json` (SYX 40.0).

use std::path::Path;

use crate::manifest::PluginManifest;

/// Emit a SYX module that forwards to plugin host symbols.
pub fn generate_syx_module(manifest: &PluginManifest) -> String {
    let module_name = manifest
        .hosts
        .first()
        .and_then(|h| h.module.clone())
        .unwrap_or_else(|| {
            let mut s = manifest.id.clone();
            if let Some(c) = s.get_mut(0..1) {
                c.make_ascii_uppercase();
            }
            s
        });
    let mut out = String::new();
    out.push_str(&format!(
        "// Auto-generated from plugin.json id={}\n",
        manifest.id
    ));
    out.push_str(&format!("public module {module_name} {{\n"));
    for host in &manifest.hosts {
        let syx_name = host.syx_name.as_deref().unwrap_or_else(|| {
            host.name
                .strip_prefix("__syx_plugin_")
                .unwrap_or(&host.name)
        });
        let params = if host.params.is_empty() {
            String::new()
        } else {
            host.params.join(", ")
        };
        out.push_str(&format!(
            "    public static func {syx_name}({params}) -> {} {{\n",
            host.return_type
        ));
        out.push_str(&format!("        return {}(", host.name));
        if host.params.is_empty() {
            out.push(')');
        } else {
            let arg_names: Vec<&str> = host
                .params
                .iter()
                .filter_map(|p| p.split(':').next().map(str::trim))
                .collect();
            out.push_str(&arg_names.join(", "));
            out.push(')');
        }
        out.push_str("\n    }\n");
    }
    out.push_str("}\n");
    out
}

/// Write generated SYX to `out_dir/{module}.sy`.
pub fn write_codegen_module(
    manifest: &PluginManifest,
    out_dir: &Path,
) -> Result<std::path::PathBuf, String> {
    let module_name = manifest
        .hosts
        .first()
        .and_then(|h| h.module.clone())
        .unwrap_or_else(|| manifest.id.clone());
    std::fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;
    let path = out_dir.join(format!("{module_name}.sy"));
    std::fs::write(&path, generate_syx_module(manifest)).map_err(|e| e.to_string())?;
    Ok(path)
}
