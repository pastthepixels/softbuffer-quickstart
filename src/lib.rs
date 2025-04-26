mod sb_window;

use std::num::NonZeroU32;
pub use sb_window::SoftbufferWindow;

use std::rc::Rc;

use softbuffer::Surface;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window};

pub type RawWindow = Option<Rc<Window>>;
pub type RawSurface = Option<Surface<Rc<Window>, Rc<Window>>>;

/// Simple struct that holds some properties (size, title) for windows
pub struct WindowProperties {
    /// Initial width
    pub width: u32,
    /// Initial height
    pub height: u32,
    /// Title
    pub title: &'static str,
}

impl Default for WindowProperties {
    /// Creates an 800x600 window named "Softbuffer Window"
    fn default() -> WindowProperties {
        WindowProperties {
            width: 800,
            height: 600,
            title: "Softbuffer Window",
        }
    }
}

/// Shorthand to run a struct that implements `ApplicationHandler`
pub fn run<A: ApplicationHandler<()>>(window: &mut A) -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(window)
}

/// Initialises and returns a new RawWindow and RawSurface given an `ActiveEventLoop` and `WindowProperties`.
/// For instance, implementation within `ApplicationHandler::resumed` may look like:
/// ```rust
/// //...
/// fn resumed(&mut self, event_loop: &ActiveEventLoop) {
///     (self.window, self.surface) = init(event_loop, &self.properties);
/// }
/// //...
/// ```
pub fn init(event_loop: &ActiveEventLoop, properties: &WindowProperties) -> (RawWindow, RawSurface) {
    let window = {
        let window = event_loop.create_window(
            Window::default_attributes()
                .with_title(properties.title)
                .with_inner_size(PhysicalSize::new(
                    properties.width,
                    properties.height,
                )),
        );
        Rc::new(window.unwrap())
    };
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let window : RawWindow = Some(window.clone());
    let surface : RawSurface = Some(Surface::new(&context, window.clone().unwrap()).unwrap());
    (window, surface)
}

/// Shorthand to listen for and handle WindowEvent::CloseRequested by closing windows
pub fn close(event_loop: &ActiveEventLoop, event: &WindowEvent) {
    if let WindowEvent::CloseRequested = event {
        event_loop.exit();
    }
}

/// Shorthand to listen for and handle WindowEvent::Resized by resizing a buffer
pub fn resize(event: &WindowEvent, surface: &mut RawSurface) {
    if let WindowEvent::Resized(size) = event {
        surface
            .as_mut()
            .unwrap()
            .resize(
                NonZeroU32::new(size.width).unwrap(),
                NonZeroU32::new(size.height).unwrap(),
            )
            .unwrap();
    }
}

/// Redraws a RawSurface. Call this on `Window::RedrawRequested`.
pub fn redraw(window: &mut RawWindow, surface: &mut RawSurface) {
    surface
        .as_mut()
        .unwrap()
        .buffer_mut()
        .unwrap()
        .present()
        .unwrap();
    window.as_ref().unwrap().request_redraw();
}

/// Gets a mutable reference to a buffer from a `RawSurface`
pub fn buffer_mut(surface: &mut RawSurface) -> softbuffer::Buffer<'_, Rc<Window>, Rc<Window>> {
    surface.as_mut().unwrap().buffer_mut().unwrap()
}