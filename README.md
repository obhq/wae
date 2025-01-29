# Winit Aync Executor
[![Crates.io Version](https://img.shields.io/crates/v/wae)](https://crates.io/crates/wae)

WAE is an async executor that use Winit event loop to drive the future that is ready to run.

## Example

```rust
use std::error::Error;
use std::rc::Rc;
use wae::{Signal, WindowHandler, WinitWindow};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{DeviceId, ElementState, InnerSizeWriter, MouseButton};
use winit::window::{Window, WindowId};

fn main() {
    wae::run(run()).unwrap()
}

async fn run() {
    let attrs = Window::default_attributes();
    let win = Rc::new(MyWindow {
        win: wae::create_window(attrs).unwrap(),
        close_requested: Signal::default(),
    });

    wae::register_window(&win);

    win.close_requested.wait().await;
}

struct MyWindow {
    win: Window,
    close_requested: Signal<()>,
}

impl WinitWindow for MyWindow {
    fn id(&self) -> WindowId {
        self.win.id()
    }
}

impl WindowHandler for MyWindow {
    fn on_resized(&self, _: PhysicalSize<u32>) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    fn on_close_requested(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = self.close_requested.set(());
        Ok(())
    }

    fn on_focused(&self, _: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    fn on_cursor_moved(
        &self,
        _: DeviceId,
        _: PhysicalPosition<f64>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    fn on_cursor_left(&self, _: DeviceId) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    fn on_mouse_input(
        &self,
        _: DeviceId,
        _: ElementState,
        _: MouseButton,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    fn on_scale_factor_changed(
        &self,
        _: f64,
        _: InnerSizeWriter,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    fn on_redraw_requested(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
}
```

## Breaking changes

### 0.1 to 0.2

- `RuntimeError` was renamed to `Error`.

## License

Licensed under either of

- Apache License, Version 2.0
- MIT license

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.
