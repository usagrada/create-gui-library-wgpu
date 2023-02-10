use wgpu_text::font::FontArc;
use winit::{event::WindowEvent, window::Window};

use crate::widgets::View;

pub(crate) struct RenderState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    brush: wgpu_text::TextBrush,
}

impl RenderState {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let font = FontArc::try_from_slice(include_bytes!("../fonts/mplus-2p-medium.otf")).unwrap();
        let mut brush = wgpu_text::BrushBuilder::using_font(font)
            /* .initial_cache_size((1024, 1024))) */ // use this to avoid resizing cache texture
            /* .with_depth_testing(true) */ // enable/disable depth testing
            .build(&device, &config);
        brush.resize_view(config.width as f32, config.height as f32, &queue);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            brush,
        }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.brush.resize_view(
                self.config.width as f32,
                self.config.height as f32,
                &self.queue,
            );
        }
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render<'a>(&mut self, app_view: &View<'a>) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }
        use wgpu_text::section::{Layout, Section, Text};

        let font_size = 32.0;
        for (index, widget) in app_view.root.children.iter().enumerate() {
            let widget_kind = widget.kind();
            if widget_kind == "app::widgets::Text" {
                let text = widget.to_string();
                let section = Section::default()
                    .add_text(Text::new(&text).with_scale(font_size))
                    .with_screen_position((0.0, index as f32 * font_size))
                    .with_layout(Layout::default());
                self.brush.queue(section);
            }
        }
        // self.brush.queue(&section);
        let text_buffer = self.brush.draw(&self.device, &view, &self.queue);

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.submit([text_buffer]);
        output.present();
        Ok(())
    }
}
