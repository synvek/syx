//! Phase 106: embedded CJK TTF subset rasterized via fontdue (small dylib).
//! Fingerprint: gui-ttf-expand-v1:cjk64-plus

use fontdue::{Font, FontSettings};
use std::collections::HashSet;
use std::sync::OnceLock;

/// CI-fixed glyph table (≥64 unique CJK/punct); must match `assets/cjk_subset.ttf` subset.
pub const CI_SUBSET: &str = "\
你我他的是不在有这人这中大为上国好来地个时和\
了一就也要到说会可对年以去行学民得经发成十工三么心从动同现什开者度长文方外样\
，。！？、；：\"\"''（）《》【】—…·";

const FONT_BYTES: &[u8] = include_bytes!("../assets/cjk_subset.ttf");
pub const RASTER_PX: f32 = 14.0;

static FONT: OnceLock<Font> = OnceLock::new();

fn font() -> &'static Font {
    FONT.get_or_init(|| {
        Font::from_bytes(FONT_BYTES, FontSettings::default())
            .expect("embedded cjk_subset.ttf must parse")
    })
}

pub const CJK_GLYPH_W: u32 = 14;
pub const CJK_GLYPH_H: u32 = 14;
pub const CJK_ADVANCE: u32 = 15;

pub fn ttf_subset_fingerprint() -> &'static str {
    "gui-ttf-expand-v1:cjk64-plus"
}

pub fn ci_subset_unique_char_count() -> u32 {
    CI_SUBSET.chars().collect::<HashSet<_>>().len() as u32
}

pub fn rasterize_cjk(ch: char) -> Option<(fontdue::Metrics, Vec<u8>)> {
    let (metrics, bitmap) = font().rasterize(ch, RASTER_PX);
    if bitmap.is_empty() && metrics.width == 0 {
        return None;
    }
    Some((metrics, bitmap))
}

pub fn cjk_has_glyph(ch: char) -> bool {
    rasterize_cjk(ch).is_some()
}

pub fn cjk_advance(ch: char) -> u32 {
    if !cjk_has_glyph(ch) {
        return CJK_ADVANCE;
    }
    let (metrics, _) = font().rasterize(ch, RASTER_PX);
    (metrics.advance_width.ceil() as u32).max(1).saturating_add(1)
}

/// Percent of [`CI_SUBSET`] chars that rasterize non-empty in the embedded font.
pub fn glyph_coverage_percent() -> u32 {
    let total = CI_SUBSET.chars().count().max(1);
    let covered = CI_SUBSET.chars().filter(|c| cjk_has_glyph(*c)).count();
    (covered * 100 / total) as u32
}
