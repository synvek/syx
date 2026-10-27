//! Phase 111 Track A: skia-safe glyph raster MVP (HB clusters → atlas pixels → wgpu).
//! Fingerprint: `gui-skia-glyph-v1:raster-mvp`

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

use skia_safe::{
    surfaces, AlphaType, Color, ColorType, Font, FontMgr, GlyphId, ImageInfo, Paint, PaintStyle,
    Point, Typeface,
};

static SKIA_GLYPH_USED: AtomicBool = AtomicBool::new(false);

type CacheKey = (usize, u32, u32);
static GLYPH_CACHE: OnceLock<Mutex<HashMap<CacheKey, GlyphBitmap>>> = OnceLock::new();

fn cache() -> &'static Mutex<HashMap<CacheKey, GlyphBitmap>> {
    GLYPH_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub const SKIA_GLYPH_FINGERPRINT: &str = "gui-skia-glyph-v1:raster-mvp";

#[derive(Clone, Debug)]
pub struct GlyphBitmap {
    pub width: u32,
    pub height: u32,
    pub xmin: i32,
    pub ymin: i32,
    pub advance_width: f32,
    /// Alpha8 coverage (row-major).
    pub alpha: Vec<u8>,
}

pub fn skia_glyph_fingerprint() -> &'static str {
    SKIA_GLYPH_FINGERPRINT
}

pub fn skia_glyph_used() -> bool {
    SKIA_GLYPH_USED.load(Ordering::Relaxed)
}

pub fn reset_skia_glyph_stats() {
    SKIA_GLYPH_USED.store(false, Ordering::Relaxed);
    if let Some(c) = GLYPH_CACHE.get() {
        c.lock().unwrap().clear();
    }
}

fn typeface_from_bytes(ttf: &[u8]) -> Option<Typeface> {
    FontMgr::new().new_from_data(ttf, None)
}

fn rasterize_glyph_id(ttf: &[u8], glyph_id: u16, px_size: f32) -> Option<GlyphBitmap> {
    let face = typeface_from_bytes(ttf)?;
    let font = Font::from_typeface(face, px_size);
    let gid = GlyphId::from(glyph_id);
    let path = font.get_path(gid)?;
    let bounds = path.bounds();
    if bounds.width() <= 0.0 || bounds.height() <= 0.0 {
        return None;
    }
    let xmin = bounds.left().floor() as i32;
    let ymin = bounds.top().floor() as i32;
    let width = bounds.width().ceil().max(1.0) as i32 as u32;
    let height = bounds.height().ceil().max(1.0) as i32 as u32;
    let mut widths = [0.0f32];
    font.get_widths(&[gid], &mut widths);
    let advance_width = widths[0].max(bounds.width());

    let info = ImageInfo::new(
        (width as i32, height as i32),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );
    let mut surface = surfaces::raster(&info, None, None)?;
    let canvas = surface.canvas();
    canvas.clear(Color::TRANSPARENT);
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(Color::WHITE);
    canvas.save();
    canvas.translate((-bounds.left(), -bounds.top()));
    canvas.draw_path(&path, &paint);
    canvas.restore();

    let mut rgba = vec![0u8; (width * height * 4) as usize];
    if !surface.read_pixels(&info, &mut rgba, (width * 4) as usize, (0, 0)) {
        return None;
    }

    let mut alpha = Vec::with_capacity((width * height) as usize);
    for px in rgba.chunks_exact(4) {
        alpha.push(px[3]);
    }

    SKIA_GLYPH_USED.store(true, Ordering::Relaxed);
    Some(GlyphBitmap {
        width,
        height,
        xmin,
        ymin,
        advance_width,
        alpha,
    })
}

fn rasterize_char(ttf: &[u8], ch: char, px_size: f32) -> Option<GlyphBitmap> {
    let face = typeface_from_bytes(ttf)?;
    let font = Font::from_typeface(face, px_size);
    let s = ch.to_string();
    let (_adv, bounds) = font.measure_str(&s, None);
    if bounds.width() <= 0.0 || bounds.height() <= 0.0 {
        return None;
    }
    let width = bounds.width().ceil().max(1.0) as u32;
    let height = bounds.height().ceil().max(1.0) as i32 as u32;
    let info = ImageInfo::new(
        (width as i32, height as i32),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );
    let mut surface = surfaces::raster(&info, None, None)?;
    let canvas = surface.canvas();
    canvas.clear(Color::TRANSPARENT);
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(Color::WHITE);
    canvas.draw_str(&s, Point::new(-bounds.left(), -bounds.top()), &font, &paint);

    let mut rgba = vec![0u8; (width * height * 4) as usize];
    if !surface.read_pixels(&info, &mut rgba, (width * 4) as usize, (0, 0)) {
        return None;
    }

    let mut alpha = Vec::with_capacity((width * height) as usize);
    for px in rgba.chunks_exact(4) {
        alpha.push(px[3]);
    }

    SKIA_GLYPH_USED.store(true, Ordering::Relaxed);
    Some(GlyphBitmap {
        width,
        height,
        xmin: bounds.left().floor() as i32,
        ymin: bounds.top().floor() as i32,
        advance_width: bounds.width(),
        alpha,
    })
}

/// Rasterize by OpenType glyph id (HB paint path) with atlas cache.
pub fn rasterize_cached(ttf: &[u8], glyph_id: u16, px_size: f32) -> Option<GlyphBitmap> {
    let key = (ttf.as_ptr() as usize, glyph_id as u32, px_size.to_bits());
    if let Some(hit) = cache().lock().unwrap().get(&key) {
        SKIA_GLYPH_USED.store(true, Ordering::Relaxed);
        return Some(hit.clone());
    }
    let bmp = rasterize_glyph_id(ttf, glyph_id, px_size)?;
    cache().lock().unwrap().insert(key, bmp.clone());
    Some(bmp)
}

/// Rasterize a scalar (fallback / per-char probes).
pub fn rasterize_char_cached(ttf: &[u8], ch: char, px_size: f32) -> Option<GlyphBitmap> {
    let key = (
        ttf.as_ptr() as usize,
        ch as u32 | 0x8000_0000,
        px_size.to_bits(),
    );
    if let Some(hit) = cache().lock().unwrap().get(&key) {
        SKIA_GLYPH_USED.store(true, Ordering::Relaxed);
        return Some(hit.clone());
    }
    let bmp = rasterize_char(ttf, ch, px_size)?;
    cache().lock().unwrap().insert(key, bmp.clone());
    Some(bmp)
}

pub fn blit_alpha8(
    buf: &mut [u32],
    stride: u32,
    x0: u32,
    y_baseline: u32,
    bmp: &GlyphBitmap,
    ink: u32,
) {
    let ox = x0 as i32 + bmp.xmin;
    let oy = y_baseline as i32 + bmp.ymin;
    for y in 0..bmp.height {
        for x in 0..bmp.width {
            let alpha = bmp.alpha[(y * bmp.width + x) as usize];
            if alpha > 48 {
                let px = ox + x as i32;
                let py = oy + y as i32;
                if px >= 0 && py >= 0 {
                    let xu = px as u32;
                    let yu = py as u32;
                    if xu < stride && (yu as usize * stride as usize + xu as usize) < buf.len() {
                        buf[yu as usize * stride as usize + xu as usize] = ink;
                    }
                }
            }
        }
    }
}
