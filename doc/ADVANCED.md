Advanced Introduction to `softbuffer-quickstart`
================================================
<small>a.k.a, "Well, what *more* can I do?"</small>

> [!NOTE]
> This assumes that you've read the basic introduction first. If you haven't taken a look at it--and you have time--I encourage you to do so!`

`softbuffer_quickstart`, at its core, actually provides utility functions for people who want to implement their own Winit windows. Let's work backwards. Say you want to implement your own Winit window. Then you'd certainly implement [`ApplicationHandler`](https://rust-windowing.github.io/winit/winit/application/trait.ApplicationHandler.html):

```rust
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
 
 
use softbuffer_quickstart::{init, WindowProperties};
 
struct MyWindow {}
 
impl ApplicationHandler for MyWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        todo!()
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        todo!()
    }
}
```

But wait! How does `softbuffer_quickstart` fit into this? Well, let's take a look at those TODOs.

## 1. Initializing `softbuffer_quickstart`

Initializing `softbuffer_quickstart` is simple. We'll call `init` with the given event loop and a set of initial window properties (remember `WindowProperties`?), which will give us a `RawWindow` and `RawSurface` to store ourselves:

```rust
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
 
 
use softbuffer_quickstart::{init, RawSurface, RawWindow, WindowProperties};
 
struct MyWindow {
    window: RawWindow,
    surface: RawSurface
}
 
impl ApplicationHandler for MyWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        (self.window, self.surface) = init(event_loop, &WindowProperties::default());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        todo!()
    }
}
```

## 2. Drawing to a window (and other handy functions)

Under `window_event` we're given free rein to handle anything. There are two handy functions that can make handling some events simpler:
- `softbuffer_quickstart::close(&event_loop, &event)` checks to see if a "close" event has been signalled, and closes the window if it has.
- `softbuffer_quickstart::resize(&event, &mut surface)` check to see if a "resize" event has been signalled, and resizes the underlying buffer.

We can of course omit these and implement those checks ourselves, if we'd like. (However, no one probably wants to unwrap and clone and get references a bunch of times.)

Everything else is stupid simple. We update the buffer how we want, and then call `redraw` once we're done.
- `softbuffer_quickstart::buffer_mut(surface: &mut RawSurface)` returns a mutable reference to a `RawSurface`. (Hey, just like in the basic tutorial!)
- `softbuffer_quickstart::redraw(window: &mut RawWindow, surface: &mut RawSurface)` presents the `RawSurface` to the screen, using the `RawWindow`.

The final piece of the puzzle is *running* the thing. All we have to do is call `softbuffer_quickstart::run()` with a mutable reference to our window.

So if we recreated the code in the basic introduction, it'd look like this:

```rust
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
 
 
use softbuffer_quickstart::{init, RawSurface, RawWindow, WindowProperties};

fn main() {
    let mut window = MyWindow {
        window: None,
        surface: None,
        size: (800, 600)
    };
    softbuffer_quickstart::run(&mut window).expect("Window can't run");
}
 
struct MyWindow {
    window: RawWindow,
    surface: RawSurface,
    size: (usize, usize)
}
 
impl ApplicationHandler for MyWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        (self.window, self.surface) = init(event_loop, &WindowProperties::default());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        softbuffer_quickstart::close(event_loop, &event);
        softbuffer_quickstart::resize(&event, &mut self.surface);
        
        match event {
            WindowEvent::Resized(size) => {
                self.size = (size.width as usize, size.height as usize);
            }
            WindowEvent::RedrawRequested => {
                let (width, height) = self.size;
                let mut buffer = softbuffer_quickstart::buffer_mut(&mut self.surface);
                for index in 0..(width * height) {
                    let y = index / width;
                    let x = index % width;
                    let red = x % 255;
                    let green = y % 255;
                    let blue = (255 - (red + green).min(255)) % 255;

                    buffer[index] = (blue | (green << 8) | (red << 16)).try_into().unwrap();
                }
                softbuffer_quickstart::redraw(&mut self.window, &mut self.surface);
            }
            _ => (),
        }
    }
}
```
