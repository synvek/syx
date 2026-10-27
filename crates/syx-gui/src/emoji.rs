//! Phase 117 Track A: CBDT CI font + font-file COLR Linear/Radial/Sweep (doctor synth ≠ GA).
//! Fingerprint: `gui-emoji-v9:font-radial-and-sweep`
//!
//! Primary atlas uses bundled CBDT (`emoji_cbdt.ttf`). COLR path uses
//! `emoji_colr.ttf` with at least one LinearGradient paint (Solid-only ≠ v5).

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{LazyLock, Mutex, OnceLock};

use skia_safe::{
    canvas::SaveLayerRec,
    gradient_shader, surfaces, AlphaType, BlendMode, Canvas, Color, ColorType, Data, Image,
    ImageInfo, Matrix, Paint, PaintStyle, Path, PathBuilder, Point, Rect, TileMode,
};
use ttf_parser::colr::{CompositeMode, GradientExtend, Painter, Paint as ColrPaint};
use ttf_parser::{Face, GlyphId, OutlineBuilder, RasterImageFormat, RgbaColor, Transform};

pub const EMOJI_FINGERPRINT: &str = "gui-emoji-v9:font-radial-and-sweep";

/// Grinning face U+1F600.
pub const EMOJI_GRIN: char = '\u{1F600}';

static ATLAS_HIT: AtomicBool = AtomicBool::new(false);
static ATLAS_HIT_COUNT: AtomicU32 = AtomicU32::new(0);
static COLR_GRAD_USED: AtomicBool = AtomicBool::new(false);
static COLR_RADIAL_USED: AtomicBool = AtomicBool::new(false);
static COLR_SWEEP_USED: AtomicBool = AtomicBool::new(false);
/// Doctor-only synthetic Sweep — must not alone satisfy font Sweep GA.
static COLR_SWEEP_DOCTOR: AtomicBool = AtomicBool::new(false);
/// Doctor-only synthetic Radial — must not alone satisfy font Radial GA.
static COLR_RADIAL_DOCTOR: AtomicBool = AtomicBool::new(false);
static ENGINE: OnceLock<&'static str> = OnceLock::new();

const EMOJI_CBDT_TTF: &[u8] = include_bytes!("../assets/emoji_cbdt.ttf");
const EMOJI_COLR_TTF: &[u8] = include_bytes!("../assets/emoji_colr.ttf");

pub const EMOJI_W: u32 = 16;
pub const EMOJI_H: u32 = 16;

fn pack(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (a as u32) << 24 | (b as u32) << 16 | (g as u32) << 8 | (r as u32)
}

fn engine_label() -> &'static str {
    *ENGINE.get_or_init(|| {
        let face = Face::parse(EMOJI_CBDT_TTF, 0).expect("emoji_cbdt.ttf");
        let tables = face.tables();
        if tables.cbdt.is_some() || tables.bdat.is_some() {
            "cbdt"
        } else if tables.colr.is_some() {
            "colr"
        } else {
            "unknown"
        }
    })
}

pub fn emoji_engine() -> &'static str {
    engine_label()
}

/// Engine tag for the COLR paint path (`colr-grad` once LinearGradient paints).
pub fn emoji_colr_engine() -> &'static str {
    if COLR_GRAD_USED.load(Ordering::Relaxed) {
        "colr-grad"
    } else {
        "colr"
    }
}

pub fn colr_grad_used() -> bool {
    COLR_GRAD_USED.load(Ordering::Relaxed)
}

pub fn colr_radial_used() -> bool {
    COLR_RADIAL_USED.load(Ordering::Relaxed)
}

pub fn colr_sweep_used() -> bool {
    COLR_SWEEP_USED.load(Ordering::Relaxed)
}

pub fn colr_radial_or_sweep_used() -> bool {
    colr_radial_used() && colr_sweep_used()
}

/// Doctor widget tag: actual engine in use for the last atlas hit path.
pub fn doctor_engine_tag() -> &'static str {
    if colr_radial_used() && colr_sweep_used() && !ATLAS_HIT.load(Ordering::Relaxed) {
        return "colr-radial-and-sweep";
    }
    if colr_radial_used() && !ATLAS_HIT.load(Ordering::Relaxed) {
        return "colr-radial";
    }
    if colr_sweep_used() && !ATLAS_HIT.load(Ordering::Relaxed) {
        return "colr-sweep";
    }
    if COLR_GRAD_USED.load(Ordering::Relaxed) && !ATLAS_HIT.load(Ordering::Relaxed) {
        return "colr-grad";
    }
    match emoji_engine() {
        "cbdt" => "cbdt",
        "colr" if colr_radial_used() && colr_sweep_used() => "colr-radial-and-sweep",
        "colr" if colr_radial_used() => "colr-radial",
        "colr" if colr_sweep_used() => "colr-sweep",
        "colr" if COLR_GRAD_USED.load(Ordering::Relaxed) => "colr-grad",
        "colr" => "colr",
        other => other,
    }
}

struct SkiaPathOutline {
    builder: PathBuilder,
}

impl SkiaPathOutline {
    fn new() -> Self {
        Self {
            builder: PathBuilder::new(),
        }
    }

    fn into_path(self) -> Path {
        self.builder.snapshot()
    }
}

impl OutlineBuilder for SkiaPathOutline {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.move_to((x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to((x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.builder.quad_to((x1, y1), (x, y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.builder.cubic_to((x1, y1), (x2, y2), (x, y));
    }

    fn close(&mut self) {
        self.builder.close();
    }
}

fn glyph_path(face: &Face<'_>, gid: GlyphId) -> Option<Path> {
    let mut builder = SkiaPathOutline::new();
    face.outline_glyph(gid, &mut builder)
        .map(|_| builder.into_path())
}

fn composite_blend(mode: CompositeMode) -> BlendMode {
    match mode {
        CompositeMode::Clear => BlendMode::Clear,
        CompositeMode::Source => BlendMode::Src,
        CompositeMode::Destination => BlendMode::Dst,
        CompositeMode::SourceOver => BlendMode::SrcOver,
        CompositeMode::DestinationOver => BlendMode::DstOver,
        CompositeMode::SourceIn => BlendMode::SrcIn,
        CompositeMode::DestinationIn => BlendMode::DstIn,
        CompositeMode::SourceOut => BlendMode::SrcOut,
        CompositeMode::DestinationOut => BlendMode::DstOut,
        CompositeMode::SourceAtop => BlendMode::SrcATop,
        CompositeMode::DestinationAtop => BlendMode::DstATop,
        CompositeMode::Xor => BlendMode::Xor,
        CompositeMode::Plus => BlendMode::Plus,
        CompositeMode::Screen => BlendMode::Screen,
        CompositeMode::Overlay => BlendMode::Overlay,
        CompositeMode::Darken => BlendMode::Darken,
        CompositeMode::Lighten => BlendMode::Lighten,
        CompositeMode::ColorDodge => BlendMode::ColorDodge,
        CompositeMode::ColorBurn => BlendMode::ColorBurn,
        CompositeMode::HardLight => BlendMode::HardLight,
        CompositeMode::SoftLight => BlendMode::SoftLight,
        CompositeMode::Difference => BlendMode::Difference,
        CompositeMode::Exclusion => BlendMode::Exclusion,
        CompositeMode::Multiply => BlendMode::Multiply,
        CompositeMode::Hue => BlendMode::Hue,
        CompositeMode::Saturation => BlendMode::Saturation,
        CompositeMode::Color => BlendMode::Color,
        CompositeMode::Luminosity => BlendMode::Luminosity,
    }
}

fn tile_mode(extend: GradientExtend) -> TileMode {
    match extend {
        GradientExtend::Pad => TileMode::Clamp,
        GradientExtend::Repeat => TileMode::Repeat,
        GradientExtend::Reflect => TileMode::Mirror,
    }
}

struct ColrSkiaPainter<'a> {
    face: &'a Face<'a>,
    canvas: &'a Canvas,
    current_path: Option<Path>,
    clip_depth: u32,
    layer_depth: u32,
}

impl ColrSkiaPainter<'_> {
    fn fill_path(&mut self, path: &Path, color: RgbaColor) {
        if color.alpha == 0 {
            return;
        }
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_color(Color::from_argb(
            color.alpha,
            color.red,
            color.green,
            color.blue,
        ));
        self.canvas.draw_path(path, &paint);
    }

    fn make_radial_gradient_paint(&self, grad: &ttf_parser::colr::RadialGradient<'_>) -> Option<Paint> {
        let stops: Vec<_> = grad.stops(0, &[]).collect();
        if stops.is_empty() {
            return None;
        }
        let colors: Vec<Color> = stops
            .iter()
            .map(|s| Color::from_argb(s.color.alpha, s.color.red, s.color.green, s.color.blue))
            .collect();
        let positions: Vec<f32> = stops.iter().map(|s| s.stop_offset).collect();
        #[allow(deprecated)]
        let shader = gradient_shader::two_point_conical(
            Point::new(grad.x0, grad.y0),
            grad.r0,
            Point::new(grad.x1, grad.y1),
            grad.r1,
            colors.as_slice(),
            Some(positions.as_slice()),
            tile_mode(grad.extend),
            None,
            None,
        )?;
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_shader(shader);
        Some(paint)
    }

    fn fill_radial_gradient(&mut self, path: &Path, grad: &ttf_parser::colr::RadialGradient<'_>) {
        COLR_GRAD_USED.store(true, Ordering::Relaxed);
        COLR_RADIAL_USED.store(true, Ordering::Relaxed);
        if let Some(paint) = self.make_radial_gradient_paint(grad) {
            self.canvas.draw_path(path, &paint);
        }
    }

    fn fill_clip_radial_gradient(&mut self, grad: &ttf_parser::colr::RadialGradient<'_>) {
        COLR_GRAD_USED.store(true, Ordering::Relaxed);
        COLR_RADIAL_USED.store(true, Ordering::Relaxed);
        if let Some(paint) = self.make_radial_gradient_paint(grad) {
            self.canvas.draw_paint(&paint);
        }
    }

    fn make_sweep_gradient_paint(&self, grad: &ttf_parser::colr::SweepGradient<'_>) -> Option<Paint> {
        let stops: Vec<_> = grad.stops(0, &[]).collect();
        if stops.is_empty() {
            return None;
        }
        let colors: Vec<Color> = stops
            .iter()
            .map(|s| Color::from_argb(s.color.alpha, s.color.red, s.color.green, s.color.blue))
            .collect();
        let positions: Vec<f32> = stops.iter().map(|s| s.stop_offset).collect();
        #[allow(deprecated)]
        let shader = gradient_shader::sweep(
            Point::new(grad.center_x, grad.center_y),
            colors.as_slice(),
            Some(positions.as_slice()),
            tile_mode(grad.extend),
            Some((grad.start_angle, grad.end_angle)),
            None,
            None,
        )?;
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_shader(shader);
        Some(paint)
    }

    fn fill_sweep_gradient(&mut self, path: &Path, grad: &ttf_parser::colr::SweepGradient<'_>) {
        COLR_GRAD_USED.store(true, Ordering::Relaxed);
        COLR_SWEEP_USED.store(true, Ordering::Relaxed);
        if let Some(paint) = self.make_sweep_gradient_paint(grad) {
            self.canvas.draw_path(path, &paint);
        }
    }

    fn fill_clip_sweep_gradient(&mut self, grad: &ttf_parser::colr::SweepGradient<'_>) {
        COLR_GRAD_USED.store(true, Ordering::Relaxed);
        COLR_SWEEP_USED.store(true, Ordering::Relaxed);
        if let Some(paint) = self.make_sweep_gradient_paint(grad) {
            self.canvas.draw_paint(&paint);
        }
    }

    fn make_gradient_paint(&self, grad: &ttf_parser::colr::LinearGradient<'_>) -> Option<Paint> {
        let stops: Vec<_> = grad.stops(0, &[]).collect();
        if stops.is_empty() {
            return None;
        }
        let colors: Vec<Color> = stops
            .iter()
            .map(|s| Color::from_argb(s.color.alpha, s.color.red, s.color.green, s.color.blue))
            .collect();
        let positions: Vec<f32> = stops.iter().map(|s| s.stop_offset).collect();
        #[allow(deprecated)]
        let shader = gradient_shader::linear(
            (Point::new(grad.x0, grad.y0), Point::new(grad.x1, grad.y1)),
            colors.as_slice(),
            Some(positions.as_slice()),
            tile_mode(grad.extend),
            None,
            None,
        )?;
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_shader(shader);
        Some(paint)
    }

    fn fill_linear_gradient(&mut self, path: &Path, grad: &ttf_parser::colr::LinearGradient<'_>) {
        COLR_GRAD_USED.store(true, Ordering::Relaxed);
        if let Some(paint) = self.make_gradient_paint(grad) {
            self.canvas.draw_path(path, &paint);
        }
    }

    /// COLRv1 PaintGlyph: outline → push_clip → paint. Fill the active clip.
    fn fill_clip_linear_gradient(&mut self, grad: &ttf_parser::colr::LinearGradient<'_>) {
        COLR_GRAD_USED.store(true, Ordering::Relaxed);
        if let Some(paint) = self.make_gradient_paint(grad) {
            self.canvas.draw_paint(&paint);
        }
    }

    fn fill_clip_solid(&mut self, color: RgbaColor) {
        if color.alpha == 0 {
            return;
        }
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_color(Color::from_argb(
            color.alpha,
            color.red,
            color.green,
            color.blue,
        ));
        self.canvas.draw_paint(&paint);
    }
}

impl Painter<'_> for ColrSkiaPainter<'_> {
    fn outline_glyph(&mut self, glyph_id: GlyphId) {
        self.current_path = glyph_path(self.face, glyph_id);
    }

    fn paint(&mut self, paint: ColrPaint<'_>) {
        match paint {
            ColrPaint::Solid(color) => {
                if let Some(path) = self.current_path.take() {
                    self.fill_path(&path, color);
                } else if self.clip_depth > 0 {
                    self.fill_clip_solid(color);
                }
            }
            ColrPaint::LinearGradient(grad) => {
                if let Some(path) = self.current_path.take() {
                    self.fill_linear_gradient(&path, &grad);
                } else if self.clip_depth > 0 {
                    self.fill_clip_linear_gradient(&grad);
                }
            }
            ColrPaint::RadialGradient(grad) => {
                if let Some(path) = self.current_path.take() {
                    self.fill_radial_gradient(&path, &grad);
                } else if self.clip_depth > 0 {
                    self.fill_clip_radial_gradient(&grad);
                }
            }
            ColrPaint::SweepGradient(grad) => {
                if let Some(path) = self.current_path.take() {
                    self.fill_sweep_gradient(&path, &grad);
                } else if self.clip_depth > 0 {
                    self.fill_clip_sweep_gradient(&grad);
                }
            }
        }
    }

    fn push_clip(&mut self) {
        if let Some(path) = self.current_path.take() {
            self.canvas.save();
            self.canvas.clip_path(&path, None, true);
            self.clip_depth = self.clip_depth.saturating_add(1);
        }
    }

    fn push_clip_box(&mut self, clipbox: ttf_parser::colr::ClipBox) {
        self.canvas.save();
        self.canvas.clip_rect(
            skia_safe::Rect::from_ltrb(
                clipbox.x_min as f32,
                clipbox.y_min as f32,
                clipbox.x_max as f32,
                clipbox.y_max as f32,
            ),
            None,
            true,
        );
        self.clip_depth = self.clip_depth.saturating_add(1);
    }

    fn pop_clip(&mut self) {
        if self.clip_depth > 0 {
            self.canvas.restore();
            self.clip_depth -= 1;
        }
    }

    fn push_layer(&mut self, mode: CompositeMode) {
        let mut paint = Paint::default();
        paint.set_blend_mode(composite_blend(mode));
        let rec = SaveLayerRec::default().paint(&paint);
        let _ = self.canvas.save_layer(&rec);
        self.layer_depth = self.layer_depth.saturating_add(1);
    }

    fn pop_layer(&mut self) {
        if self.layer_depth > 0 {
            self.canvas.restore();
            self.layer_depth -= 1;
        }
    }

    fn push_transform(&mut self, transform: Transform) {
        self.canvas.save();
        let m = Matrix::new_all(
            transform.a,
            transform.c,
            transform.e,
            transform.b,
            transform.d,
            transform.f,
            0.0,
            0.0,
            1.0,
        );
        self.canvas.concat(&m);
    }

    fn pop_transform(&mut self) {
        self.canvas.restore();
    }
}

fn decode_png_pixels(png: &[u8], w: u32, h: u32) -> Option<Vec<u32>> {
    let data = Data::new_copy(png);
    let image = Image::from_encoded(data)?;
    let info = ImageInfo::new(
        (w as i32, h as i32),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
        None,
    );
    let mut surface = surfaces::raster(&info, None, None)?;
    let canvas = surface.canvas();
    canvas.clear(Color::TRANSPARENT);
    canvas.draw_image(&image, (0, 0), None);
    let mut rgba = vec![0u8; (w * h * 4) as usize];
    if !surface.read_pixels(&info, &mut rgba, (w * 4) as usize, (0, 0)) {
        return None;
    }
    let mut out = Vec::with_capacity((w * h) as usize);
    for px in rgba.chunks_exact(4) {
        out.push(pack(px[0], px[1], px[2], px[3]));
    }
    Some(out)
}

fn rasterize_cbdt_glyph(face: &Face<'_>, ch: char) -> Option<(Vec<u32>, u32, u32)> {
    let gid = face.glyph_index(ch)?;
    let img = face.glyph_raster_image(gid, EMOJI_H as u16)?;
    let w = img.width as u32;
    let h = img.height as u32;
    if w == 0 || h == 0 {
        return None;
    }
    match img.format {
        RasterImageFormat::PNG => {
            let canvas_px = decode_png_pixels(img.data, w, h)?;
            Some((canvas_px, w, h))
        }
        RasterImageFormat::BitmapPremulBgra32 => {
            let mut canvas_px = Vec::with_capacity((w * h) as usize);
            for px in img.data.chunks_exact(4) {
                canvas_px.push(pack(px[2], px[1], px[0], px[3]));
            }
            Some((canvas_px, w, h))
        }
        RasterImageFormat::BitmapGray8 => {
            let mut canvas_px = Vec::with_capacity((w * h) as usize);
            for &cover in img.data {
                let v = cover;
                canvas_px.push(pack(v, v, v, if v > 0 { 255 } else { 0 }));
            }
            Some((canvas_px, w, h))
        }
        _ => {
            let mut canvas_px = Vec::with_capacity((w * h) as usize);
            for &cover in img.data {
                let v = if cover > 0 { 255 } else { 0 };
                canvas_px.push(pack(v, v, v, v));
            }
            Some((canvas_px, w, h))
        }
    }
}

fn rasterize_colr_glyph(ch: char) -> Option<(Vec<u32>, u32, u32)> {
    let face = Face::parse(EMOJI_COLR_TTF, 0).ok()?;
    let gid = face.glyph_index(ch)?;
    let units_per_em = face.units_per_em() as f32;
    let scale = EMOJI_H as f32 / units_per_em;
    let info = ImageInfo::new(
        (EMOJI_W as i32, EMOJI_H as i32),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );
    let mut surface = surfaces::raster(&info, None, None)?;
    let canvas = surface.canvas();
    canvas.clear(Color::TRANSPARENT);
    canvas.save();
    canvas.translate((0.0, EMOJI_H as f32));
    canvas.scale((scale, -scale));
    let mut painter = ColrSkiaPainter {
        face: &face,
        canvas,
        current_path: None,
        clip_depth: 0,
        layer_depth: 0,
    };
    face.paint_color_glyph(gid, 0, RgbaColor::new(0, 0, 0, 255), &mut painter)?;
    canvas.restore();

    let mut rgba = vec![0u8; (EMOJI_W * EMOJI_H * 4) as usize];
    if !surface.read_pixels(&info, &mut rgba, (EMOJI_W * 4) as usize, (0, 0)) {
        return None;
    }
    let mut canvas_px = Vec::with_capacity((EMOJI_W * EMOJI_H) as usize);
    for px in rgba.chunks_exact(4) {
        canvas_px.push(pack(px[0], px[1], px[2], px[3]));
    }
    Some((canvas_px, EMOJI_W, EMOJI_H))
}

fn rasterize_emoji_glyph(ch: char) -> Option<(Vec<u32>, u32, u32)> {
    let face = Face::parse(EMOJI_CBDT_TTF, 0).ok()?;
    let tables = face.tables();
    if tables.cbdt.is_some() || tables.bdat.is_some() {
        if let Some(px) = rasterize_cbdt_glyph(&face, ch) {
            return Some(px);
        }
    }
    rasterize_colr_glyph(ch)
}

static CBDT_ATLAS: OnceLock<(Vec<u32>, u32, u32)> = OnceLock::new();
static COLR_ATLAS: OnceLock<(Vec<u32>, u32, u32)> = OnceLock::new();
static CBDT_GLYPH_CACHE: LazyLock<Mutex<HashMap<char, (Vec<u32>, u32, u32)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn cbdt_atlas() -> &'static (Vec<u32>, u32, u32) {
    CBDT_ATLAS.get_or_init(|| {
        let face = Face::parse(EMOJI_CBDT_TTF, 0).expect("emoji_cbdt.ttf");
        rasterize_cbdt_glyph(&face, EMOJI_GRIN).expect("CBDT atlas for U+1F600")
    })
}

fn colr_atlas() -> &'static (Vec<u32>, u32, u32) {
    COLR_ATLAS.get_or_init(|| rasterize_colr_glyph(EMOJI_GRIN).expect("COLR atlas for U+1F600"))
}

/// Force COLR LinearGradient paint path (CI / doctor probe).
pub fn probe_colr_gradient_atlas() -> bool {
    COLR_GRAD_USED.store(false, Ordering::Relaxed);
    COLR_RADIAL_USED.store(false, Ordering::Relaxed);
    COLR_SWEEP_USED.store(false, Ordering::Relaxed);
    let Some((canvas, _, _)) = rasterize_colr_glyph(EMOJI_GRIN) else {
        return false;
    };
    (COLR_GRAD_USED.load(Ordering::Relaxed)
        || colr_radial_used()
        || colr_sweep_used())
        && canvas.iter().any(|&px| px != 0)
}

fn prove_colr_radial_doctor_paint() -> bool {
    let info = ImageInfo::new(
        (EMOJI_W as i32, EMOJI_H as i32),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );
    let Some(mut surface) = surfaces::raster(&info, None, None) else {
        return false;
    };
    let canvas = surface.canvas();
    canvas.clear(Color::TRANSPARENT);
    let mut builder = PathBuilder::new();
    builder.add_circle(
        (EMOJI_W as f32 / 2.0, EMOJI_H as f32 / 2.0),
        EMOJI_W as f32 / 2.0 - 1.0,
        None,
    );
    let path = builder.snapshot();
    let colors = [Color::RED, Color::BLUE];
    #[allow(deprecated)]
    let Some(shader) = gradient_shader::two_point_conical(
        Point::new(EMOJI_W as f32 / 2.0, EMOJI_H as f32 / 2.0),
        1.0,
        Point::new(EMOJI_W as f32 / 2.0, EMOJI_H as f32 / 2.0),
        EMOJI_W as f32 / 2.0 - 1.0,
        &colors[..],
        None,
        TileMode::Clamp,
        None,
        None,
    ) else {
        return false;
    };
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_shader(shader);
    canvas.draw_path(&path, &paint);
    COLR_RADIAL_DOCTOR.store(true, Ordering::Relaxed);
    true
}

fn prove_colr_sweep_doctor_paint() -> bool {
    let info = ImageInfo::new(
        (EMOJI_W as i32, EMOJI_H as i32),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );
    let Some(mut surface) = surfaces::raster(&info, None, None) else {
        return false;
    };
    let canvas = surface.canvas();
    canvas.clear(Color::TRANSPARENT);
    let mut builder = PathBuilder::new();
    builder.add_circle(
        (EMOJI_W as f32 / 2.0, EMOJI_H as f32 / 2.0),
        EMOJI_W as f32 / 2.0 - 1.0,
        None,
    );
    let path = builder.snapshot();
    let colors = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
    ];
    #[allow(deprecated)]
    let Some(shader) = gradient_shader::sweep(
        Point::new(EMOJI_W as f32 / 2.0, EMOJI_H as f32 / 2.0),
        &colors[..],
        None,
        TileMode::Clamp,
        Some((0.0, 360.0)),
        None,
        None,
    ) else {
        return false;
    };
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_shader(shader);
    canvas.draw_path(&path, &paint);
    COLR_SWEEP_DOCTOR.store(true, Ordering::Relaxed);
    true
}

pub fn colr_radial_doctor_used() -> bool {
    COLR_RADIAL_DOCTOR.load(Ordering::Relaxed)
}

pub fn colr_sweep_doctor_used() -> bool {
    COLR_SWEEP_DOCTOR.load(Ordering::Relaxed)
}

pub fn probe_colr_sweep_paint() -> bool {
    // Phase 117: GA requires font paint-tree Sweep; doctor synth alone is insufficient.
    COLR_SWEEP_USED.store(false, Ordering::Relaxed);
    COLR_SWEEP_DOCTOR.store(false, Ordering::Relaxed);
    let _ = prove_colr_sweep_doctor_paint();
    if colr_sweep_used() {
        // Doctor incorrectly mutated GA flag.
        return false;
    }
    if !colr_sweep_doctor_used() {
        return false;
    }
    COLR_SWEEP_USED.store(false, Ordering::Relaxed);
    rasterize_colr_glyph(EMOJI_GRIN)
        .map(|(canvas, _, _)| canvas.iter().any(|&px| px != 0) && colr_sweep_used())
        .unwrap_or(false)
}

pub fn probe_colr_radial_or_sweep_atlas() -> bool {
    COLR_RADIAL_USED.store(false, Ordering::Relaxed);
    COLR_SWEEP_USED.store(false, Ordering::Relaxed);
    COLR_RADIAL_DOCTOR.store(false, Ordering::Relaxed);
    COLR_SWEEP_DOCTOR.store(false, Ordering::Relaxed);
    // Doctor probes may run but must not alone satisfy GA flags.
    let _ = prove_colr_radial_doctor_paint();
    let _ = prove_colr_sweep_doctor_paint();
    if colr_radial_used() || colr_sweep_used() {
        return false;
    }
    COLR_RADIAL_USED.store(false, Ordering::Relaxed);
    COLR_SWEEP_USED.store(false, Ordering::Relaxed);
    let font_ok = rasterize_colr_glyph(EMOJI_GRIN)
        .map(|(canvas, _, _)| {
            canvas.iter().any(|&px| px != 0) && colr_radial_used() && colr_sweep_used()
        })
        .unwrap_or(false);
    font_ok && colr_radial_or_sweep_used() && colr_radial_doctor_used() && colr_sweep_doctor_used()
}

pub fn is_color_emoji(ch: char) -> bool {
    ch == EMOJI_GRIN || ('\u{1F600}'..='\u{1F64F}').contains(&ch)
}

pub fn text_has_color_emoji(text: &str) -> bool {
    text.chars().any(is_color_emoji)
}

pub fn mark_atlas_hit() {
    ATLAS_HIT.store(true, Ordering::Relaxed);
    ATLAS_HIT_COUNT.fetch_add(1, Ordering::Relaxed);
}

pub fn atlas_hit() -> bool {
    ATLAS_HIT.load(Ordering::Relaxed)
}

pub fn atlas_hit_count() -> u32 {
    ATLAS_HIT_COUNT.load(Ordering::Relaxed)
}

pub fn reset_emoji_stats() {
    ATLAS_HIT.store(false, Ordering::Relaxed);
    ATLAS_HIT_COUNT.store(0, Ordering::Relaxed);
    COLR_GRAD_USED.store(false, Ordering::Relaxed);
    COLR_RADIAL_USED.store(false, Ordering::Relaxed);
    COLR_SWEEP_USED.store(false, Ordering::Relaxed);
    COLR_RADIAL_DOCTOR.store(false, Ordering::Relaxed);
    COLR_SWEEP_DOCTOR.store(false, Ordering::Relaxed);
    CBDT_GLYPH_CACHE.lock().unwrap().clear();
}

pub fn emoji_fingerprint() -> &'static str {
    EMOJI_FINGERPRINT
}

pub fn emoji_pixel(x: u32, y: u32) -> u32 {
    let (canvas, w, h) = cbdt_atlas();
    if x >= *w || y >= *h {
        return 0;
    }
    canvas[(y * w + x) as usize]
}

fn cached_cbdt_glyph(ch: char) -> Option<(Vec<u32>, u32, u32)> {
    if ch == EMOJI_GRIN {
        let (canvas, w, h) = cbdt_atlas();
        return Some((canvas.clone(), *w, *h));
    }
    if let Some(hit) = CBDT_GLYPH_CACHE.lock().unwrap().get(&ch).cloned() {
        return Some(hit);
    }
    let face = Face::parse(EMOJI_CBDT_TTF, 0).ok()?;
    let px = rasterize_cbdt_glyph(&face, ch)?;
    CBDT_GLYPH_CACHE.lock().unwrap().insert(ch, px.clone());
    Some(px)
}

fn blit_pixels(buf: &mut [u32], stride: u32, x0: u32, y0: u32, canvas: &[u32], w: u32, h: u32) {
    for dy in 0..h {
        for dx in 0..w {
            let px = canvas[(dy * w + dx) as usize];
            if px == 0 {
                continue;
            }
            let x = x0 + dx;
            let y = y0 + dy;
            if x < stride {
                let idx = (y * stride + x) as usize;
                if idx < buf.len() {
                    buf[idx] = px;
                }
            }
        }
    }
}

fn resolve_emoji_pixels(ch: char) -> Option<(Vec<u32>, u32, u32)> {
    if let Some(px) = cached_cbdt_glyph(ch) {
        return Some(px);
    }
    rasterize_emoji_glyph(ch)
}

/// Blit color emoji into CPU pixel buffer; returns advance width.
/// Primary path is CBDT; falls back to COLR gradient atlas.
pub fn blit_color_emoji(
    buf: &mut [u32],
    stride: u32,
    x0: u32,
    y0: u32,
    ch: char,
) -> u32 {
    if !is_color_emoji(ch) {
        return 0;
    }
    let Some((canvas, w, h)) = resolve_emoji_pixels(ch) else {
        return 0;
    };
    mark_atlas_hit();
    blit_pixels(buf, stride, x0, y0, &canvas, w, h);
    w + 1
}

/// True when a painted pixel is recognizably color-emoji (yellow chroma, not gray box).
pub fn pixel_is_emoji_chroma(px: u32) -> bool {
    let r = (px & 0xFF) as u8;
    let g = ((px >> 8) & 0xFF) as u8;
    let b = ((px >> 16) & 0xFF) as u8;
    r > 180 && g > 140 && b < 100
}
