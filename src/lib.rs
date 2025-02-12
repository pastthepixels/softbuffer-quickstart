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
    /// Initial width
    pub width: u32,
    /// Initial height
    pub height: u32,
    /// Title
    pub title: &'static str,
}

impl Default for WindowProperties {
    fn default() -> WindowProperties {
        WindowProperties {
            width: 800,
            height: 600,
            title: "Softbuffer Window",
        }
    }
}

/// Wrapper for Softbuffer and a Winit window
pub struct SoftbufferWindow {
    window: Option<Rc<Window>>,
    loop_fn: Option<Box<dyn FnMut(&mut SoftbufferWindow, WindowEvent)>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    properties: WindowProperties,
}

impl ApplicationHandler for SoftbufferWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let window = event_loop.create_window(
                Window::default_attributes()
                    .with_title(self.properties.title)
                    .with_inner_size(PhysicalSize::new(
                        self.properties.width,
                        self.properties.height,
                    )),
            );
            Rc::new(window.unwrap())
        };
        let context = softbuffer::Context::new(window.clone()).unwrap();
        self.window = Some(window.clone());
        self.surface = Some(Surface::new(&context, window.clone()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        // Automatic event handling
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.surface
                    .as_mut()
                    .unwrap()
                    .resize(
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    )
                    .unwrap();
            }
            _ => (),
        }

        let is_redraw_requested = event == WindowEvent::RedrawRequested;

        // User event handling
        if self.loop_fn.is_some() {
            let mut loop_fn = self.loop_fn.take().unwrap();
            loop_fn(self, event);
            self.loop_fn = Some(loop_fn);
        }

        // Displays buffer automatically if event is RedrawRequested
        if is_redraw_requested {
            self.surface
                .as_mut()
                .unwrap()
                .buffer_mut()
                .unwrap()
                .present()
                .unwrap();
            self.window.as_ref().unwrap().request_redraw();
        }
    }
}

impl SoftbufferWindow {
    /// Creates a new SoftbufferWindow.
    /// Example usage:
    /// ```rust
    /// let window = SoftbufferWindow::new(WindowProperties::default());
    /// ```
    pub fn new(properties: WindowProperties) -> Self {
        SoftbufferWindow {
            window: None,
            loop_fn: None,
            surface: None,
            properties,
        }
    }

    /// Runs a SoftbufferWindow event loop.
    /// To handle events, you need winit's `WindowEvent` enum.
    /// Example usage:
    /// ```rust
    /// window.run(move |window, event| {
    ///     match event {
    ///         WindowEvent::RedrawRequested => (),
    ///         _ => ()
    ///     }
    /// });
    /// ```
    pub fn run(
        &mut self,
        event_fn: impl FnMut(&mut SoftbufferWindow, WindowEvent) + 'static,
    ) -> Result<(), EventLoopError> {
        self.loop_fn = Some(Box::new(event_fn));
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self)
    }

    /// Returns the size of a window as a tuple
    pub fn inner_size(&mut self) -> (usize, usize) {
        let size = self.window.clone().unwrap().inner_size();
        (size.width as usize, size.height as usize)
    }

    /// Gets a mutable reference to the buffer
    pub fn buffer_mut(&mut self) -> softbuffer::Buffer<'_, Rc<Window>, Rc<Window>> {
        self.surface.as_mut().unwrap().buffer_mut().unwrap()
    }
}
