//! Phase 118 Track A: skia-safe paragraph layout + paint (OS-only all APIs + OT-aligned).
//! Fingerprints: `gui-skia-paragraph-v6:os-ot-aligned`, `gui-font-system-v5:os-only-all-apis`
//!
//! Skia's paragraph builder uses HarfBuzz for shaping under the hood; wrap/bidi come
//! from the textlayout module. Labels paint via `Paragraph::paint` OR HB/skia-glyph — never both.

use std::sync::atomic::{AtomicBool, Ordering};

use skia_safe::{
    surfaces,
    textlayout::{
        FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle, TypefaceFontProvider,
    },
    AlphaType, Color, ColorType, FontMgr, ImageInfo, Typeface,
};
use ttf_parser::Face;

use crate::shape;

static LAYOUT_USED: AtomicBool = AtomicBool::new(false);
static BIDI_USED: AtomicBool = AtomicBool::new(false);
static TEXTLAYOUT_API: AtomicBool = AtomicBool::new(false);
static PARAGRAPH_PAINT_USED: AtomicBool = AtomicBool::new(false);
static VERTICAL_PARAGRAPH_USED: AtomicBool = AtomicBool::new(false);
static FONT_MGR_OS: AtomicBool = AtomicBool::new(false);
static FONT_MGR_BUNDLED: AtomicBool = AtomicBool::new(false);

pub const SKIA_LAYOUT_FINGERPRINT: &str = "gui-skia-paragraph-v6:os-ot-aligned";
pub const FONT_SYSTEM_FINGERPRINT: &str = "gui-font-system-v5:os-only-all-apis";

const PRIMARY_TTF: &[u8] = include_bytes!("../assets/cjk_subset.ttf");
/// Bundled stand-in for CI when OS FontMgr lacks the OOB glyph (`龍` absent from cjk_subset).
const SYSTEM_FALLBACK_TTF: &[u8] = include_bytes!("../assets/cjk_fallback_oob.ttf");
const FAMILY: &str = "SYXParagraph";
const PARAGRAPH_INK: Color = Color::from_argb(255, 238, 238, 238);

pub fn skia_layout_fingerprint() -> &'static str {
    SKIA_LAYOUT_FINGERPRINT
}

pub fn font_system_fingerprint() -> &'static str {
    FONT_SYSTEM_FINGERPRINT
}

pub fn layout_used() -> bool {
    LAYOUT_USED.load(Ordering::Relaxed)
}

pub fn bidi_used() -> bool {
    BIDI_USED.load(Ordering::Relaxed)
}

pub fn textlayout_api_used() -> bool {
    TEXTLAYOUT_API.load(Ordering::Relaxed)
}

pub fn paragraph_paint_used() -> bool {
    PARAGRAPH_PAINT_USED.load(Ordering::Relaxed)
}

pub fn vertical_paragraph_used() -> bool {
    VERTICAL_PARAGRAPH_USED.load(Ordering::Relaxed)
}

pub fn mark_vertical_paragraph_used() {
    VERTICAL_PARAGRAPH_USED.store(true, Ordering::Relaxed);
}

pub fn font_mgr_os_used() -> bool {
    FONT_MGR_OS.load(Ordering::Relaxed)
}

pub fn font_mgr_bundled_used() -> bool {
    FONT_MGR_BUNDLED.load(Ordering::Relaxed)
}

/// Legacy doctor alias — true when either OS or bundled FontMgr resolved a missing glyph.
pub fn font_mgr_fallback_used() -> bool {
    font_mgr_os_used() || font_mgr_bundled_used()
}

pub fn reset_skia_layout_stats() {
    LAYOUT_USED.store(false, Ordering::Relaxed);
    BIDI_USED.store(false, Ordering::Relaxed);
    TEXTLAYOUT_API.store(false, Ordering::Relaxed);
    PARAGRAPH_PAINT_USED.store(false, Ordering::Relaxed);
    VERTICAL_PARAGRAPH_USED.store(false, Ordering::Relaxed);
    FONT_MGR_OS.store(false, Ordering::Relaxed);
    FONT_MGR_BUNDLED.store(false, Ordering::Relaxed);
}

fn text_has_bidi(text: &str) -> bool {
    text.chars().any(|ch| {
        let u = ch as u32;
        (0x0590..=0x08FF).contains(&u)
            || (0xFB1D..=0xFDFF).contains(&u)
            || (0xFE70..=0xFEFF).contains(&u)
            || matches!(ch, '\u{200F}' | '\u{202B}' | '\u{202E}')
    })
}

fn paragraph_typeface() -> Option<Typeface> {
    FontMgr::new().new_from_data(PRIMARY_TTF, None)
}

fn system_fallback_typeface() -> Option<Typeface> {
    FontMgr::new().new_from_data(SYSTEM_FALLBACK_TTF, None)
}

fn ttf_has_glyph(ttf: &[u8], ch: char) -> bool {
    Face::parse(ttf, 0)
        .ok()
        .and_then(|face| face.glyph_index(ch))
        .is_some()
}

fn asset_has_glyph(ch: char) -> bool {
    ttf_has_glyph(PRIMARY_TTF, ch)
}

fn system_fallback_has_glyph(ch: char) -> bool {
    ttf_has_glyph(SYSTEM_FALLBACK_TTF, ch)
}

fn text_needs_font_mgr_fallback(text: &str) -> bool {
    text.chars()
        .any(|ch| !asset_has_glyph(ch) && system_fallback_has_glyph(ch))
}

fn rgba_has_ink(rgba: &[u8]) -> bool {
    rgba.chunks_exact(4).any(|px| px[3] > 0)
}

fn font_collection(include_test: bool) -> FontCollection {
    let mut provider = TypefaceFontProvider::new();
    if let Some(tf) = paragraph_typeface() {
        provider.register_typeface(tf, Some(FAMILY));
    }
    let mut fc = FontCollection::new();
    fc.set_asset_font_manager(Some(provider.into()));
    fc.set_default_font_manager(FontMgr::default(), Some(FAMILY));
    if include_test {
        let mut test_provider = TypefaceFontProvider::new();
        if let Some(tf) = system_fallback_typeface() {
            test_provider.register_typeface(tf, Some("SYXSystemFallback"));
        }
        fc.set_test_font_manager(Some(test_provider.into()));
    }
    fc
}

fn paint_paragraph_with_collection(
    buf: &mut [u32],
    stride: u32,
    x0: u32,
    y0: u32,
    text: &str,
    max_width: f32,
    include_test: bool,
) -> Option<(f32, u32)> {
    let (paragraph, height, lines) = build_paragraph(text, max_width, include_test)?;
    let w = max_width.max(1.0).ceil() as i32;
    let h = height.ceil().max(1.0) as i32;
    let info = ImageInfo::new((w, h), ColorType::RGBA8888, AlphaType::Premul, None);
    let mut surface = surfaces::raster(&info, None, None)?;
    let canvas = surface.canvas();
    canvas.clear(Color::TRANSPARENT);
    paragraph.paint(canvas, (0.0, 0.0));
    PARAGRAPH_PAINT_USED.store(true, Ordering::Relaxed);

    let mut rgba = vec![0u8; (w * h * 4) as usize];
    if surface.read_pixels(&info, &mut rgba, (w * 4) as usize, (0, 0)) {
        blit_rgba_to_buffer(buf, stride, x0, y0, &rgba, w, h);
        Some((height, lines))
    } else {
        None
    }
}

fn mark_font_mgr_resolution(text: &str) {
    if !text_needs_font_mgr_fallback(text) {
        return;
    }
    let mut buf_os = vec![0u32; 64 * 32];
    let os_ok = paint_paragraph_with_collection(&mut buf_os, 64, 0, 0, text, 60.0, false)
        .is_some()
        && rgba_has_ink_from_buffer(&buf_os);
    if os_ok {
        FONT_MGR_OS.store(true, Ordering::Relaxed);
        return;
    }
    let mut buf_bundled = vec![0u32; 64 * 32];
    let bundled_ok = paint_paragraph_with_collection(&mut buf_bundled, 64, 0, 0, text, 60.0, true)
        .is_some()
        && rgba_has_ink_from_buffer(&buf_bundled);
    if bundled_ok {
        FONT_MGR_BUNDLED.store(true, Ordering::Relaxed);
    }
}

fn rgba_has_ink_from_buffer(buf: &[u32]) -> bool {
    buf.iter().any(|px| (*px >> 24) & 0xFF > 0)
}

fn build_paragraph(
    text: &str,
    max_width: f32,
    include_test: bool,
) -> Option<(skia_safe::textlayout::Paragraph, f32, u32)> {
    if text.is_empty() || max_width <= 0.0 {
        return None;
    }

    if text_has_bidi(text) {
        BIDI_USED.store(true, Ordering::Relaxed);
    }

    let font_size = shape::caret_metric_from_font().max(1) as f32;
    let mut para_style = ParagraphStyle::new();
    let mut text_style = TextStyle::new();
    text_style.set_font_size(font_size);
    text_style.set_font_families(&[FAMILY]);
    text_style.set_color(PARAGRAPH_INK);
    para_style.set_text_style(&text_style);

    let mut builder = ParagraphBuilder::new(&para_style, font_collection(include_test));
    builder.add_text(text);
    let mut paragraph = builder.build();
    paragraph.layout(max_width.max(1.0));

    TEXTLAYOUT_API.store(true, Ordering::Relaxed);
    LAYOUT_USED.store(true, Ordering::Relaxed);

    let lines = paragraph.line_number().max(1) as u32;
    let height = paragraph.height().max(font_size);
    Some((paragraph, height, lines))
}

/// Paragraph via skia-safe `textlayout::ParagraphBuilder` (line break + bidi + HB shaping).
pub fn layout_paragraph(text: &str, max_width: f32) -> Option<(f32, u32)> {
    // Phase 118: OS-only for all paragraph APIs (bundled only via explicit *_for_test / doctor).
    build_paragraph(text, max_width, false).map(|(_, height, lines)| (height, lines))
}

/// Test/doctor helper: layout with bundled test FontMgr (explicit non-GA).
pub fn layout_paragraph_for_test(text: &str, max_width: f32) -> Option<(f32, u32)> {
    build_paragraph(text, max_width, true).map(|(_, height, lines)| (height, lines))
}

fn blit_rgba_to_buffer(
    buf: &mut [u32],
    stride: u32,
    x0: u32,
    y0: u32,
    rgba: &[u8],
    w: i32,
    h: i32,
) {
    for dy in 0..h {
        for dx in 0..w {
            let src = ((dy * w + dx) * 4) as usize;
            if src + 3 >= rgba.len() {
                continue;
            }
            let a = rgba[src + 3];
            if a == 0 {
                continue;
            }
            let x = x0.saturating_add(dx as u32);
            let y = y0.saturating_add(dy as u32);
            if x >= stride {
                continue;
            }
            let idx = (y as usize) * stride as usize + x as usize;
            if idx >= buf.len() {
                continue;
            }
            let r = rgba[src];
            let g = rgba[src + 1];
            let b = rgba[src + 2];
            buf[idx] = (a as u32) << 24 | (b as u32) << 16 | (g as u32) << 8 | (r as u32);
        }
    }
}

/// Paint paragraph text into the CPU buffer via `Paragraph::paint` (proves paint path).
pub fn paint_paragraph(
    buf: &mut [u32],
    stride: u32,
    x0: u32,
    y0: u32,
    text: &str,
    max_width: f32,
) -> Option<(f32, u32)> {
    // Phase 117: OS-only FontMgr GA — bundled test fonts are doctor/CI only.
    let out = paint_paragraph_with_collection(buf, stride, x0, y0, text, max_width, false)?;
    if rgba_has_ink_from_buffer(buf) {
        mark_font_mgr_resolution(text);
    }
    Some(out)
}

/// Doctor probe: OS `FontMgr::default()` resolves OOB glyph without bundled test FontMgr.
pub fn probe_font_mgr_os_paint() -> bool {
    FONT_MGR_OS.store(false, Ordering::Relaxed);
    let mut buf = vec![0u32; 64 * 32];
    let ok = paint_paragraph_with_collection(&mut buf, 64, 0, 0, "龍", 60.0, false).is_some()
        && rgba_has_ink_from_buffer(&buf);
    if ok {
        FONT_MGR_OS.store(true, Ordering::Relaxed);
    }
    ok && font_mgr_os_used()
}

/// Doctor probe: bundled test FontMgr resolves OOB glyph (CI auxiliary — not OS GA).
pub fn probe_font_mgr_bundled_paint() -> bool {
    FONT_MGR_BUNDLED.store(false, Ordering::Relaxed);
    let mut os_only = vec![0u32; 64 * 32];
    let os_ok = paint_paragraph_with_collection(&mut os_only, 64, 0, 0, "龍", 60.0, false).is_some()
        && rgba_has_ink_from_buffer(&os_only);
    if os_ok {
        return false;
    }
    let mut buf = vec![0u32; 64 * 32];
    let ok = paint_paragraph_with_collection(&mut buf, 64, 0, 0, "龍", 60.0, true).is_some()
        && rgba_has_ink_from_buffer(&buf);
    if ok {
        FONT_MGR_BUNDLED.store(true, Ordering::Relaxed);
    }
    ok && font_mgr_bundled_used()
}

/// Legacy doctor alias.
pub fn probe_font_mgr_fallback_paint() -> bool {
    probe_font_mgr_os_paint() || probe_font_mgr_bundled_paint()
}
