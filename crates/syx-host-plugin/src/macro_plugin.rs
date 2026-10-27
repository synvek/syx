//! Phase 53: cdylib JSON RPC macro expansion (ADR-070 M4 + ADR-071).

pub fn macro_plugin_path_from_env() -> Option<std::path::PathBuf> {
    std::env::var("SYX_MACRO_PLUGIN_PATH")
        .ok()
        .map(std::path::PathBuf::from)
}

#[cfg(unix)]
pub fn expand_macro_json(request_json: &str) -> Result<String, String> {
    use crate::registry::register_loaded_library;
    use libloading::{Library, Symbol};

    let path = macro_plugin_path_from_env()
        .ok_or_else(|| "SYX_MACRO_PLUGIN_PATH not set for PluginMacroExpander".to_string())?;
    type ExpandFn = unsafe extern "C" fn(*const std::ffi::c_char) -> *const std::ffi::c_char;
    let lib = unsafe { Library::new(&path).map_err(|e| format!("macro plugin dlopen: {e}"))? };
    let expand: Symbol<ExpandFn> = unsafe {
        lib.get(b"syx_macro_expand")
            .map_err(|e| format!("macro plugin dlsym syx_macro_expand: {e}"))?
    };
    let c_req =
        std::ffi::CString::new(request_json).map_err(|e| format!("macro req cstring: {e}"))?;
    let resp_ptr = unsafe { expand(c_req.as_ptr()) };
    if resp_ptr.is_null() {
        return Err("macro plugin returned null response".into());
    }
    let resp = unsafe { std::ffi::CStr::from_ptr(resp_ptr) }
        .to_string_lossy()
        .into_owned();
    register_loaded_library("syx-macro-plugin".into(), lib);
    Ok(resp)
}

#[cfg(not(unix))]
pub fn expand_macro_json(_request_json: &str) -> Result<String, String> {
    Err("macro plugin dlopen requires Unix".into())
}
