Softbuffer Quickstart
=====================
A wrapper that makes using Softbuffer as easy as using minifb.

Running the Softbuffer example in softbuffer-quickstart:
```rust
use softbuffer_quickstart::{SoftbufferWindow, WindowProperties};
use winit::event::WindowEvent;

fn main() {
    let mut window = SoftbufferWindow::new(WindowProperties::default());
    window
        .run(move |window, event| match event {
            WindowEvent::RedrawRequested => {
                let (width, height) = window.inner_size();
                let mut buffer = window.buffer_mut();
                for index in 0..(width * height) {
                    let y = index / width;
                    let x = index % width;
                    let red = x % 255;
                    let green = y % 255;
                    let blue = (x * y) % 255;

                    buffer[index] = (blue | (green << 8) | (red << 16)).try_into().unwrap();
                }
            }
            _ => (),
        })
        .expect("window can't run :(");
}
```


## Contributing
PRs are welcome! As with any of my other projects it might take a while for me to respond to issues/pull requests. I recommend not squashing your commits before you submit a PR as doing so makes it a bit harder to review your code.  
I'm looking for any ways to boost performance as much as possible while making the library simpler and more intuitive.

## Ideas:
- Adding icons to WindowProperties (probably good for new contributors)
