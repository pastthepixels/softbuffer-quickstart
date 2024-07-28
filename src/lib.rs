use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::Surface;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

/// Contains a few potential properties to set for a SoftbufferWindow when it is created.
pub struct WindowProperties {
    pub size: PhysicalSize<u32>,
    pub title: Box<str>,
}

impl Default for WindowProperties {
    fn default() -> WindowProperties {
        WindowProperties {
            size: PhysicalSize::new(800, 600),
            title: "Softbuffer window".into(),
        }
    }
}

impl WindowProperties {
    pub fn new(width: u32, height: u32, title: &str) -> WindowProperties {
        WindowProperties {
            size: PhysicalSize::new(width, height),
            title: title.into(),
        }
    }
}

/// Wrapper for Softbuffer and a Winit window
pub struct SoftbufferWindow<T>
where
    T: FnMut(Rc<Window>, &mut [u32]),
{
    window: Option<Rc<Window>>,
    loop_fn: T,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    properties: WindowProperties,
}

impl<T> ApplicationHandler for SoftbufferWindow<T>
where
    T: FnMut(Rc<Window>, &mut [u32]),
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let window = event_loop.create_window(
                Window::default_attributes()
                    .with_title(self.properties.title.clone())
                    .with_inner_size(self.properties.size),
            );
            Rc::new(window.unwrap())
        };
        let context = softbuffer::Context::new(window.clone()).unwrap();
        self.window = Some(window.clone());
        self.surface = Some(Surface::new(&context, window.clone()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                self.properties.size = new_size;
                let (width, height) = (new_size.width, new_size.height);
                self.surface
                    .as_mut()
                    .unwrap()
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();
            }
            WindowEvent::RedrawRequested => {
                let mut buffer = self.surface.as_mut().unwrap().buffer_mut().unwrap();
                (self.loop_fn)(self.window.clone().unwrap(), buffer.as_mut());
                buffer.present().unwrap();
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

impl<T> SoftbufferWindow<T>
where
    T: FnMut(Rc<Window>, &mut [u32]),
{
    /// Creates a new SoftbufferWindow.
    /// `loop_fn` will be called every time the window needs to redraw,
    /// and `properties` contains a WindowProperties instance that will be
    /// read when the window is created.
    pub fn new(loop_fn: T, properties: WindowProperties) -> SoftbufferWindow<T> {
        SoftbufferWindow {
            window: None,
            loop_fn,
            surface: None,
            properties,
        }
    }

    /// Runs a SoftbufferWindow event loop.
    pub fn run(&mut self) -> Result<(), EventLoopError> {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self)
    }
}
