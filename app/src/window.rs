use winit::event::{Event, WindowEvent};

use crate::{render::RenderState, widgets::View};

pub struct Window {
    window: winit::window::Window,
    state: Option<RenderState>,
    event_loop: winit::event_loop::EventLoop<()>,
}

impl Window {
    pub(crate) fn new() -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();
        Self {
            window,
            event_loop,
            state: None,
        }
    }

    pub fn add_state(&mut self) {
        pollster::block_on(async {
            let state = RenderState::new(&self.window).await;
            self.state = Some(state);
        });
    }
    pub(crate) fn run(mut self, app_view: View<'static>) -> ! {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Wait;
            if self.state.is_none() {
                return;
            }
            match event {
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => *control_flow = winit::event_loop::ControlFlow::Exit,
                winit::event::Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::Resized(physical_size) => {
                            let state= self.state.as_mut();
                            let state = state.unwrap();
                            state.resize(physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            let state= self.state.as_mut();
                            let state = state.unwrap();
                            state.resize(*new_inner_size);
                        }
                        _ => (),
                    }
                }
                Event::RedrawRequested(_) => {
                    let state= self.state.as_mut();
                    let state = state.unwrap();
                    state.update();
                    match state.render(&app_view) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = winit::event_loop::ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                _ => (),
            }
        })
    }
    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    pub fn set_size(&self, width: u32, height: u32) {
        self.window
            .set_inner_size(winit::dpi::PhysicalSize::new(width, height));
    }
}
