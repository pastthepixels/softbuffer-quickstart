use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::Surface;
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

// Window properties
pub struct WindowProperties {
    width: u32,
    height: u32,
    title: String,
}

impl WindowProperties {
    pub fn new(width: u32, height: u32, title: &str) -> WindowProperties {
        WindowProperties {
            width,
            height,
            title: String::from(title),
        }
    }
    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }
}

// Softbuffer Window
pub struct SoftbufferWindow<T>
where
    T: FnMut(Rc<Window>) -> Box<[u32]>,
{
    window: Option<Rc<Window>>,
    loop_fn: T,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    properties: WindowProperties,
}

impl<T> ApplicationHandler for SoftbufferWindow<T>
where
    T: FnMut(Rc<Window>) -> Box<[u32]>,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let window = event_loop.create_window(
                Window::default_attributes()
                    .with_title(self.properties.get_title())
                    .with_inner_size(winit::dpi::LogicalSize::new(
                        self.properties.get_size().0,
                        self.properties.get_size().1,
                    )),
            );
            Rc::new(window.unwrap())
        };
        let context = softbuffer::Context::new(window.clone()).unwrap();
        self.window = Some(window.clone());
        self.surface = Some(softbuffer::Surface::new(&context, window.clone()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let window = self.window.clone().unwrap();
                let surface = self.surface.as_mut().unwrap();
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                let loop_buffer = (self.loop_fn)(window);

                buffer[..loop_buffer.len()].clone_from_slice(loop_buffer.as_ref());

                buffer.present().unwrap();

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

impl<T> SoftbufferWindow<T>
where
    T: FnMut(Rc<Window>) -> Box<[u32]>,
{
    pub fn new(loop_fn: T, properties: WindowProperties) -> SoftbufferWindow<T> {
        SoftbufferWindow {
            window: None,
            loop_fn,
            surface: None,
            properties,
        }
    }

    pub fn run(&mut self) -> Result<(), EventLoopError> {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self)
    }
}
