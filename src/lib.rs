use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::Surface;
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

pub struct WindowProperties {
    pub width: u32,
    pub height: u32,
}

pub struct SoftbufferWindow<T>
where
    T: FnMut(WindowProperties) -> Box<[u32]>,
{
    window: Option<Rc<Window>>,
    loop_fn: T,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
}

impl<T> ApplicationHandler for SoftbufferWindow<T>
where
    T: FnMut(WindowProperties) -> Box<[u32]>,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let window = event_loop.create_window(Window::default_attributes());
            Rc::new(window.unwrap())
        };
        let context = softbuffer::Context::new(window.clone()).unwrap();
        self.window = Some(window.clone());
        self.surface = Some(softbuffer::Surface::new(&context, window.clone()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
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
                let loop_buffer = (self.loop_fn)(WindowProperties { width, height });
                for index in 0..(loop_buffer.len()) {
                    buffer[index as usize] = loop_buffer.as_ref()[index as usize];
                }

                buffer.present().unwrap();

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

impl<T> SoftbufferWindow<T>
where
    T: FnMut(WindowProperties) -> Box<[u32]>,
{
    pub fn new(loop_fn: T) -> SoftbufferWindow<T> {
        SoftbufferWindow {
            window: None,
            loop_fn,
            surface: None,
        }
    }

    pub fn run(&mut self) -> Result<(), EventLoopError> {
        let event_loop = EventLoop::new().unwrap();

        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        event_loop.set_control_flow(ControlFlow::Poll);

        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        event_loop.set_control_flow(ControlFlow::Wait);

        event_loop.run_app(self)
    }
}
