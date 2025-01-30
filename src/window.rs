use std::error::Error;
use std::path::PathBuf;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{DeviceId, ElementState, InnerSizeWriter, KeyEvent, MouseButton};
use winit::window::WindowId;

/// Encapsulates winit window with window-specific logic.
///
/// The event loop will exit immediately if any method return an error.
pub trait WindowHandler: WinitWindow {
    fn on_resized(&self, new: PhysicalSize<u32>) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = new;
        Ok(())
    }

    fn on_moved(&self, new: PhysicalPosition<i32>) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = new;
        Ok(())
    }

    fn on_close_requested(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    fn on_dropped_file(&self, path: PathBuf) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = path;
        Ok(())
    }

    fn on_hovered_file(&self, path: PathBuf) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = path;
        Ok(())
    }

    fn on_hovered_file_cancelled(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    fn on_focused(&self, gained: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = gained;
        Ok(())
    }

    fn on_keyboard_input(
        &self,
        dev: DeviceId,
        event: KeyEvent,
        is_synthetic: bool,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = is_synthetic;
        let _ = event;
        let _ = dev;
        Ok(())
    }

    fn on_cursor_moved(
        &self,
        dev: DeviceId,
        pos: PhysicalPosition<f64>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = pos;
        let _ = dev;
        Ok(())
    }

    fn on_cursor_left(&self, dev: DeviceId) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = dev;
        Ok(())
    }

    fn on_mouse_input(
        &self,
        dev: DeviceId,
        st: ElementState,
        btn: MouseButton,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = btn;
        let _ = st;
        let _ = dev;
        Ok(())
    }

    fn on_scale_factor_changed(
        &self,
        new: f64,
        sw: InnerSizeWriter,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = sw;
        let _ = new;
        Ok(())
    }

    fn on_redraw_requested(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
}

/// Provides method to return winit properties.
pub trait WinitWindow {
    fn id(&self) -> WindowId;
}
