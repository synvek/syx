//! Phase 118 Track A: egui + egui-wgpu + all-widgets frame CPU ink + doctor v5.
//! Fingerprints:
//!   gui-egui-wgpu-v1:winit-present
//!   gui-skia-paragraph-v6:os-ot-aligned
//!   gui-skia-glyph-v1:raster-mvp
//!   gui-hb-paint-v1:string-shape
//!   gui-emoji-v9:font-radial-and-sweep
//!   gui-font-system-v5:os-only-all-apis
//!   gui-ot-features-v3:script-subset
//!   gui-vertical-unify-v1:hb-advance
//!   gui-doctor-engine-v6:phase118

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

use egui::{
    epaint::{ClippedPrimitive, Color32, Primitive, Rect, TextureId, Vertex},
    pos2, vec2, Button, CentralPanel, ColorImage, Context, FontData, FontDefinitions, FontFamily,
    FontImage, ImageData, Label, RawInput, RichText, Sense, TextEdit,
};
use egui_winit::State as EguiWinitState;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::emoji;
use crate::shape;
#[cfg(feature = "skia-glyph")]
use crate::skia_glyph;
#[cfg(feature = "skia-glyph")]
use crate::skia_layout;
use crate::{Widget, WidgetKind, WritingMode};

pub const EGUI_WGPU_FINGERPRINT: &str = "gui-egui-wgpu-v1:winit-present";
pub const HB_PAINT_FINGERPRINT: &str = "gui-hb-paint-v1:string-shape";
pub const EMOJI_FINGERPRINT: &str = "gui-emoji-v9:font-radial-and-sweep";
pub const VERTICAL_RL_FINGERPRINT: &str = "gui-vertical-unify-v1:hb-advance";
pub const DOCTOR_WGPU_FINGERPRINT: &str = "gui-doctor-engine-v6:phase118";
pub const PUMP_REENTRANCY_FINGERPRINT: &str = "gui-pump-reentrancy-v1:dirty-defer";

/// Legacy alias — Phase 109 present GA is wgpu, not softbuffer.
pub const EGUI_MVP_FINGERPRINT: &str = EGUI_WGPU_FINGERPRINT;
pub const DOCTOR_EGUI_FINGERPRINT: &str = DOCTOR_WGPU_FINGERPRINT;

static FONTS_INJECTED: AtomicBool = AtomicBool::new(false);
static EGUI_WINIT_BRIDGED: AtomicBool = AtomicBool::new(false);
static VERTICAL_RL_USED: AtomicBool = AtomicBool::new(false);
static VERTICAL_UNIFY: AtomicBool = AtomicBool::new(false);
static VERTICAL_INK: AtomicBool = AtomicBool::new(false);
static VERTICAL_ASCII_INK: AtomicBool = AtomicBool::new(false);
static VERTICAL_HB_VERTICAL: AtomicBool = AtomicBool::new(false);
static HB_PAINT_PATH_USED: AtomicBool = AtomicBool::new(false);
static EGUI_TEXT_SUPPRESSED: AtomicBool = AtomicBool::new(false);
static NO_DOUBLE_PAINT: AtomicBool = AtomicBool::new(false);
static FRAME_CPU_INK: AtomicBool = AtomicBool::new(false);

struct GpuTexture {
    width: usize,
    height: usize,
    /// RGBA8 pixels.
    pixels: Vec<u8>,
}

pub struct EguiWindowState {
    pub ctx: Context,
    pub frames: u32,
    textures: HashMap<TextureId, GpuTexture>,
    pending_events: Vec<egui::Event>,
    /// Real egui-winit event bridge (not a stub import).
    pub egui_winit: Option<EguiWinitState>,
    /// Last tessellated primitives (for wgpu present).
    pub last_primitives: Vec<ClippedPrimitive>,
    pub last_textures_delta: egui::TexturesDelta,
    pub last_pixels_per_point: f32,
}

impl Default for EguiWindowState {
    fn default() -> Self {
        let ctx = Context::default();
        inject_cjk_fonts(&ctx);
        Self {
            ctx,
            frames: 0,
            textures: HashMap::new(),
            pending_events: Vec::new(),
            egui_winit: None,
            last_primitives: Vec::new(),
            last_textures_delta: egui::TexturesDelta::default(),
            last_pixels_per_point: 1.0,
        }
    }
}

impl EguiWindowState {
    /// Ensure a real `egui_winit::State` is bound to this OS window.
    pub fn ensure_egui_winit(&mut self, window: &Window) {
        if self.egui_winit.is_some() {
            return;
        }
        let state = EguiWinitState::new(
            self.ctx.clone(),
            self.ctx.viewport_id(),
            window,
            Some(window.scale_factor() as f32),
            Some(2048),
        );
        self.egui_winit = Some(state);
        EGUI_WINIT_BRIDGED.store(true, Ordering::Relaxed);
    }

    /// Feed a winit window event through the real egui-winit bridge.
    pub fn on_window_event(&mut self, window: &Window, event: &WindowEvent) -> egui_winit::EventResponse {
        self.ensure_egui_winit(window);
        let response = self
            .egui_winit
            .as_mut()
            .expect("egui_winit state")
            .on_window_event(window, event);
        EGUI_WINIT_BRIDGED.store(true, Ordering::Relaxed);
        response
    }

    pub fn take_bridged_input(&mut self, window: &Window) -> RawInput {
        self.ensure_egui_winit(window);
        self.egui_winit
            .as_mut()
            .expect("egui_winit state")
            .take_egui_input(window)
    }

    pub fn push_text_event(&mut self, text: &str) {
        if !text.is_empty() {
            self.pending_events
                .push(egui::Event::Text(text.to_string()));
        }
    }

    pub fn push_key_event(&mut self, key: egui::Key, pressed: bool, modifiers: egui::Modifiers) {
        self.pending_events.push(egui::Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers,
        });
    }

    pub fn queue_pointer(&mut self, pos: egui::Pos2, primary_pressed: Option<bool>) {
        self.pending_events.push(egui::Event::PointerMoved(pos));
        if let Some(pressed) = primary_pressed {
            self.pending_events.push(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::default(),
            });
        }
    }
}

pub fn egui_winit_bridged() -> bool {
    EGUI_WINIT_BRIDGED.load(Ordering::Relaxed)
}

pub fn fonts_injected() -> bool {
    FONTS_INJECTED.load(Ordering::Relaxed)
}

/// Clear used-only doctor probes (test / reset_gui_state hygiene).
pub fn reset_doctor_probes() {
    FONTS_INJECTED.store(false, Ordering::Relaxed);
    VERTICAL_RL_USED.store(false, Ordering::Relaxed);
    VERTICAL_UNIFY.store(false, Ordering::Relaxed);
    VERTICAL_INK.store(false, Ordering::Relaxed);
    VERTICAL_ASCII_INK.store(false, Ordering::Relaxed);
    VERTICAL_HB_VERTICAL.store(false, Ordering::Relaxed);
    HB_PAINT_PATH_USED.store(false, Ordering::Relaxed);
    EGUI_TEXT_SUPPRESSED.store(false, Ordering::Relaxed);
    NO_DOUBLE_PAINT.store(false, Ordering::Relaxed);
    FRAME_CPU_INK.store(false, Ordering::Relaxed);
    EGUI_WINIT_BRIDGED.store(false, Ordering::Relaxed);
    #[cfg(feature = "skia-glyph")]
    skia_layout::reset_skia_layout_stats();
}

pub fn vertical_rl_used() -> bool {
    VERTICAL_RL_USED.load(Ordering::Relaxed)
}

pub fn vertical_rl_layout_used() -> bool {
    vertical_unify_used()
}

pub fn vertical_unify_used() -> bool {
    VERTICAL_UNIFY.load(Ordering::Relaxed)
}

pub fn vertical_ink_used() -> bool {
    VERTICAL_INK.load(Ordering::Relaxed)
}

pub fn vertical_hb_vertical_used() -> bool {
    VERTICAL_HB_VERTICAL.load(Ordering::Relaxed)
}

pub fn egui_text_suppressed_used() -> bool {
    EGUI_TEXT_SUPPRESSED.load(Ordering::Relaxed)
}

pub fn no_double_paint_probe() -> bool {
    NO_DOUBLE_PAINT.load(Ordering::Relaxed)
}

pub fn vertical_ascii_ink_used() -> bool {
    VERTICAL_ASCII_INK.load(Ordering::Relaxed)
}

pub fn hb_paint_path_used() -> bool {
    HB_PAINT_PATH_USED.load(Ordering::Relaxed)
}

fn inject_cjk_fonts(ctx: &Context) {
    if FONTS_INJECTED.load(Ordering::Relaxed) {
        // Still set on this context — each window has its own Context.
    }
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "syx_cjk".to_owned(),
        FontData::from_static(shape::primary_ttf_bytes()),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "syx_cjk".to_owned());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("syx_cjk".to_owned());
    ctx.set_fonts(fonts);
    FONTS_INJECTED.store(true, Ordering::Relaxed);
}

/// Pack RGBA → softbuffer-compatible u32 matching existing syx-gui colors (0xAABBGGRR).
fn rgba_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (a as u32) << 24 | (b as u32) << 16 | (g as u32) << 8 | (r as u32)
}

fn blend_over(dst: u32, src: Color32) -> u32 {
    let [sr, sg, sb, sa] = src.to_array();
    if sa == 0 {
        return dst;
    }
    if sa == 255 {
        return rgba_to_u32(sr, sg, sb, 255);
    }
    let dr = (dst & 0xFF) as u32;
    let dg = ((dst >> 8) & 0xFF) as u32;
    let db = ((dst >> 16) & 0xFF) as u32;
    let a = sa as u32;
    let inv = 255 - a;
    let r = (sr as u32 * a + dr * inv) / 255;
    let g = (sg as u32 * a + dg * inv) / 255;
    let b = (sb as u32 * a + db * inv) / 255;
    rgba_to_u32(r as u8, g as u8, b as u8, 255)
}

fn sample_texture(tex: &GpuTexture, uv: egui::Pos2) -> Color32 {
    if tex.width == 0 || tex.height == 0 || tex.pixels.is_empty() {
        return Color32::TRANSPARENT;
    }
    let u = uv.x.clamp(0.0, 1.0);
    let v = uv.y.clamp(0.0, 1.0);
    let x = ((u * (tex.width as f32 - 1.0)) as usize).min(tex.width - 1);
    let y = ((v * (tex.height as f32 - 1.0)) as usize).min(tex.height - 1);
    let i = (y * tex.width + x) * 4;
    if i + 3 >= tex.pixels.len() {
        return Color32::TRANSPARENT;
    }
    Color32::from_rgba_unmultiplied(
        tex.pixels[i],
        tex.pixels[i + 1],
        tex.pixels[i + 2],
        tex.pixels[i + 3],
    )
}

fn apply_textures_delta(state: &mut EguiWindowState, delta: &egui::TexturesDelta) {
    for (id, image_delta) in &delta.set {
        let image: std::sync::Arc<ColorImage> = match &image_delta.image {
            ImageData::Color(color) => color.clone(),
            ImageData::Font(font) => std::sync::Arc::new(font_image_to_color(font)),
        };
        let (w, h) = (image.width(), image.height());
        let mut rgba = Vec::with_capacity(w * h * 4);
        for px in image.pixels.iter() {
            let [r, g, b, a] = px.to_array();
            rgba.extend_from_slice(&[r, g, b, a]);
        }
        if let Some(pos) = image_delta.pos {
            let tex = state.textures.entry(*id).or_insert_with(|| GpuTexture {
                width: w,
                height: h,
                pixels: vec![0; w * h * 4],
            });
            for row in 0..h {
                for col in 0..w {
                    let src = (row * w + col) * 4;
                    let dx = pos[0] + col;
                    let dy = pos[1] + row;
                    if dx < tex.width && dy < tex.height {
                        let dst = (dy * tex.width + dx) * 4;
                        tex.pixels[dst..dst + 4].copy_from_slice(&rgba[src..src + 4]);
                    }
                }
            }
        } else {
            state.textures.insert(
                *id,
                GpuTexture {
                    width: w,
                    height: h,
                    pixels: rgba,
                },
            );
        }
    }
    for id in &delta.free {
        state.textures.remove(id);
    }
}

fn font_image_to_color(font: &FontImage) -> ColorImage {
    let mut pixels = Vec::with_capacity(font.pixels.len());
    for &cover in &font.pixels {
        let a = (cover * 255.0).round().clamp(0.0, 255.0) as u8;
        pixels.push(Color32::from_rgba_unmultiplied(255, 255, 255, a));
    }
    ColorImage {
        size: font.size,
        pixels,
    }
}

fn edge(ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32) -> f32 {
    (cx - ax) * (by - ay) - (cy - ay) * (bx - ax)
}

fn paint_triangle(
    buf: &mut [u32],
    width: u32,
    height: u32,
    clip: Rect,
    v0: &Vertex,
    v1: &Vertex,
    v2: &Vertex,
    tex: Option<&GpuTexture>,
) {
    let min_x = v0.pos.x.min(v1.pos.x).min(v2.pos.x).max(clip.min.x).max(0.0);
    let max_x = v0
        .pos
        .x
        .max(v1.pos.x)
        .max(v2.pos.x)
        .min(clip.max.x)
        .min(width as f32);
    let min_y = v0.pos.y.min(v1.pos.y).min(v2.pos.y).max(clip.min.y).max(0.0);
    let max_y = v0
        .pos
        .y
        .max(v1.pos.y)
        .max(v2.pos.y)
        .min(clip.max.y)
        .min(height as f32);
    if min_x >= max_x || min_y >= max_y {
        return;
    }
    let area = edge(v0.pos.x, v0.pos.y, v1.pos.x, v1.pos.y, v2.pos.x, v2.pos.y);
    if area.abs() < 1e-6 {
        return;
    }
    let x0 = min_x.floor() as i32;
    let x1 = max_x.ceil() as i32;
    let y0 = min_y.floor() as i32;
    let y1 = max_y.ceil() as i32;
    for y in y0..y1 {
        if y < 0 || y as u32 >= height {
            continue;
        }
        for x in x0..x1 {
            if x < 0 || x as u32 >= width {
                continue;
            }
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let w0 = edge(v1.pos.x, v1.pos.y, v2.pos.x, v2.pos.y, px, py);
            let w1 = edge(v2.pos.x, v2.pos.y, v0.pos.x, v0.pos.y, px, py);
            let w2 = edge(v0.pos.x, v0.pos.y, v1.pos.x, v1.pos.y, px, py);
            if (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0) || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0) {
                let b0 = w0 / area;
                let b1 = w1 / area;
                let b2 = w2 / area;
                let c0 = v0.color.to_array();
                let c1 = v1.color.to_array();
                let c2 = v2.color.to_array();
                let mut color = Color32::from_rgba_unmultiplied(
                    (c0[0] as f32 * b0 + c1[0] as f32 * b1 + c2[0] as f32 * b2).round() as u8,
                    (c0[1] as f32 * b0 + c1[1] as f32 * b1 + c2[1] as f32 * b2).round() as u8,
                    (c0[2] as f32 * b0 + c1[2] as f32 * b1 + c2[2] as f32 * b2).round() as u8,
                    (c0[3] as f32 * b0 + c1[3] as f32 * b1 + c2[3] as f32 * b2).round() as u8,
                );
                if let Some(tex) = tex {
                    let uv = pos2(
                        v0.uv.x * b0 + v1.uv.x * b1 + v2.uv.x * b2,
                        v0.uv.y * b0 + v1.uv.y * b1 + v2.uv.y * b2,
                    );
                    let sampled = sample_texture(tex, uv);
                    let s = sampled.to_array();
                    let c = color.to_array();
                    color = Color32::from_rgba_unmultiplied(
                        ((s[0] as u16 * c[0] as u16) / 255) as u8,
                        ((s[1] as u16 * c[1] as u16) / 255) as u8,
                        ((s[2] as u16 * c[2] as u16) / 255) as u8,
                        ((s[3] as u16 * c[3] as u16) / 255) as u8,
                    );
                }
                if color.a() == 0 {
                    continue;
                }
                let idx = (y as u32 * width + x as u32) as usize;
                if idx < buf.len() {
                    buf[idx] = blend_over(buf[idx], color);
                }
            }
        }
    }
}

fn paint_primitives(
    buf: &mut [u32],
    width: u32,
    height: u32,
    primitives: &[ClippedPrimitive],
    textures: &HashMap<TextureId, GpuTexture>,
) {
    for prim in primitives {
        let clip = prim.clip_rect;
        match &prim.primitive {
            Primitive::Mesh(mesh) => {
                let tex = textures.get(&mesh.texture_id);
                for tri in mesh.indices.chunks_exact(3) {
                    let i0 = tri[0] as usize;
                    let i1 = tri[1] as usize;
                    let i2 = tri[2] as usize;
                    if i0 >= mesh.vertices.len()
                        || i1 >= mesh.vertices.len()
                        || i2 >= mesh.vertices.len()
                    {
                        continue;
                    }
                    paint_triangle(
                        buf,
                        width,
                        height,
                        clip,
                        &mesh.vertices[i0],
                        &mesh.vertices[i1],
                        &mesh.vertices[i2],
                        tex,
                    );
                }
            }
            Primitive::Callback(_) => {}
        }
    }
}

fn blit_shaped_glyph(
    buf: &mut [u32],
    stride: u32,
    x0: u32,
    y_baseline: u32,
    g: &shape::ShapedGlyph,
    ink: u32,
) {
    #[cfg(feature = "skia-glyph")]
    if let Some(ref bmp) = g.skia {
        skia_glyph::blit_alpha8(buf, stride, x0, y_baseline, bmp, ink);
        return;
    }
    #[cfg(not(feature = "skia-glyph"))]
    {
        let ox = x0 as i32 + g.metrics.xmin;
        let oy = y_baseline as i32 + g.metrics.ymin;
        for y in 0..g.metrics.height {
            for x in 0..g.metrics.width {
                let alpha = g.bitmap[y * g.metrics.width + x];
                if alpha > 48 {
                    let px = ox + x as i32;
                    let py = oy + y as i32;
                    if px >= 0 && py >= 0 {
                        let xu = px as u32;
                        let yu = py as u32;
                        if xu < stride && (yu as usize * stride as usize + xu as usize) < buf.len()
                        {
                            buf[yu as usize * stride as usize + xu as usize] = ink;
                        }
                    }
                }
            }
        }
    }
}

fn paint_emoji_overlay(
    buf: &mut [u32],
    width: u32,
    x0: u32,
    y0: u32,
    text: &str,
    vertical_rl: bool,
) {
    if !emoji::text_has_color_emoji(text) {
        return;
    }
    let glyphs = shape::shape_string_for_paint(text, 64);
    if vertical_rl {
        let mut cy = y0 as f32;
        for g in &glyphs {
            if emoji::is_color_emoji(g.ch) {
                let gy = (cy + g.y_offset).max(0.0) as u32;
                let adv = emoji::blit_color_emoji(buf, width, x0, gy, g.ch);
                cy += g.y_advance.max(adv as f32).max(1.0);
            } else {
                cy += g.y_advance.max(g.advance as f32).max(1.0);
            }
        }
    } else {
        let mut cx = x0 as f32;
        for g in &glyphs {
            if emoji::is_color_emoji(g.ch) {
                let x = (cx + g.x_offset).max(0.0) as u32;
                let adv = emoji::blit_color_emoji(buf, width, x, y0, g.ch);
                cx += adv as f32;
            } else {
                cx += g.x_advance.max(g.advance as f32);
            }
        }
    }
}

fn paint_hb_glyph_path(
    buf: &mut [u32],
    width: u32,
    x0: u32,
    y0: u32,
    text: &str,
    vertical_rl: bool,
) {
    HB_PAINT_PATH_USED.store(true, Ordering::Relaxed);
    let glyphs = shape::shape_string_for_paint(text, 64);
    if glyphs.is_empty() {
        return;
    }
    const INK: u32 = 0xFFEEEEEE;
    if vertical_rl {
        VERTICAL_RL_USED.store(true, Ordering::Relaxed);
        VERTICAL_UNIFY.store(true, Ordering::Relaxed);
        VERTICAL_INK.store(true, Ordering::Relaxed);
        VERTICAL_HB_VERTICAL.store(true, Ordering::Relaxed);
        let mut cy = y0 as f32;
        let x = x0;
        for g in &glyphs {
            let gy = (cy + g.y_offset).max(0.0) as u32;
            if emoji::is_color_emoji(g.ch) {
                let adv = emoji::blit_color_emoji(buf, width, x, gy, g.ch);
                cy += g.y_advance.max(adv as f32).max(1.0);
                continue;
            }
            if g.ch.is_ascii() && (0x20..=0x7E).contains(&(g.ch as u8)) {
                if !g.is_box {
                    blit_shaped_glyph(buf, width, x, gy + 11, g, INK);
                    VERTICAL_ASCII_INK.store(true, Ordering::Relaxed);
                }
                cy += g.y_advance.max(g.advance as f32).max(1.0);
                continue;
            }
            if !g.is_box {
                blit_shaped_glyph(buf, width, x, gy + 11, g, INK);
            }
            cy += g.y_advance.max(g.advance as f32).max(1.0);
        }
    } else {
        let mut cx = x0 as f32;
        for g in &glyphs {
            let x = (cx + g.x_offset).max(0.0) as u32;
            if emoji::is_color_emoji(g.ch) {
                let adv = emoji::blit_color_emoji(buf, width, x, y0, g.ch);
                cx += adv as f32;
                continue;
            }
            if g.ch.is_ascii() && (0x20..=0x7E).contains(&(g.ch as u8)) {
                if !g.is_box {
                    blit_shaped_glyph(buf, width, x, y0 + 11, g, INK);
                }
                cx += g.x_advance.max(g.advance as f32);
                continue;
            }
            if !g.is_box {
                blit_shaped_glyph(buf, width, x, y0 + 11, g, INK);
            }
            cx += g.x_advance.max(g.advance as f32);
        }
    }
}

/// Paint widget text: paragraph OR HB/skia-glyph — never both for the same string.
/// Vertical-rl still attempts paragraph (narrow column wrap) before HB fallback.
fn paint_hb_glyphs(
    buf: &mut [u32],
    width: u32,
    x0: u32,
    y0: u32,
    text: &str,
    vertical_rl: bool,
) {
    #[cfg(feature = "skia-glyph")]
    {
        let max_width = if vertical_rl {
            // One-em column so paragraph line-break stacks chars top→bottom.
            (shape::caret_metric_from_font().max(1) as f32) * 1.25
        } else {
            width.saturating_sub(x0).max(1) as f32
        };
        if skia_layout::paint_paragraph(buf, width, x0, y0, text, max_width).is_some() {
            if vertical_rl {
                VERTICAL_RL_USED.store(true, Ordering::Relaxed);
                VERTICAL_UNIFY.store(true, Ordering::Relaxed);
                VERTICAL_INK.store(true, Ordering::Relaxed);
                skia_layout::mark_vertical_paragraph_used();
            }
            paint_emoji_overlay(buf, width, x0, y0, text, vertical_rl);
            return;
        }
    }
    paint_hb_glyph_path(buf, width, x0, y0, text, vertical_rl);
}

fn put_cpu_ink_label(ui: &mut egui::Ui, rect: Rect) {
    EGUI_TEXT_SUPPRESSED.store(true, Ordering::Relaxed);
    ui.put(
        rect,
        Label::new(RichText::new(" ").color(Color32::TRANSPARENT)).sense(Sense::hover()),
    );
}

fn put_cpu_ink_button(ui: &mut egui::Ui, rect: Rect) -> egui::Response {
    EGUI_TEXT_SUPPRESSED.store(true, Ordering::Relaxed);
    // Transparent placeholder — visible ink is CPU-only (ban egui Button text ink).
    ui.put(
        rect,
        Button::new(RichText::new(" ").color(Color32::TRANSPARENT)),
    )
}

fn put_cpu_ink_text_field(
    ui: &mut egui::Ui,
    rect: Rect,
    id: &str,
    text: &mut String,
    focus: &Option<String>,
    widget_id: &str,
) -> (bool, bool) {
    EGUI_TEXT_SUPPRESSED.store(true, Ordering::Relaxed);
    let te = TextEdit::singleline(text)
        .id(egui::Id::new(("syx-tf", id)))
        .desired_width(rect.width())
        .text_color(Color32::TRANSPARENT)
        .frame(true);
    let response = ui.put(rect, te);
    let gained = response.gained_focus();
    let has = response.has_focus();
    if focus.as_ref() == Some(&widget_id.to_string()) && !has {
        response.request_focus();
    }
    (gained, has)
}

fn put_vertical_rl_label(ui: &mut egui::Ui, rect: Rect) {
    VERTICAL_RL_USED.store(true, Ordering::Relaxed);
    put_cpu_ink_label(ui, rect);
}

/// Run one egui frame for Label / Button / TextField; paint into `pixels`.
pub fn run_egui_frame(
    state: &mut EguiWindowState,
    widgets: &mut [Widget],
    focus_widget: &mut Option<String>,
    pixels: &mut [u32],
    width: u32,
    height: u32,
    bg: u32,
) {
    inject_cjk_fonts(&state.ctx);
    let screen = Rect::from_min_size(pos2(0.0, 0.0), vec2(width as f32, height as f32));
    let mut input = RawInput {
        screen_rect: Some(screen),
        max_texture_side: Some(2048),
        ..Default::default()
    };
    input.events.append(&mut state.pending_events);

    let mut texts: Vec<String> = widgets.iter().map(|w| w.text.clone()).collect();
    let ids: Vec<String> = widgets.iter().map(|w| w.id.clone()).collect();
    let kinds: Vec<WidgetKind> = widgets.iter().map(|w| w.kind).collect();
    let modes: Vec<WritingMode> = widgets.iter().map(|w| w.writing_mode).collect();
    let rects: Vec<(u32, u32, u32, u32)> = widgets.iter().map(|w| (w.x, w.y, w.w, w.h)).collect();
    let focus = focus_widget.clone();
    let mut clicked_ids: Vec<String> = Vec::new();
    let mut focused_id: Option<String> = focus.clone();

    let full = state.ctx.run(input, |ctx| {
        CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(
                (bg & 0xFF) as u8,
                ((bg >> 8) & 0xFF) as u8,
                ((bg >> 16) & 0xFF) as u8,
            )))
            .show(ctx, |ui| {
                for i in 0..ids.len() {
                    let (x, y, w, h) = rects[i];
                    let rect = Rect::from_min_size(
                        pos2(x as f32, y as f32),
                        vec2(w.max(8) as f32, h.max(8) as f32),
                    );
                    match kinds[i] {
                        WidgetKind::Label => {
                            if modes[i] == WritingMode::VerticalRl {
                                put_vertical_rl_label(ui, rect);
                            } else {
                                put_cpu_ink_label(ui, rect);
                            }
                        }
                        WidgetKind::Button => {
                            let response = put_cpu_ink_button(ui, rect);
                            if response.clicked() {
                                clicked_ids.push(ids[i].clone());
                                focused_id = Some(ids[i].clone());
                            }
                        }
                        WidgetKind::TextField => {
                            let gained;
                            let has;
                            (gained, has) = put_cpu_ink_text_field(
                                ui,
                                rect,
                                &ids[i],
                                &mut texts[i],
                                &focus,
                                &ids[i],
                            );
                            if gained || has {
                                focused_id = Some(ids[i].clone());
                            }
                        }
                    }
                }
            });
    });

    apply_textures_delta(state, &full.textures_delta);
    let primitives = state.ctx.tessellate(full.shapes, full.pixels_per_point);
    pixels.fill(bg);
    paint_primitives(pixels, width, height, &primitives, &state.textures);

    // CPU ink overlay — same frame, egui text suppressed for Label/Button/TextField.
    FRAME_CPU_INK.store(false, Ordering::Relaxed);
    for i in 0..ids.len() {
        let (x, y, _, _) = rects[i];
        let vertical = modes[i] == WritingMode::VerticalRl;
        paint_hb_glyphs(pixels, width, x + 1, y + 1, &texts[i], vertical);
        FRAME_CPU_INK.store(true, Ordering::Relaxed);
    }
    if FRAME_CPU_INK.load(Ordering::Relaxed) && EGUI_TEXT_SUPPRESSED.load(Ordering::Relaxed) {
        NO_DOUBLE_PAINT.store(true, Ordering::Relaxed);
    }

    state.last_textures_delta = full.textures_delta.clone();
    state.last_primitives = primitives;
    state.last_pixels_per_point = full.pixels_per_point;

    for (i, widget) in widgets.iter_mut().enumerate() {
        if widget.text != texts[i] {
            widget.text = texts[i].clone();
            if widget.kind == WidgetKind::TextField {
                widget.edited = true;
            }
        }
        if clicked_ids.iter().any(|id| id == &widget.id) {
            widget.clicked = true;
        }
    }
    if let Some(id) = focused_id {
        *focus_widget = Some(id);
    }
    state.frames = state.frames.saturating_add(1);
}

/// Map a host key-inject string into egui events (text / named keys).
pub fn enqueue_key_inject(state: &mut EguiWindowState, key: &str) {
    match key {
        "Backspace" | "\u{8}" => {
            state.push_key_event(egui::Key::Backspace, true, egui::Modifiers::default());
        }
        "Delete" => {
            state.push_key_event(egui::Key::Delete, true, egui::Modifiers::default());
        }
        "Left" | "ArrowLeft" => {
            state.push_key_event(egui::Key::ArrowLeft, true, egui::Modifiers::default());
        }
        "Right" | "ArrowRight" => {
            state.push_key_event(egui::Key::ArrowRight, true, egui::Modifiers::default());
        }
        "Enter" | "\n" => {
            state.push_key_event(egui::Key::Enter, true, egui::Modifiers::default());
            state.push_text_event("\n");
        }
        "Tab" => {
            state.push_key_event(egui::Key::Tab, true, egui::Modifiers::default());
        }
        "Shift-Tab" | "Shift+Tab" => {
            let mut mods = egui::Modifiers::default();
            mods.shift = true;
            state.push_key_event(egui::Key::Tab, true, mods);
        }
        _ if !key.is_empty() => {
            let printable: String = key.chars().filter(|c| !c.is_control()).collect();
            if !printable.is_empty() {
                state.push_text_event(&printable);
            }
        }
        _ => {}
    }
}
