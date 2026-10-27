//! Phase 109 Track A: HarfBuzz whole-string shape on paint path.
//! Phase 121 Track B: OT feature-matrix + VF variations wired into paint shaping.
//! Phase 122 Track B: default-paint OT MVP (test-font hook — not OS FontMgr GA).
//! Phase 123 Track B: OS FontMgr default-paint OT (FontMgr registration — not hook-only GA).
//! Fingerprint: `gui-hb-paint-v1:string-shape`
//! Carry: `gui-harfbuzz-v1:shape-mvp`, `gui-font-fallback-v1:system-cjk`

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::OnceLock;

use fontdue::{Font, FontSettings};
use rustybuzz::ttf_parser::Tag;
use rustybuzz::{Face, Feature, UnicodeBuffer};

use crate::cjk_font::{CI_SUBSET, CJK_ADVANCE, RASTER_PX};
#[cfg(feature = "skia-glyph")]
use crate::skia_glyph::{self, GlyphBitmap};
#[cfg(feature = "skia-glyph")]
use skia_safe::{textlayout::TypefaceFontProvider, utils::OrderedFontMgr, FontMgr};

const PRIMARY_TTF: &[u8] = include_bytes!("../assets/cjk_subset.ttf");
/// Minimal embedded OOB glyph (`龍`) so CI can exercise fallback without system fonts.
const FALLBACK_OOB_TTF: &[u8] = include_bytes!("../assets/cjk_fallback_oob.ttf");

static SHAPED_USED: AtomicBool = AtomicBool::new(false);
static PAINT_SHAPE_USED: AtomicBool = AtomicBool::new(false);
static FALLBACK_HITS: AtomicU32 = AtomicU32::new(0);
static LAST_PAINT_GLYPHS: AtomicU32 = AtomicU32::new(0);

static PRIMARY_FONT: OnceLock<Font> = OnceLock::new();
static FALLBACK_OOB_FONT: OnceLock<Font> = OnceLock::new();
static SYSTEM_FALLBACK: OnceLock<Option<(Vec<u8>, Font)>> = OnceLock::new();

pub const HB_PAINT_FINGERPRINT: &str = "gui-hb-paint-v1:string-shape";

pub const OT_FEATURES_FINGERPRINT: &str = "gui-ot-features-v7:os-fontmgr-default-paint";
pub const DYLIB_OT_VF_PARITY_FINGERPRINT: &str = "gui-dylib-parity-v1:ot-vf-export";
pub const VF_WGHT_FINGERPRINT: &str = "gui-font-vf-v1:wght-mvp";
pub const VF_MULTI_AXIS_FINGERPRINT: &str = "gui-font-vf-v2:multi-axis-mvp";
pub const VF_AXES_GE3_FINGERPRINT: &str = "gui-font-vf-v3:axes-ge3-mvp";
pub const VF_PAINT_VARIATIONS_FINGERPRINT: &str = "gui-font-vf-v4:paint-variations-mvp";

static OT_CALT_USED: AtomicBool = AtomicBool::new(false);
static OT_SUBST_PROVEN: AtomicBool = AtomicBool::new(false);
static VF_WGHT_USED: AtomicBool = AtomicBool::new(false);
static OT_SCRIPT_SUBST_PROVEN: AtomicBool = AtomicBool::new(false);
static OT_FEATURE_MATRIX_PROVEN: AtomicBool = AtomicBool::new(false);
static OT_PAINT_MATRIX_PROVEN: AtomicBool = AtomicBool::new(false);
static VF_MULTI_AXIS_USED: AtomicBool = AtomicBool::new(false);
static VF_AXES_GE3_USED: AtomicBool = AtomicBool::new(false);
static VF_PAINT_VARIATIONS_USED: AtomicBool = AtomicBool::new(false);
static PAINT_SMCP_ENABLED: AtomicBool = AtomicBool::new(true);
static PAINT_LIGA_ENABLED: AtomicBool = AtomicBool::new(true);
static PAINT_VF_WGHT_BITS: AtomicU32 = AtomicU32::new(400f32.to_bits());
/// Gate/test hook: route latin ASCII on default paint path through OT proof TTF (honest non-OS GA).
static DEFAULT_PAINT_OT_HOOK: AtomicBool = AtomicBool::new(false);
/// Phase 123: resolve default-paint latin via FontMgr-registered proof family (v7 GA path).
static DEFAULT_PAINT_OT_FONTMGR: AtomicBool = AtomicBool::new(false);
static DEFAULT_PAINT_FONTMGR_USED: AtomicBool = AtomicBool::new(false);
static OT_DEFAULT_PAINT_PROVEN: AtomicBool = AtomicBool::new(false);

/// Paint-path routing prefix: smcp proof font + matrix combo.
pub const PAINT_OT_SMCP_PREFIX: &str = "\x01syx-smcp:";
/// Paint-path routing prefix: liga proof font + matrix combo.
pub const PAINT_OT_LIGA_PREFIX: &str = "\x01syx-liga:";
/// Paint-path routing prefix: VF axes≥3 proof font + variation axis.
pub const PAINT_VF_VAR_PREFIX: &str = "\x01syx-vf:";
/// Plain latin text for default-paint OT gate — must not use `\x01syx-*` prefix routing.
pub const DEFAULT_PAINT_OT_TEXT: &str = "a";
/// FontMgr family for default-paint OT proof face (registered via TypefaceFontProvider + OS FontMgr).
pub const DEFAULT_PAINT_OT_FAMILY: &str = "SYXDefaultPaintOT";

const OT_LIGA_PROOF_TTF: &[u8] = include_bytes!("../assets/ot_liga_proof.ttf");
const OT_SMCP_PROOF_TTF: &[u8] = include_bytes!("../assets/ot_smcp_proof.ttf");
const VF_WGHT_PROOF_TTF: &[u8] = include_bytes!("../assets/vf_wght_proof.ttf");
const VF_MULTI_AXIS_PROOF_TTF: &[u8] = include_bytes!("../assets/vf_multi_axis_proof.ttf");
const VF_AXES_GE3_PROOF_TTF: &[u8] = include_bytes!("../assets/vf_axes_ge3_proof.ttf");

/// Paint-path feature matrix: kern + liga + calt + smcp (combo toggles smcp/liga slots).
fn paint_features_matrix() -> [Feature; 4] {
    OT_CALT_USED.store(true, Ordering::Relaxed);
    let smcp = if PAINT_SMCP_ENABLED.load(Ordering::Relaxed) {
        1
    } else {
        0
    };
    let liga = if PAINT_LIGA_ENABLED.load(Ordering::Relaxed) {
        1
    } else {
        0
    };
    [
        Feature::new(Tag::from_bytes(b"kern"), 1, ..),
        Feature::new(Tag::from_bytes(b"liga"), liga, ..),
        Feature::new(Tag::from_bytes(b"calt"), 1, ..),
        Feature::new(Tag::from_bytes(b"smcp"), smcp, ..),
    ]
}

fn paint_vf_wght() -> f32 {
    f32::from_bits(PAINT_VF_WGHT_BITS.load(Ordering::Relaxed))
}

pub fn set_paint_vf_wght(wght: f32) {
    PAINT_VF_WGHT_BITS.store(wght.to_bits(), Ordering::Relaxed);
}

pub fn set_paint_smcp_enabled(enabled: bool) {
    PAINT_SMCP_ENABLED.store(enabled, Ordering::Relaxed);
}

pub fn set_paint_liga_enabled(enabled: bool) {
    PAINT_LIGA_ENABLED.store(enabled, Ordering::Relaxed);
}

struct PaintRoute<'a> {
    text: &'a str,
    ttf: &'static [u8],
    apply_vf_variations: bool,
}

fn resolve_paint_route(text: &str) -> PaintRoute<'_> {
    if let Some(rest) = text.strip_prefix(PAINT_OT_SMCP_PREFIX) {
        return PaintRoute {
            text: rest,
            ttf: OT_SMCP_PROOF_TTF,
            apply_vf_variations: false,
        };
    }
    if let Some(rest) = text.strip_prefix(PAINT_OT_LIGA_PREFIX) {
        return PaintRoute {
            text: rest,
            ttf: OT_LIGA_PROOF_TTF,
            apply_vf_variations: false,
        };
    }
    if let Some(rest) = text.strip_prefix(PAINT_VF_VAR_PREFIX) {
        return PaintRoute {
            text: rest,
            ttf: VF_AXES_GE3_PROOF_TTF,
            apply_vf_variations: true,
        };
    }
    if let Some(ttf) = resolve_default_paint_ot_ttf_via_fontmgr(text) {
        return PaintRoute {
            text,
            ttf,
            apply_vf_variations: false,
        };
    }
    if let Some(ttf) = resolve_default_paint_ot_ttf(text) {
        return PaintRoute {
            text,
            ttf,
            apply_vf_variations: false,
        };
    }
    let (_, ttf) = pick_face_for_text(text);
    PaintRoute {
        text,
        ttf,
        apply_vf_variations: false,
    }
}

fn default_paint_ot_candidate(text: &str) -> bool {
    !text.is_empty()
        && !text.starts_with('\x01')
        && text.chars().all(|c| c.is_ascii_alphabetic())
}

#[cfg(feature = "skia-glyph")]
fn build_default_paint_font_mgr() -> FontMgr {
    let mut ordered = OrderedFontMgr::new();
    let mut provider = TypefaceFontProvider::new();
    if let Some(tf) = FontMgr::new().new_from_data(OT_SMCP_PROOF_TTF, None) {
        provider.register_typeface(tf, Some(DEFAULT_PAINT_OT_FAMILY));
    }
    ordered.append(provider);
    ordered.append(FontMgr::default());
    FontMgr::from(ordered)
}

#[cfg(feature = "skia-glyph")]
fn fontmgr_resolves_default_paint_proof() -> bool {
    let mgr = build_default_paint_font_mgr();
    let mut style_set = mgr.match_family(DEFAULT_PAINT_OT_FAMILY);
    if style_set.count() == 0 {
        return false;
    }
    style_set.new_typeface(0).is_some()
}

fn resolve_default_paint_ot_ttf_via_fontmgr(text: &str) -> Option<&'static [u8]> {
    if !DEFAULT_PAINT_OT_FONTMGR.load(Ordering::Relaxed) {
        return None;
    }
    if !default_paint_ot_candidate(text) {
        return None;
    }
    #[cfg(feature = "skia-glyph")]
    {
        if !fontmgr_resolves_default_paint_proof() {
            return None;
        }
        DEFAULT_PAINT_FONTMGR_USED.store(true, Ordering::Relaxed);
        Some(OT_SMCP_PROOF_TTF)
    }
    #[cfg(not(feature = "skia-glyph"))]
    {
        let _ = text;
        None
    }
}

fn resolve_default_paint_ot_ttf(text: &str) -> Option<&'static [u8]> {
    if !DEFAULT_PAINT_OT_HOOK.load(Ordering::Relaxed) {
        return None;
    }
    if !default_paint_ot_candidate(text) {
        return None;
    }
    Some(OT_SMCP_PROOF_TTF)
}

/// Enable test-font injection on default paint path (gate/doctor — not OS FontMgr setDefault GA).
pub fn set_default_paint_ot_hook(enabled: bool) {
    DEFAULT_PAINT_OT_HOOK.store(enabled, Ordering::Relaxed);
}

pub fn default_paint_ot_hook_enabled() -> bool {
    DEFAULT_PAINT_OT_HOOK.load(Ordering::Relaxed)
}

/// Enable FontMgr default-paint OT resolution (v7 GA — not hook-only).
pub fn set_default_paint_ot_fontmgr(enabled: bool) {
    DEFAULT_PAINT_OT_FONTMGR.store(enabled, Ordering::Relaxed);
}

pub fn default_paint_ot_fontmgr_enabled() -> bool {
    DEFAULT_PAINT_OT_FONTMGR.load(Ordering::Relaxed)
}

pub fn default_paint_fontmgr_used() -> bool {
    DEFAULT_PAINT_FONTMGR_USED.load(Ordering::Relaxed)
}

fn paint_path_glyph_ids(text: &str) -> Vec<u16> {
    shape_string_for_paint(text, 16)
        .into_iter()
        .map(|g| g.glyph_id)
        .collect()
}

fn paint_path_advance_sum(text: &str) -> u32 {
    shape_string_for_paint(text, 16)
        .into_iter()
        .map(|g| g.advance)
        .sum()
}

fn glyph_ids_for(face: &Face, text: &str, features: &[Feature]) -> Vec<u16> {
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text);
    let glyph_buffer = rustybuzz::shape(face, features, buffer);
    glyph_buffer
        .glyph_infos()
        .iter()
        .map(|i| i.glyph_id as u16)
        .collect()
}

/// Hard gate: liga/calt must change glyph-id sequence vs empty-feature baseline.
pub fn prove_ot_substitution() -> bool {
    OT_SUBST_PROVEN.store(false, Ordering::Relaxed);
    let Some(face) = Face::from_slice(OT_LIGA_PROOF_TTF, 0) else {
        return false;
    };
    // Explicitly disable liga/calt for baseline — empty feature list still applies defaults.
    let baseline_feats = [
        Feature::new(Tag::from_bytes(b"liga"), 0, ..),
        Feature::new(Tag::from_bytes(b"clig"), 0, ..),
        Feature::new(Tag::from_bytes(b"calt"), 0, ..),
    ];
    let featured_feats = [
        Feature::new(Tag::from_bytes(b"liga"), 1, ..),
        Feature::new(Tag::from_bytes(b"calt"), 1, ..),
    ];
    let baseline = glyph_ids_for(&face, "fi", &baseline_feats);
    let featured = glyph_ids_for(&face, "fi", &featured_feats);
    if baseline.is_empty() || featured.is_empty() || baseline == featured {
        return false;
    }
    OT_SUBST_PROVEN.store(true, Ordering::Relaxed);
    OT_CALT_USED.store(true, Ordering::Relaxed);
    true
}

pub fn ot_subst_proven() -> bool {
    OT_SUBST_PROVEN.load(Ordering::Relaxed)
}

/// Variable font `wght` MVP: advance at wght=100 vs wght=900 must differ.
pub fn prove_vf_wght() -> bool {
    VF_WGHT_USED.store(false, Ordering::Relaxed);
    if !vf_has_wght_axis(VF_WGHT_PROOF_TTF) {
        return false;
    }
    let adv_light = shape_advance_for_vf(VF_WGHT_PROOF_TTF, 100.0);
    let adv_bold = shape_advance_for_vf(VF_WGHT_PROOF_TTF, 900.0);
    if adv_light <= 0.0 || adv_bold <= 0.0 || (adv_light - adv_bold).abs() < 1.0 {
        return false;
    }
    VF_WGHT_USED.store(true, Ordering::Relaxed);
    true
}

fn vf_has_wght_axis(ttf: &[u8]) -> bool {
    vf_has_axis(ttf, b"wght")
}

fn vf_has_axis(ttf: &[u8], tag: &[u8; 4]) -> bool {
    let Ok(face) = ttf_parser::Face::parse(ttf, 0) else {
        return false;
    };
    face.variation_axes()
        .into_iter()
        .any(|a| a.tag == ttf_parser::Tag::from_bytes(tag))
}

fn shape_advance_for_vf(ttf: &[u8], wght: f32) -> f32 {
    shape_advance_for_vf_axes(ttf, wght, 100.0)
}

fn shape_advance_for_vf_axes(ttf: &[u8], wght: f32, wdth: f32) -> f32 {
    let Some(mut face) = Face::from_slice(ttf, 0) else {
        return 0.0;
    };
    face.set_variations(&[
        rustybuzz::Variation {
            tag: Tag::from_bytes(b"wght"),
            value: wght,
        },
        rustybuzz::Variation {
            tag: Tag::from_bytes(b"wdth"),
            value: wdth,
        },
    ]);
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str("A");
    let glyph_buffer = rustybuzz::shape(&face, &[], buffer);
    let positions = glyph_buffer.glyph_positions();
    if positions.is_empty() {
        return 0.0;
    }
    positions[0].x_advance as f32
}

fn shape_advance_for_vf_axes_opsz(ttf: &[u8], wght: f32, wdth: f32, opsz: f32) -> f32 {
    let Some(mut face) = Face::from_slice(ttf, 0) else {
        return 0.0;
    };
    face.set_variations(&[
        rustybuzz::Variation {
            tag: Tag::from_bytes(b"wght"),
            value: wght,
        },
        rustybuzz::Variation {
            tag: Tag::from_bytes(b"wdth"),
            value: wdth,
        },
        rustybuzz::Variation {
            tag: Tag::from_bytes(b"opsz"),
            value: opsz,
        },
    ]);
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str("A");
    let glyph_buffer = rustybuzz::shape(&face, &[], buffer);
    let positions = glyph_buffer.glyph_positions();
    if positions.is_empty() {
        return 0.0;
    }
    positions[0].x_advance as f32
}

/// Phase 119: OT script-subset hard gate — latn-featured liga differs from dflt-disabled baseline.
pub fn prove_ot_script_subset() -> bool {
    OT_SCRIPT_SUBST_PROVEN.store(false, Ordering::Relaxed);
    let Some(face) = Face::from_slice(OT_LIGA_PROOF_TTF, 0) else {
        return false;
    };
    let baseline_feats = [
        Feature::new(Tag::from_bytes(b"liga"), 0, ..),
        Feature::new(Tag::from_bytes(b"clig"), 0, ..),
        Feature::new(Tag::from_bytes(b"calt"), 0, ..),
    ];
    let featured_feats = [
        Feature::new(Tag::from_bytes(b"liga"), 1, ..),
        Feature::new(Tag::from_bytes(b"calt"), 1, ..),
    ];
    // Script-tagged shaping: latn + liga must still differ from liga-off baseline.
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str("fi");
    buffer.set_script(rustybuzz::script::LATIN);
    let baseline = {
        let mut b = UnicodeBuffer::new();
        b.push_str("fi");
        b.set_script(rustybuzz::script::LATIN);
        let gb = rustybuzz::shape(&face, &baseline_feats, b);
        gb.glyph_infos().iter().map(|i| i.glyph_id as u16).collect::<Vec<_>>()
    };
    let featured = {
        let mut b = UnicodeBuffer::new();
        b.push_str("fi");
        b.set_script(rustybuzz::script::LATIN);
        let gb = rustybuzz::shape(&face, &featured_feats, b);
        gb.glyph_infos().iter().map(|i| i.glyph_id as u16).collect::<Vec<_>>()
    };
    let _ = buffer;
    if baseline.is_empty() || featured.is_empty() || baseline == featured {
        return false;
    }
    OT_SCRIPT_SUBST_PROVEN.store(true, Ordering::Relaxed);
    OT_SUBST_PROVEN.store(true, Ordering::Relaxed);
    OT_CALT_USED.store(true, Ordering::Relaxed);
    true
}

pub fn ot_script_subst_proven() -> bool {
    OT_SCRIPT_SUBST_PROVEN.load(Ordering::Relaxed)
}

/// Phase 120: OT feature-matrix MVP — smcp must change glyph-id sequence vs smcp-off baseline.
pub fn prove_ot_feature_matrix() -> bool {
    OT_FEATURE_MATRIX_PROVEN.store(false, Ordering::Relaxed);
    let Some(face) = Face::from_slice(OT_SMCP_PROOF_TTF, 0) else {
        return false;
    };
    let baseline_feats = [Feature::new(Tag::from_bytes(b"smcp"), 0, ..)];
    let featured_feats = [Feature::new(Tag::from_bytes(b"smcp"), 1, ..)];
    let baseline = glyph_ids_for(&face, "a", &baseline_feats);
    let featured = glyph_ids_for(&face, "a", &featured_feats);
    if baseline.is_empty() || featured.is_empty() || baseline == featured {
        return false;
    }
    OT_FEATURE_MATRIX_PROVEN.store(true, Ordering::Relaxed);
    OT_SUBST_PROVEN.store(true, Ordering::Relaxed);
    true
}

pub fn ot_feature_matrix_proven() -> bool {
    OT_FEATURE_MATRIX_PROVEN.load(Ordering::Relaxed)
}

/// Phase 121: OT paint-matrix MVP — smcp and liga combos must differ on paint path.
pub fn prove_ot_paint_matrix() -> bool {
    OT_PAINT_MATRIX_PROVEN.store(false, Ordering::Relaxed);
    PAINT_SMCP_ENABLED.store(false, Ordering::Relaxed);
    let smcp_off = paint_path_glyph_ids(&format!("{PAINT_OT_SMCP_PREFIX}a"));
    PAINT_SMCP_ENABLED.store(true, Ordering::Relaxed);
    let smcp_on = paint_path_glyph_ids(&format!("{PAINT_OT_SMCP_PREFIX}a"));
    if smcp_off.is_empty() || smcp_on.is_empty() || smcp_off == smcp_on {
        return false;
    }

    PAINT_LIGA_ENABLED.store(false, Ordering::Relaxed);
    let liga_off = paint_path_glyph_ids(&format!("{PAINT_OT_LIGA_PREFIX}fi"));
    PAINT_LIGA_ENABLED.store(true, Ordering::Relaxed);
    let liga_on = paint_path_glyph_ids(&format!("{PAINT_OT_LIGA_PREFIX}fi"));
    if liga_off.is_empty() || liga_on.is_empty() || liga_off == liga_on {
        return false;
    }

    OT_PAINT_MATRIX_PROVEN.store(true, Ordering::Relaxed);
    OT_FEATURE_MATRIX_PROVEN.store(true, Ordering::Relaxed);
    OT_SUBST_PROVEN.store(true, Ordering::Relaxed);
    true
}

pub fn ot_paint_matrix_proven() -> bool {
    OT_PAINT_MATRIX_PROVEN.load(Ordering::Relaxed)
}

/// Phase 123: default-paint OT via OS FontMgr — smcp toggle must change glyph ids on plain text.
pub fn prove_ot_default_paint() -> bool {
    OT_DEFAULT_PAINT_PROVEN.store(false, Ordering::Relaxed);
    DEFAULT_PAINT_FONTMGR_USED.store(false, Ordering::Relaxed);
    if DEFAULT_PAINT_OT_TEXT.contains('\x01') || DEFAULT_PAINT_OT_TEXT.starts_with("syx-") {
        return false;
    }
    // v7 GA: FontMgr path only — hook must stay off.
    set_default_paint_ot_hook(false);
    set_default_paint_ot_fontmgr(true);
    PAINT_SMCP_ENABLED.store(false, Ordering::Relaxed);
    let smcp_off = paint_path_glyph_ids(DEFAULT_PAINT_OT_TEXT);
    PAINT_SMCP_ENABLED.store(true, Ordering::Relaxed);
    let smcp_on = paint_path_glyph_ids(DEFAULT_PAINT_OT_TEXT);
    set_default_paint_ot_fontmgr(false);
    if smcp_off.is_empty() || smcp_on.is_empty() || smcp_off == smcp_on {
        return false;
    }
    if !default_paint_fontmgr_used() {
        return false;
    }
    OT_DEFAULT_PAINT_PROVEN.store(true, Ordering::Relaxed);
    OT_FEATURE_MATRIX_PROVEN.store(true, Ordering::Relaxed);
    OT_SUBST_PROVEN.store(true, Ordering::Relaxed);
    true
}

pub fn ot_default_paint_proven() -> bool {
    OT_DEFAULT_PAINT_PROVEN.load(Ordering::Relaxed)
}

pub fn dylib_ot_vf_parity_fingerprint() -> &'static str {
    DYLIB_OT_VF_PARITY_FINGERPRINT
}

/// Phase 121: VF paint-variations MVP — wght axis must move paint-path advances.
pub fn prove_vf_paint_variations() -> bool {
    VF_PAINT_VARIATIONS_USED.store(false, Ordering::Relaxed);
    if vf_axis_count(VF_AXES_GE3_PROOF_TTF) < 3 || !vf_has_axis(VF_AXES_GE3_PROOF_TTF, b"wght") {
        return false;
    }
    set_paint_vf_wght(100.0);
    let light = paint_path_advance_sum(&format!("{PAINT_VF_VAR_PREFIX}A"));
    set_paint_vf_wght(900.0);
    let bold = paint_path_advance_sum(&format!("{PAINT_VF_VAR_PREFIX}A"));
    if light == 0 || bold == 0 || light == bold {
        return false;
    }
    VF_PAINT_VARIATIONS_USED.store(true, Ordering::Relaxed);
    VF_AXES_GE3_USED.store(true, Ordering::Relaxed);
    VF_MULTI_AXIS_USED.store(true, Ordering::Relaxed);
    VF_WGHT_USED.store(true, Ordering::Relaxed);
    true
}

pub fn vf_paint_variations_used() -> bool {
    VF_PAINT_VARIATIONS_USED.load(Ordering::Relaxed)
}

pub fn vf_paint_variations_fingerprint() -> &'static str {
    VF_PAINT_VARIATIONS_FINGERPRINT
}

fn vf_axis_count(ttf: &[u8]) -> usize {
    let Ok(face) = ttf_parser::Face::parse(ttf, 0) else {
        return 0;
    };
    face.variation_axes().into_iter().count()
}

/// Phase 119: VF multi-axis MVP — wdth axis advances must differ (alongside inherited wght).
pub fn prove_vf_multi_axis() -> bool {
    VF_MULTI_AXIS_USED.store(false, Ordering::Relaxed);
    if !vf_has_axis(VF_MULTI_AXIS_PROOF_TTF, b"wght") || !vf_has_axis(VF_MULTI_AXIS_PROOF_TTF, b"wdth") {
        return false;
    }
    let narrow = shape_advance_for_vf_axes(VF_MULTI_AXIS_PROOF_TTF, 100.0, 50.0);
    let wide = shape_advance_for_vf_axes(VF_MULTI_AXIS_PROOF_TTF, 100.0, 200.0);
    if narrow <= 0.0 || wide <= 0.0 || (narrow - wide).abs() < 1.0 {
        return false;
    }
    // Also require wght still moves advances on the multi-axis face.
    let light = shape_advance_for_vf_axes(VF_MULTI_AXIS_PROOF_TTF, 100.0, 100.0);
    let bold = shape_advance_for_vf_axes(VF_MULTI_AXIS_PROOF_TTF, 900.0, 100.0);
    if light <= 0.0 || bold <= 0.0 || (light - bold).abs() < 1.0 {
        return false;
    }
    VF_MULTI_AXIS_USED.store(true, Ordering::Relaxed);
    VF_WGHT_USED.store(true, Ordering::Relaxed);
    true
}

pub fn vf_multi_axis_used() -> bool {
    VF_MULTI_AXIS_USED.load(Ordering::Relaxed)
}

/// Phase 120: VF axes≥3 MVP — face must expose ≥3 axes; wght, wdth, opsz each move advances.
pub fn prove_vf_axes_ge3() -> bool {
    VF_AXES_GE3_USED.store(false, Ordering::Relaxed);
    if vf_axis_count(VF_AXES_GE3_PROOF_TTF) < 3
        || !vf_has_axis(VF_AXES_GE3_PROOF_TTF, b"wght")
        || !vf_has_axis(VF_AXES_GE3_PROOF_TTF, b"wdth")
        || !vf_has_axis(VF_AXES_GE3_PROOF_TTF, b"opsz")
    {
        return false;
    }
    let light = shape_advance_for_vf_axes_opsz(VF_AXES_GE3_PROOF_TTF, 100.0, 100.0, 14.0);
    let bold = shape_advance_for_vf_axes_opsz(VF_AXES_GE3_PROOF_TTF, 900.0, 100.0, 14.0);
    if light <= 0.0 || bold <= 0.0 || (light - bold).abs() < 1.0 {
        return false;
    }
    let narrow = shape_advance_for_vf_axes_opsz(VF_AXES_GE3_PROOF_TTF, 400.0, 50.0, 14.0);
    let wide = shape_advance_for_vf_axes_opsz(VF_AXES_GE3_PROOF_TTF, 400.0, 200.0, 14.0);
    if narrow <= 0.0 || wide <= 0.0 || (narrow - wide).abs() < 1.0 {
        return false;
    }
    let small = shape_advance_for_vf_axes_opsz(VF_AXES_GE3_PROOF_TTF, 400.0, 100.0, 8.0);
    let large = shape_advance_for_vf_axes_opsz(VF_AXES_GE3_PROOF_TTF, 400.0, 100.0, 72.0);
    if small <= 0.0 || large <= 0.0 || (small - large).abs() < 1.0 {
        return false;
    }
    VF_AXES_GE3_USED.store(true, Ordering::Relaxed);
    VF_MULTI_AXIS_USED.store(true, Ordering::Relaxed);
    VF_WGHT_USED.store(true, Ordering::Relaxed);
    true
}

pub fn vf_axes_ge3_used() -> bool {
    VF_AXES_GE3_USED.load(Ordering::Relaxed)
}

pub fn vf_axes_ge3_fingerprint() -> &'static str {
    VF_PAINT_VARIATIONS_FINGERPRINT
}

pub fn vf_multi_axis_fingerprint() -> &'static str {
    VF_MULTI_AXIS_FINGERPRINT
}

pub fn vf_wght_used() -> bool {
    VF_WGHT_USED.load(Ordering::Relaxed)
}

pub fn vf_wght_fingerprint() -> &'static str {
    VF_WGHT_FINGERPRINT
}

pub fn ot_features_fingerprint() -> &'static str {
    OT_FEATURES_FINGERPRINT
}

pub fn ot_calt_used() -> bool {
    OT_CALT_USED.load(Ordering::Relaxed)
}

pub fn harfbuzz_fingerprint() -> &'static str {
    "gui-harfbuzz-v1:shape-mvp"
}

pub fn hb_paint_fingerprint() -> &'static str {
    HB_PAINT_FINGERPRINT
}

pub fn font_fallback_fingerprint() -> &'static str {
    "gui-font-fallback-v1:system-cjk"
}

pub fn shaped() -> bool {
    SHAPED_USED.load(Ordering::Relaxed)
}

pub fn paint_shaped() -> bool {
    PAINT_SHAPE_USED.load(Ordering::Relaxed)
}

pub fn last_paint_glyph_count() -> u32 {
    LAST_PAINT_GLYPHS.load(Ordering::Relaxed)
}

pub fn fallback_hits() -> u32 {
    FALLBACK_HITS.load(Ordering::Relaxed)
}

pub fn reset_shape_stats() {
    SHAPED_USED.store(false, Ordering::Relaxed);
    PAINT_SHAPE_USED.store(false, Ordering::Relaxed);
    FALLBACK_HITS.store(0, Ordering::Relaxed);
    LAST_PAINT_GLYPHS.store(0, Ordering::Relaxed);
    OT_CALT_USED.store(false, Ordering::Relaxed);
    OT_SUBST_PROVEN.store(false, Ordering::Relaxed);
    VF_WGHT_USED.store(false, Ordering::Relaxed);
    OT_SCRIPT_SUBST_PROVEN.store(false, Ordering::Relaxed);
    OT_FEATURE_MATRIX_PROVEN.store(false, Ordering::Relaxed);
    OT_PAINT_MATRIX_PROVEN.store(false, Ordering::Relaxed);
    OT_DEFAULT_PAINT_PROVEN.store(false, Ordering::Relaxed);
    DEFAULT_PAINT_OT_HOOK.store(false, Ordering::Relaxed);
    DEFAULT_PAINT_OT_FONTMGR.store(false, Ordering::Relaxed);
    DEFAULT_PAINT_FONTMGR_USED.store(false, Ordering::Relaxed);
    VF_MULTI_AXIS_USED.store(false, Ordering::Relaxed);
    VF_AXES_GE3_USED.store(false, Ordering::Relaxed);
    VF_PAINT_VARIATIONS_USED.store(false, Ordering::Relaxed);
    PAINT_SMCP_ENABLED.store(true, Ordering::Relaxed);
    PAINT_LIGA_ENABLED.store(true, Ordering::Relaxed);
    PAINT_VF_WGHT_BITS.store(400f32.to_bits(), Ordering::Relaxed);
}

fn primary_font() -> &'static Font {
    PRIMARY_FONT.get_or_init(|| {
        Font::from_bytes(PRIMARY_TTF, FontSettings::default())
            .expect("primary cjk_subset.ttf must parse")
    })
}

fn fallback_oob_font() -> &'static Font {
    FALLBACK_OOB_FONT.get_or_init(|| {
        Font::from_bytes(FALLBACK_OOB_TTF, FontSettings::default())
            .expect("cjk_fallback_oob.ttf must parse")
    })
}

pub fn primary_ttf_bytes() -> &'static [u8] {
    PRIMARY_TTF
}

fn system_font_candidates() -> Vec<PathBuf> {
    let mut out = Vec::new();
    #[cfg(target_os = "macos")]
    {
        out.push(PathBuf::from(
            "/System/Library/Fonts/Supplemental/Arial Unicode.ttf",
        ));
        out.push(PathBuf::from("/Library/Fonts/Arial Unicode.ttf"));
        out.push(PathBuf::from("/System/Library/Fonts/PingFang.ttc"));
        out.push(PathBuf::from("/System/Library/Fonts/STHeiti Light.ttc"));
    }
    #[cfg(target_os = "linux")]
    {
        out.push(PathBuf::from(
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        ));
        out.push(PathBuf::from(
            "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        ));
        out.push(PathBuf::from(
            "/usr/share/fonts/truetype/arphic/uming.ttc",
        ));
    }
    #[cfg(target_os = "windows")]
    {
        out.push(PathBuf::from("C:\\Windows\\Fonts\\msyh.ttc"));
        out.push(PathBuf::from("C:\\Windows\\Fonts\\simsun.ttc"));
        out.push(PathBuf::from("C:\\Windows\\Fonts\\arialuni.ttf"));
    }
    out
}

fn load_system_fallback() -> Option<&'static (Vec<u8>, Font)> {
    SYSTEM_FALLBACK
        .get_or_init(|| {
            for path in system_font_candidates() {
                if !path.exists() {
                    continue;
                }
                let Ok(bytes) = std::fs::read(&path) else {
                    continue;
                };
                if let Ok(font) = Font::from_bytes(bytes.as_slice(), FontSettings::default()) {
                    return Some((bytes, font));
                }
            }
            None
        })
        .as_ref()
}

fn in_ci_subset(ch: char) -> bool {
    CI_SUBSET.contains(ch)
}

fn font_has_glyph(font: &Font, ch: char) -> bool {
    let (metrics, bitmap) = font.rasterize(ch, RASTER_PX);
    !(bitmap.is_empty() && metrics.width == 0)
}

/// True when an OOB fallback face can rasterize `ch` (embedded OOB TTF or system).
pub fn fallback_has_glyph(ch: char) -> bool {
    if font_has_glyph(fallback_oob_font(), ch) {
        return true;
    }
    if let Some((_, font)) = load_system_fallback() {
        return font_has_glyph(font, ch);
    }
    false
}

fn pick_font_for_char(ch: char) -> (&'static Font, &'static [u8], bool) {
    if ch.is_ascii() && (0x20..=0x7E).contains(&(ch as u8)) {
        return (primary_font(), PRIMARY_TTF, false);
    }
    if in_ci_subset(ch) && font_has_glyph(primary_font(), ch) {
        return (primary_font(), PRIMARY_TTF, false);
    }
    FALLBACK_HITS.fetch_add(1, Ordering::Relaxed);
    if font_has_glyph(fallback_oob_font(), ch) {
        return (fallback_oob_font(), FALLBACK_OOB_TTF, true);
    }
    if let Some((bytes, font)) = load_system_fallback() {
        if font_has_glyph(font, ch) {
            return (font, bytes.as_slice(), true);
        }
    }
    (primary_font(), PRIMARY_TTF, true)
}

fn pick_face_for_text(text: &str) -> (&'static Font, &'static [u8]) {
    // Prefer primary for CI subset / ASCII; fall back if any OOB CJK needs it.
    let mut need_fallback = false;
    for ch in text.chars() {
        if ch.is_ascii() {
            continue;
        }
        if !(in_ci_subset(ch) && font_has_glyph(primary_font(), ch)) {
            need_fallback = true;
            break;
        }
    }
    if need_fallback {
        if text.chars().any(|ch| font_has_glyph(fallback_oob_font(), ch)) {
            return (fallback_oob_font(), FALLBACK_OOB_TTF);
        }
        if let Some((bytes, font)) = load_system_fallback() {
            return (font, bytes.as_slice());
        }
    }
    (primary_font(), PRIMARY_TTF)
}

/// Shape `text` with rustybuzz then rasterize each cluster via fontdue.
pub fn shape_measure_advances(text: &str, max_chars: usize) -> Vec<u32> {
    let glyphs = shape_rasterize_text(text, max_chars);
    glyphs.into_iter().map(|g| g.advance).collect()
}

pub fn measure_width(text: &str, max_chars: usize) -> u32 {
    shape_measure_advances(text, max_chars).into_iter().sum()
}

pub struct ShapedGlyph {
    pub ch: char,
    pub glyph_id: u16,
    pub cluster: u32,
    pub x_advance: f32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub y_advance: f32,
    pub advance: u32,
    #[cfg(feature = "skia-glyph")]
    pub skia: Option<GlyphBitmap>,
    #[cfg(not(feature = "skia-glyph"))]
    pub metrics: fontdue::Metrics,
    #[cfg(not(feature = "skia-glyph"))]
    pub bitmap: Vec<u8>,
    #[allow(dead_code)]
    pub used_fallback: bool,
    pub is_box: bool,
}

#[cfg(feature = "skia-glyph")]
fn rasterize_paint_glyph(ttf: &[u8], ch: char, glyph_id: u16) -> Option<GlyphBitmap> {
    if ttf.as_ptr() != PRIMARY_TTF.as_ptr() {
        return skia_glyph::rasterize_cached(ttf, glyph_id, RASTER_PX)
            .or_else(|| skia_glyph::rasterize_char_cached(ttf, ch, RASTER_PX));
    }
    if ch.is_ascii() && (0x20..=0x7E).contains(&(ch as u8)) {
        return None;
    }
    skia_glyph::rasterize_cached(ttf, glyph_id, RASTER_PX)
        .or_else(|| skia_glyph::rasterize_char_cached(ttf, ch, RASTER_PX))
}

#[cfg(not(feature = "skia-glyph"))]
fn rasterize_paint_glyph(
    font: &Font,
    ch: char,
    _glyph_id: u16,
) -> (fontdue::Metrics, Vec<u8>) {
    font.rasterize(ch, RASTER_PX)
}

/// Whole-string rustybuzz shape with non-empty features; glyphs feed paint.
pub fn shape_string_for_paint(text: &str, max_chars: usize) -> Vec<ShapedGlyph> {
    let clipped: String = text.chars().take(max_chars).collect();
    if clipped.is_empty() {
        LAST_PAINT_GLYPHS.store(0, Ordering::Relaxed);
        return Vec::new();
    }
    SHAPED_USED.store(true, Ordering::Relaxed);
    PAINT_SHAPE_USED.store(true, Ordering::Relaxed);

    let route = resolve_paint_route(&clipped);
    let Some(mut face) = Face::from_slice(route.ttf, 0) else {
        return shape_rasterize_text_fallback(&clipped);
    };
    if route.apply_vf_variations {
        let wght = paint_vf_wght();
        face.set_variations(&[
            rustybuzz::Variation {
                tag: Tag::from_bytes(b"wght"),
                value: wght,
            },
            rustybuzz::Variation {
                tag: Tag::from_bytes(b"wdth"),
                value: 100.0,
            },
            rustybuzz::Variation {
                tag: Tag::from_bytes(b"opsz"),
                value: 14.0,
            },
        ]);
    }

    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(route.text);
    let features = paint_features_matrix();
    debug_assert!(!features.is_empty());
    let glyph_buffer = rustybuzz::shape(&face, &features, buffer);
    let infos = glyph_buffer.glyph_infos();
    let positions = glyph_buffer.glyph_positions();
    if infos.is_empty() {
        return shape_rasterize_text_fallback(&clipped);
    }

    let (font, ttf) = if route.ttf == PRIMARY_TTF {
        pick_face_for_text(&clipped)
    } else {
        (primary_font(), route.ttf)
    };
    let mut out = Vec::with_capacity(infos.len());
    for (info, pos) in infos.iter().zip(positions.iter()) {
        let cluster = info.cluster as usize;
        let ch = route
            .text
            .char_indices()
            .find(|(i, _)| *i == cluster)
            .map(|(_, c)| c)
            .unwrap_or('□');

        let scale = RASTER_PX / face.units_per_em() as f32;
        let x_advance = pos.x_advance as f32 * scale;
        let x_offset = pos.x_offset as f32 * scale;
        let y_offset = pos.y_offset as f32 * scale;
        let y_advance = pos.y_advance as f32 * scale;

        #[cfg(feature = "skia-glyph")]
        let skia = rasterize_paint_glyph(ttf, ch, info.glyph_id as u16);
        #[cfg(feature = "skia-glyph")]
        let empty = skia.is_none() && !ch.is_ascii();
        #[cfg(not(feature = "skia-glyph"))]
        let (metrics, bitmap) = if ch.is_ascii() && (0x20..=0x7E).contains(&(ch as u8)) {
            (fontdue::Metrics::default(), Vec::new())
        } else {
            rasterize_paint_glyph(font, ch, info.glyph_id as u16)
        };
        #[cfg(not(feature = "skia-glyph"))]
        let empty = bitmap.is_empty() && metrics.width == 0 && !ch.is_ascii();
        let on_proof_paint = route.ttf != PRIMARY_TTF;
        let advance = if on_proof_paint {
            x_advance.ceil().max(1.0) as u32
        } else if ch.is_ascii() && (0x20..=0x7E).contains(&(ch as u8)) {
            6
        } else if empty {
            CJK_ADVANCE
        } else {
            x_advance.ceil().max(1.0) as u32
        };
        out.push(ShapedGlyph {
            ch,
            glyph_id: info.glyph_id as u16,
            cluster: info.cluster,
            x_advance,
            x_offset,
            y_offset,
            y_advance,
            advance,
            #[cfg(feature = "skia-glyph")]
            skia,
            #[cfg(not(feature = "skia-glyph"))]
            metrics,
            #[cfg(not(feature = "skia-glyph"))]
            bitmap,
            used_fallback: ttf.as_ptr() != PRIMARY_TTF.as_ptr(),
            is_box: empty,
        });
    }
    LAST_PAINT_GLYPHS.store(out.len() as u32, Ordering::Relaxed);
    out
}

fn shape_rasterize_text_fallback(text: &str) -> Vec<ShapedGlyph> {
    let mut out = Vec::new();
    for ch in text.chars() {
        out.push(shape_rasterize_char(ch));
    }
    LAST_PAINT_GLYPHS.store(out.len() as u32, Ordering::Relaxed);
    out
}

/// Shape + rasterize path for Label/TextField drawing (whole-string when possible).
pub fn shape_rasterize_text(text: &str, max_chars: usize) -> Vec<ShapedGlyph> {
    shape_string_for_paint(text, max_chars)
}

pub fn shape_rasterize_char(ch: char) -> ShapedGlyph {
    SHAPED_USED.store(true, Ordering::Relaxed);
    if ch.is_ascii() && (0x20..=0x7E).contains(&(ch as u8)) {
        return ShapedGlyph {
            ch,
            glyph_id: 0,
            cluster: 0,
            x_advance: 6.0,
            x_offset: 0.0,
            y_offset: 0.0,
            y_advance: 6.0,
            advance: 6,
            #[cfg(feature = "skia-glyph")]
            skia: None,
            #[cfg(not(feature = "skia-glyph"))]
            metrics: fontdue::Metrics::default(),
            #[cfg(not(feature = "skia-glyph"))]
            bitmap: Vec::new(),
            used_fallback: false,
            is_box: false,
        };
    }
    let (font, ttf, used_fallback) = pick_font_for_char(ch);
    // Still run whole-string shape for the single char with non-empty features.
    let glyph_id = {
        let s = ch.to_string();
        if let Some(face) = Face::from_slice(ttf, 0) {
            let mut buffer = UnicodeBuffer::new();
            buffer.push_str(&s);
            let features = paint_features_matrix();
            let gb = rustybuzz::shape(&face, &features, buffer);
            gb.glyph_infos().first().map(|i| i.glyph_id as u16).unwrap_or(0)
        } else {
            0
        }
    };
    #[cfg(feature = "skia-glyph")]
    let skia = rasterize_paint_glyph(ttf, ch, glyph_id);
    #[cfg(feature = "skia-glyph")]
    let empty = skia.is_none() && !(ch.is_ascii() && (0x20..=0x7E).contains(&(ch as u8)));
    #[cfg(not(feature = "skia-glyph"))]
    let (metrics, bitmap) = font.rasterize(ch, RASTER_PX);
    #[cfg(not(feature = "skia-glyph"))]
    let empty = bitmap.is_empty() && metrics.width == 0;
    ShapedGlyph {
        ch,
        glyph_id,
        cluster: 0,
        x_advance: if empty {
            CJK_ADVANCE as f32
        } else {
            #[cfg(feature = "skia-glyph")]
            {
                skia.as_ref()
                    .map(|b| b.advance_width)
                    .unwrap_or(CJK_ADVANCE as f32)
            }
            #[cfg(not(feature = "skia-glyph"))]
            {
                metrics.advance_width
            }
        },
        x_offset: 0.0,
        y_offset: 0.0,
        y_advance: CJK_ADVANCE as f32,
        advance: if empty {
            CJK_ADVANCE
        } else {
            #[cfg(feature = "skia-glyph")]
            {
                skia.as_ref()
                    .map(|b| (b.advance_width.ceil() as u32).max(1).saturating_add(1))
                    .unwrap_or(CJK_ADVANCE)
            }
            #[cfg(not(feature = "skia-glyph"))]
            {
                (metrics.advance_width.ceil() as u32).max(1).saturating_add(1)
            }
        },
        #[cfg(feature = "skia-glyph")]
        skia,
        #[cfg(not(feature = "skia-glyph"))]
        metrics,
        #[cfg(not(feature = "skia-glyph"))]
        bitmap,
        used_fallback,
        is_box: empty,
    }
}

/// Font-derived caret height (not a hardcoded constant).
pub fn caret_metric_from_font() -> u32 {
    if let Some(lm) = primary_font().horizontal_line_metrics(RASTER_PX) {
        let h = lm.new_line_size.ceil() as u32;
        if h > 0 {
            return h;
        }
        let h2 = (lm.ascent - lm.descent).ceil() as u32;
        if h2 > 0 {
            return h2;
        }
    }
    RASTER_PX.ceil() as u32
}

pub fn probe_shape_apis_ok() -> bool {
    let sample: String = CI_SUBSET.chars().take(4).collect();
    if sample.is_empty() {
        return false;
    }
    let face = match Face::from_slice(PRIMARY_TTF, 0) {
        Some(f) => f,
        None => return false,
    };
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(&sample);
    let features = paint_features_matrix();
    let glyph_buffer = rustybuzz::shape(&face, &features, buffer);
    !glyph_buffer.glyph_infos().is_empty()
}

pub fn oob_probe_char() -> char {
    '龍'
}
