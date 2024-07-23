use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

struct SoftbufferWindow<T>
where
    T: FnMut() -> (),
{
    window: Option<Window>,
    loop_fn: T,
}

impl<T> ApplicationHandler for SoftbufferWindow<T>
where
    T: FnMut() -> (),
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                (self.loop_fn)();
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

impl<T> SoftbufferWindow<T>
where
    T: FnMut() -> (),
{
    pub fn new(loop_fn: T) -> SoftbufferWindow<T> {
        SoftbufferWindow {
            window: None,
            loop_fn,
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

// TODO: DELETEME
fn main() {
    let mut x = 0;
    let mut window = SoftbufferWindow::new(|| {
        x += 1;
        println!("{}", x);
    });
    window.run().expect("damn! :(");
}
