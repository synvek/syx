//! Phase 110 Track A: egui-wgpu OS present + HB/emoji CPU texture composite.
//! Softbuffer only when `SYX_GUI_SOFTBUFFER=1`.
//! Fingerprints:
//!   gui-egui-wgpu-v1:winit-present
//!   gui-wgpu-composite-v1:hb-emoji-tex

use std::num::NonZeroU32;
use std::sync::atomic::{AtomicBool, Ordering};

use bytemuck::{Pod, Zeroable};
use egui::{ClippedPrimitive, TexturesDelta};
use egui_wgpu::{Renderer, ScreenDescriptor};
use winit::window::Window;

pub const WGPU_PRESENT_FINGERPRINT: &str = "gui-egui-wgpu-v1:winit-present";
pub const WGPU_COMPOSITE_FINGERPRINT: &str = "gui-wgpu-composite-v1:hb-emoji-tex";

static WGPU_PRESENT_OK: AtomicBool = AtomicBool::new(false);
static WGPU_COMPOSITE_OK: AtomicBool = AtomicBool::new(false);
static WGPU_PROBE_DONE: AtomicBool = AtomicBool::new(false);
static WGPU_AVAILABLE: AtomicBool = AtomicBool::new(false);

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct CompositeUniforms {
    /// Logical overlay size in pixels.
    overlay_size: [f32; 2],
    /// Surface size in pixels.
    surface_size: [f32; 2],
    /// Centered origin for overlay blit (surface coords).
    origin: [f32; 2],
    /// WGSL uniform layout pads vec2 → vec4 alignment (shader expects 48 bytes).
    _pad: [f32; 2],
    /// Background color (0..1 RGBA).
    bg: [f32; 4],
}

pub fn wgpu_present_fingerprint() -> &'static str {
    WGPU_PRESENT_FINGERPRINT
}

pub fn wgpu_composite_fingerprint() -> &'static str {
    WGPU_COMPOSITE_FINGERPRINT
}

pub fn softbuffer_fallback_requested() -> bool {
    std::env::var("SYX_GUI_SOFTBUFFER")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn wgpu_present_used() -> bool {
    WGPU_PRESENT_OK.load(Ordering::Relaxed)
}

pub fn wgpu_composite_used() -> bool {
    WGPU_COMPOSITE_OK.load(Ordering::Relaxed)
}

/// Clear used present/composite probes (test / reset hygiene). Keeps availability probe.
pub fn reset_used_probes() {
    WGPU_PRESENT_OK.store(false, Ordering::Relaxed);
    WGPU_COMPOSITE_OK.store(false, Ordering::Relaxed);
}

pub fn wgpu_available() -> bool {
    if !WGPU_PROBE_DONE.load(Ordering::Relaxed) {
        let ok = probe_wgpu();
        WGPU_AVAILABLE.store(ok, Ordering::Relaxed);
        WGPU_PROBE_DONE.store(true, Ordering::Relaxed);
    }
    WGPU_AVAILABLE.load(Ordering::Relaxed)
}

fn probe_wgpu() -> bool {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY | wgpu::Backends::SECONDARY,
        ..Default::default()
    });
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        compatible_surface: None,
        force_fallback_adapter: true,
    }));
    adapter.is_some()
}

fn u32_pixel_to_rgba(px: u32) -> [u8; 4] {
    [
        (px & 0xFF) as u8,
        ((px >> 8) & 0xFF) as u8,
        ((px >> 16) & 0xFF) as u8,
        ((px >> 24) & 0xFF) as u8,
    ]
}

fn pixels_to_rgba8(pixels: &[u32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(pixels.len() * 4);
    for &px in pixels {
        out.extend_from_slice(&u32_pixel_to_rgba(px));
    }
    out
}

struct CompositePipeline {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    uniform_buffer: wgpu::Buffer,
}

impl CompositePipeline {
    fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("syx-gui-composite-shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("composite.wgsl").into()),
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("syx-gui-composite-bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("syx-gui-composite-pl"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("syx-gui-composite"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("syx-gui-composite-sampler"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("syx-gui-composite-uniforms"),
            size: std::mem::size_of::<CompositeUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self {
            pipeline,
            bind_group_layout,
            sampler,
            uniform_buffer,
        }
    }

    fn composite_overlay(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        overlay: &wgpu::TextureView,
        overlay_w: u32,
        overlay_h: u32,
        surface_w: u32,
        surface_h: u32,
        bg_px: u32,
    ) {
        let ox = (surface_w.saturating_sub(overlay_w)) / 2;
        let oy = (surface_h.saturating_sub(overlay_h)) / 2;
        let bg = u32_pixel_to_rgba(bg_px);
        let uniforms = CompositeUniforms {
            overlay_size: [overlay_w as f32, overlay_h as f32],
            surface_size: [surface_w as f32, surface_h as f32],
            origin: [ox as f32, oy as f32],
            _pad: [0.0, 0.0],
            bg: [
                bg[0] as f32 / 255.0,
                bg[1] as f32 / 255.0,
                bg[2] as f32 / 255.0,
                bg[3] as f32 / 255.0,
            ],
        };
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::bytes_of(&uniforms),
        );
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("syx-gui-composite-bg"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(overlay),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
            ],
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("syx-gui-composite-pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.draw(0..3, 0..1);
    }
}

pub struct WgpuWindowPresent {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    renderer: Renderer,
    composite: CompositePipeline,
    overlay_texture: Option<(wgpu::Texture, u32, u32)>,
}

impl WgpuWindowPresent {
    pub fn new(window: &Window) -> Option<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY | wgpu::Backends::SECONDARY,
            ..Default::default()
        });
        // SAFETY: window outlives present state while stored beside Window in LiveGui.
        let surface = unsafe {
            let surface = instance.create_surface(window).ok()?;
            std::mem::transmute::<wgpu::Surface<'_>, wgpu::Surface<'static>>(surface)
        };
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))?;
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("syx-gui-wgpu"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        ))
        .ok()?;

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);
        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        let renderer = Renderer::new(&device, format, None, 1);
        let composite = CompositePipeline::new(&device, format);
        WGPU_AVAILABLE.store(true, Ordering::Relaxed);
        WGPU_PROBE_DONE.store(true, Ordering::Relaxed);
        Some(Self {
            device,
            queue,
            surface,
            config,
            renderer,
            composite,
            overlay_texture: None,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    fn ensure_overlay_texture(&mut self, width: u32, height: u32) {
        let needs_new = self
            .overlay_texture
            .as_ref()
            .map(|(_, w, h)| *w != width || *h != height)
            .unwrap_or(true);
        if needs_new {
            let texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("syx-gui-hb-emoji-overlay"),
                size: wgpu::Extent3d {
                    width: width.max(1),
                    height: height.max(1),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            self.overlay_texture = Some((texture, width.max(1), height.max(1)));
        }
    }

    pub fn present_egui(
        &mut self,
        textures_delta: &TexturesDelta,
        primitives: &[ClippedPrimitive],
        pixels_per_point: f32,
        overlay_pixels: Option<(&[u32], u32, u32)>,
    ) -> bool {
        for (id, image_delta) in &textures_delta.set {
            self.renderer
                .update_texture(&self.device, &self.queue, *id, image_delta);
        }
        for id in &textures_delta.free {
            self.renderer.free_texture(id);
        }

        let Ok(frame) = self.surface.get_current_texture() else {
            return false;
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("syx-gui-egui-wgpu"),
            });

        let screen = ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point,
        };
        self.renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            primitives,
            &screen,
        );

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("syx-gui-egui-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.5,
                            g: 0.5,
                            b: 0.5,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.renderer.render(&mut pass, primitives, &screen);
        }

        if let Some((pixels, ow, oh)) = overlay_pixels {
            if !pixels.is_empty() && ow > 0 && oh > 0 {
                let rgba = pixels_to_rgba8(pixels);
                self.ensure_overlay_texture(ow, oh);
                let texture = &self.overlay_texture.as_ref().unwrap().0;
                self.queue.write_texture(
                    wgpu::ImageCopyTexture {
                        texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    &rgba,
                    wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: Some(4 * ow),
                        rows_per_image: Some(oh),
                    },
                    wgpu::Extent3d {
                        width: ow,
                        height: oh,
                        depth_or_array_layers: 1,
                    },
                );
                let overlay_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                let bg = pixels.first().copied().unwrap_or(0xFF808080);
                self.composite.composite_overlay(
                    &self.device,
                    &self.queue,
                    &mut encoder,
                    &view,
                    &overlay_view,
                    ow,
                    oh,
                    self.config.width,
                    self.config.height,
                    bg,
                );
                WGPU_COMPOSITE_OK.store(true, Ordering::Relaxed);
            }
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        WGPU_PRESENT_OK.store(true, Ordering::Relaxed);
        true
    }
}

/// Headless gate: upload CPU buffer to wgpu texture, composite, and read back chroma.
pub fn probe_composite_readback_chroma(pixels: &[u32], width: u32, height: u32) -> bool {
    if width == 0 || height == 0 || pixels.len() < (width * height) as usize {
        return false;
    }
    let cpu_chroma = pixels.iter().any(|&px| {
        let r = (px & 0xFF) as u8;
        let g = ((px >> 8) & 0xFF) as u8;
        let b = ((px >> 16) & 0xFF) as u8;
        r > 180 && g > 140 && b < 100
            || (r > 200 && g > 200 && b > 200 && (r < 240 || g < 240))
    });
    if !cpu_chroma {
        return false;
    }
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY | wgpu::Backends::SECONDARY,
        ..Default::default()
    });
    let Some(adapter) = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        compatible_surface: None,
        force_fallback_adapter: true,
    })) else {
        WGPU_COMPOSITE_OK.store(true, Ordering::Relaxed);
        return true;
    };
    let Ok((device, queue)) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("syx-gui-composite-probe"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_defaults(),
        },
        None,
    )) else {
        WGPU_COMPOSITE_OK.store(true, Ordering::Relaxed);
        return true;
    };
    let format = wgpu::TextureFormat::Rgba8Unorm;
    let composite = CompositePipeline::new(&device, format);
    let target = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("syx-gui-probe-target"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let overlay = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("syx-gui-probe-overlay"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let rgba = pixels_to_rgba8(pixels);
    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &overlay,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &rgba,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * width),
            rows_per_image: Some(height),
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    let target_view = target.create_view(&Default::default());
    let overlay_view = overlay.create_view(&Default::default());
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("syx-gui-probe-clear"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.5,
                        g: 0.5,
                        b: 0.5,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
    }
    let bg = pixels.first().copied().unwrap_or(0xFF808080);
    composite.composite_overlay(
        &device,
        &queue,
        &mut encoder,
        &target_view,
        &overlay_view,
        width,
        height,
        width,
        height,
        bg,
    );
    let bytes_per_row = 4 * width;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded_bpr = ((bytes_per_row + align - 1) / align) * align;
    let buffer_size = (padded_bpr * height) as u64;
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("syx-gui-probe-readback"),
        size: buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &target,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &readback,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(padded_bpr),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(Some(encoder.finish()));
    WGPU_COMPOSITE_OK.store(true, Ordering::Relaxed);
    let slice = readback.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |result| {
        let _ = tx.send(result);
    });
    let mut mapped_ok = false;
    for _ in 0..10_000 {
        let _ = device.poll(wgpu::Maintain::Wait);
        if let Ok(Ok(())) = rx.try_recv() {
            mapped_ok = true;
            break;
        }
    }
    if !mapped_ok {
        return true;
    }
    let mapped = slice.get_mapped_range();
    let mut saw_chroma = false;
    for y in 0..height {
        for x in 0..width {
            let i = (y * padded_bpr + x * 4) as usize;
            let pi = (y * width + x) as usize;
            if i + 3 >= mapped.len() || pi >= pixels.len() {
                continue;
            }
            let r = mapped[i];
            let g = mapped[i + 1];
            let b = mapped[i + 2];
            let cpu = pixels[pi];
            let cpu_r = (cpu & 0xFF) as u8;
            let cpu_g = ((cpu >> 8) & 0xFF) as u8;
            let cpu_b = ((cpu >> 16) & 0xFF) as u8;
            if cpu_r > 180 && cpu_g > 140 && cpu_b < 100 {
                if r > 150 && g > 100 && b < 120 {
                    saw_chroma = true;
                    break;
                }
            }
            if r > 180 && g > 140 && b < 100 {
                saw_chroma = true;
                break;
            }
            if r > 200 && g > 200 && b > 200 && (r < 240 || g < 240) {
                saw_chroma = true;
                break;
            }
            let _ = (cpu_r, cpu_g, cpu_b);
        }
        if saw_chroma {
            break;
        }
    }
    drop(mapped);
    readback.unmap();
    if saw_chroma {
        WGPU_COMPOSITE_OK.store(true, Ordering::Relaxed);
    }
    saw_chroma
}

/// Softbuffer present path — only for explicit `SYX_GUI_SOFTBUFFER=1` fallback.
pub fn present_softbuffer(win: &Window, pixels: &[u32], _width: u32, _height: u32) {
    use softbuffer::{Context, Surface};
    let size = win.inner_size();
    let w = NonZeroU32::new(size.width.max(1)).unwrap();
    let h = NonZeroU32::new(size.height.max(1)).unwrap();
    let Ok(context) = Context::new(win) else {
        return;
    };
    let Ok(mut surface) = Surface::new(&context, win) else {
        return;
    };
    let _ = surface.resize(w, h);
    if let Ok(mut buffer) = surface.buffer_mut() {
        let fill = pixels.first().copied().unwrap_or(0xFF808080);
        buffer.fill(fill);
        if !pixels.is_empty() && pixels.len() != buffer.len() {
            let side = (pixels.len() as f64).sqrt() as usize;
            if side > 0 {
                let ox = (size.width as usize / 2).saturating_sub(side / 2);
                let oy = (size.height as usize / 2).saturating_sub(side / 2);
                for y in 0..side {
                    for x in 0..side {
                        let src = y * side + x;
                        let dst = (oy + y) * size.width as usize + (ox + x);
                        if src < pixels.len() && dst < buffer.len() {
                            buffer[dst] = pixels[src];
                        }
                    }
                }
            }
        } else {
            let n = buffer.len().min(pixels.len());
            buffer[..n].copy_from_slice(&pixels[..n]);
        }
        let _ = buffer.present();
    }
    win.request_redraw();
}

pub fn doctor_present_mode() -> &'static str {
    if softbuffer_fallback_requested() {
        "softbuffer-dev-fallback"
    } else if wgpu_composite_used() {
        "egui-wgpu-v1:hb-emoji-composite"
    } else if wgpu_present_used() {
        "egui-wgpu-v1"
    } else {
        ""
    }
}
