//! Phase 117: GUI backend report for `syx doctor --gui-backend-report`.
//! Fingerprint: gui-doctor-engine-v5:phase117

#[derive(Debug, Clone, Default)]
pub struct GuiBackendReport {
    pub phase: u32,
    pub backend: String,
    pub widgets: String,
    pub present_mode: String,
    pub gui_dlopen_loaded: bool,
    pub gui_winit_feature: bool,
    pub gui_visible_env: bool,
    pub gui_hold_default: bool,
    pub gui_headless_env: bool,
    pub gui_softbuffer_fallback: bool,
    pub wgpu_present_used: bool,
    pub wgpu_composite_used: bool,
    pub emoji_engine: String,
    pub vertical_rl_layout: bool,
    pub visible_window_count: usize,
    pub sample_center_pixel: i32,
    pub dylib_path: String,
    pub dlopen_ok: bool,
    pub ttf_fingerprint: String,
    pub harfbuzz_fingerprint: String,
    pub font_fallback_fingerprint: String,
    pub ime_preedit_fingerprint: String,
    pub doctor_coverage_fingerprint: String,
    pub wgpu_composite_fingerprint: String,
    pub emoji_fingerprint: String,
    pub vertical_rl_fingerprint: String,
    pub skia_glyph_fingerprint: String,
    pub glyph_coverage_percent: u32,
    pub subset_chars: u32,
    pub shaped: bool,
    pub fallback_hits: u32,
    pub ime_preedit_supported: bool,
    pub ime_commit_supported: bool,
    pub caret_supported: bool,
    pub caret_metric_px: u32,
}

fn doctor_widgets_from_probes(
    loaded: bool,
    egui_text_suppressed: bool,
    no_double_paint: bool,
    paragraph_paint: bool,
    hb_paint_path: bool,
    hb_vertical: bool,
    font_mgr_os: bool,
    font_mgr_bundled: bool,
    composite_used: bool,
    present_used: bool,
    vertical_ink: bool,
    vertical_ascii: bool,
    emoji_hit: bool,
    doctor_engine_tag: &str,
    colr_radial: bool,
    colr_sweep: bool,
    colr_grad: bool,
) -> String {
    if !loaded {
        return String::new();
    }
    let mut parts = Vec::new();
    if egui_text_suppressed {
        parts.push("egui-text-suppressed");
    }
    if no_double_paint {
        parts.push("no-double-paint");
    }
    if paragraph_paint {
        parts.push("paragraph-paint");
    }
    if hb_paint_path {
        parts.push("hb-paint");
    }
    if hb_vertical {
        parts.push("hb-vertical");
    }
    if font_mgr_os {
        parts.push("font-mgr-os");
    }
    if font_mgr_bundled {
        parts.push("font-mgr-bundled");
    }
    if emoji_hit && !doctor_engine_tag.is_empty() {
        parts.push(doctor_engine_tag);
    }
    if colr_radial {
        parts.push("colr-radial");
    }
    if colr_sweep {
        parts.push("colr-sweep");
    }
    if colr_grad && !colr_radial && !colr_sweep {
        parts.push("colr-grad");
    }
    if vertical_ink {
        parts.push("vertical-ink");
    }
    if vertical_ascii {
        parts.push("vertical-ascii-ink");
    }
    if composite_used {
        parts.push("wgpu-composite");
    } else if present_used {
        parts.push("wgpu-present");
    }
    parts.join(":")
}

pub fn run_gui_backend_report() -> GuiBackendReport {
    syx_vm::ensure_gui_loaded();
    let loaded = syx_vm::gui_dlopen_loaded();
    let sample_pixel = syx_vm::gui_sample_center_pixel("winit-SYX Demo");
    let softbuffer_fallback = std::env::var("SYX_GUI_SOFTBUFFER")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let present_mode = if softbuffer_fallback {
        "softbuffer-dev-fallback".to_string()
    } else if loaded {
        syx_vm::gui_doctor_present_mode()
    } else {
        String::new()
    };

    let coverage = if loaded {
        syx_vm::gui_glyph_coverage_percent()
    } else {
        0
    };
    let subset_chars = if loaded {
        syx_vm::gui_cjk_subset_char_count()
    } else {
        0
    };
    let ttf_fp = if loaded {
        syx_vm::gui_ttf_subset_fingerprint()
    } else {
        String::new()
    };
    let preedit_fp = if loaded {
        syx_vm::gui_ime_preedit_fingerprint()
    } else {
        String::new()
    };
    let doctor_fp = if loaded {
        syx_vm::gui_doctor_coverage_fingerprint()
    } else {
        String::new()
    };
    let composite_fp = if loaded {
        syx_vm::gui_wgpu_composite_fingerprint()
    } else {
        String::new()
    };
    let emoji_fp = if loaded {
        syx_vm::gui_emoji_fingerprint()
    } else {
        String::new()
    };
    let vertical_fp = if loaded {
        syx_vm::gui_vertical_rl_fingerprint()
    } else {
        String::new()
    };
    let skia_fp = if loaded {
        syx_vm::gui_skia_glyph_fingerprint()
    } else {
        String::new()
    };
    let (
        shaped,
        fallback_hits,
        hb_fp,
        fb_fp,
        wgpu_used,
        composite_used,
        emoji_engine,
        doctor_engine_tag,
        vrl_layout,
        egui_text_suppressed,
        no_double_paint,
        vertical_ink,
        paragraph_paint,
        hb_paint_path,
        hb_vertical,
        font_mgr_os,
        font_mgr_bundled,
        vertical_ascii,
        colr_radial,
        colr_sweep,
        colr_grad,
    ) = if loaded {
        (
            syx_vm::gui_shaped(),
            syx_vm::gui_fallback_hits(),
            syx_vm::gui_harfbuzz_fingerprint(),
            syx_vm::gui_font_fallback_fingerprint(),
            syx_vm::gui_wgpu_present_used(),
            syx_vm::gui_wgpu_composite_used(),
            syx_vm::gui_emoji_engine(),
            syx_vm::gui_doctor_engine_tag(),
            syx_vm::gui_vertical_rl_layout_used(),
            syx_vm::gui_egui_text_suppressed_used(),
            syx_vm::gui_no_double_paint_probe(),
            syx_vm::gui_vertical_ink_used(),
            syx_vm::gui_paragraph_paint_used(),
            syx_vm::gui_hb_paint_path_used(),
            syx_vm::gui_vertical_hb_vertical_used(),
            syx_vm::gui_font_mgr_os_used(),
            syx_vm::gui_font_mgr_bundled_used(),
            syx_vm::gui_vertical_ascii_ink_used(),
            syx_vm::gui_colr_radial_used(),
            syx_vm::gui_colr_sweep_used(),
            syx_vm::gui_colr_grad_used(),
        )
    } else {
        (
            false,
            0,
            String::new(),
            String::new(),
            false,
            false,
            String::new(),
            String::new(),
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
        )
    };
    let emoji_hit = loaded && syx_vm::gui_emoji_atlas_hit();
    let widgets = if loaded {
        syx_vm::gui_doctor_widgets()
    } else {
        doctor_widgets_from_probes(
            loaded,
            egui_text_suppressed,
            no_double_paint,
            paragraph_paint,
            hb_paint_path,
            hb_vertical,
            font_mgr_os,
            font_mgr_bundled,
            composite_used,
            wgpu_used,
            vertical_ink,
            vertical_ascii,
            emoji_hit,
            &doctor_engine_tag,
            colr_radial,
            colr_sweep,
            colr_grad,
        )
    };

    GuiBackendReport {
        phase: 117,
        backend: syx_vm::gui_backend_name().to_string(),
        widgets,
        present_mode,
        gui_dlopen_loaded: loaded,
        gui_winit_feature: true,
        gui_visible_env: std::env::var("SYX_GUI_VISIBLE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false),
        gui_hold_default: std::env::var("SYX_GUI_HOLD")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false),
        gui_headless_env: std::env::var("SYX_GUI_HEADLESS")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false),
        gui_softbuffer_fallback: softbuffer_fallback,
        wgpu_present_used: wgpu_used,
        wgpu_composite_used: composite_used,
        emoji_engine,
        vertical_rl_layout: vrl_layout,
        visible_window_count: syx_vm::visible_window_count(),
        sample_center_pixel: sample_pixel,
        dylib_path: syx_vm::gui_dylib_path_report(),
        dlopen_ok: loaded,
        ttf_fingerprint: ttf_fp,
        harfbuzz_fingerprint: hb_fp,
        font_fallback_fingerprint: fb_fp,
        ime_preedit_fingerprint: preedit_fp,
        doctor_coverage_fingerprint: doctor_fp,
        wgpu_composite_fingerprint: composite_fp,
        emoji_fingerprint: emoji_fp,
        vertical_rl_fingerprint: vertical_fp,
        skia_glyph_fingerprint: skia_fp,
        glyph_coverage_percent: coverage,
        subset_chars,
        shaped,
        fallback_hits,
        ime_preedit_supported: syx_vm::gui_ime_preedit_supported(),
        ime_commit_supported: syx_vm::gui_ime_commit_supported(),
        caret_supported: syx_vm::gui_caret_supported(),
        caret_metric_px: syx_vm::gui_caret_metric_px(),
    }
}

pub fn print_gui_backend_report(report: &GuiBackendReport) {
    println!("syx doctor --gui-backend-report (Phase {})", report.phase);
    println!("  backend: {}", report.backend);
    println!("  widgets: {}", report.widgets);
    println!("  present mode: {}", report.present_mode);
    if report.gui_softbuffer_fallback {
        println!("  softbuffer: SYX_GUI_SOFTBUFFER=1 (dev fallback, not GA)");
    }
    println!("  wgpu present used: {}", report.wgpu_present_used);
    println!("  wgpu composite used: {}", report.wgpu_composite_used);
    println!("  emoji engine: {}", report.emoji_engine);
    println!("  vertical-rl layout: {}", report.vertical_rl_layout);
    println!(
        "  skia glyph fingerprint: {}",
        report.skia_glyph_fingerprint
    );
    println!(
        "  wgpu composite fingerprint: {}",
        report.wgpu_composite_fingerprint
    );
    println!("  emoji fingerprint: {}", report.emoji_fingerprint);
    println!(
        "  vertical-rl fingerprint: {}",
        report.vertical_rl_fingerprint
    );
    println!("  ttf fingerprint: {}", report.ttf_fingerprint);
    println!("  harfbuzz fingerprint: {}", report.harfbuzz_fingerprint);
    println!(
        "  font fallback fingerprint: {}",
        report.font_fallback_fingerprint
    );
    println!(
        "  ime preedit fingerprint: {}",
        report.ime_preedit_fingerprint
    );
    println!(
        "  doctor coverage fingerprint: {}",
        report.doctor_coverage_fingerprint
    );
    println!("  shaped: {}", report.shaped);
    println!("  fallback_hits: {}", report.fallback_hits);
    println!("  glyph coverage %: {}", report.glyph_coverage_percent);
    println!("  subset chars: {}", report.subset_chars);
    println!("  ime preedit supported: {}", report.ime_preedit_supported);
    println!("  ime commit supported: {}", report.ime_commit_supported);
    println!("  caret supported: {}", report.caret_supported);
    println!("  caret metric px: {}", report.caret_metric_px);
    println!("  gui-dlopen loaded: {}", report.gui_dlopen_loaded);
    println!("  gui-dlopen ok: {}", report.dlopen_ok);
    println!("  gui-dlopen feature: {}", report.gui_winit_feature);
    println!("  SYX_GUI_VISIBLE: {}", report.gui_visible_env);
    println!("  SYX_GUI_HOLD: {}", report.gui_hold_default);
    println!("  SYX_GUI_HEADLESS: {}", report.gui_headless_env);
    println!("  SYX_GUI_SOFTBUFFER: {}", report.gui_softbuffer_fallback);
    println!("  visible windows: {}", report.visible_window_count);
    println!("  sample center pixel: {}", report.sample_center_pixel);
    println!("  dylib path: {}", report.dylib_path);
}
