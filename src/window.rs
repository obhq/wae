use std::error::Error;
use std::path::PathBuf;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{
    AxisId, DeviceId, ElementState, Ime, InnerSizeWriter, KeyEvent, Modifiers, MouseButton,
    MouseScrollDelta, Touch, TouchPhase,
};
use winit::event_loop::AsyncRequestSerial;
use winit::window::{ActivationToken, Theme, WindowId};

/// Encapsulates winit window with window-specific logic.
///
/// The event loop will exit immediately if any method return an error.
pub trait WindowHandler: WinitWindow {
    fn on_activation_token_done(
        &self,
        serial: AsyncRequestSerial,
        token: ActivationToken,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = token;
        let _ = serial;
        Ok(())
    }

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

    fn on_modifiers_changed(&self, m: Modifiers) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = m;
        Ok(())
    }

    fn on_ime(&self, ev: Ime) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = ev;
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

    fn on_cursor_entered(&self, dev: DeviceId) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = dev;
        Ok(())
    }

    fn on_cursor_left(&self, dev: DeviceId) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = dev;
        Ok(())
    }

    fn on_mouse_wheel(
        &self,
        dev: DeviceId,
        delta: MouseScrollDelta,
        phase: TouchPhase,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = phase;
        let _ = delta;
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

    fn on_pinch_gesture(
        &self,
        dev: DeviceId,
        delta: f64,
        phase: TouchPhase,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = phase;
        let _ = delta;
        let _ = dev;
        Ok(())
    }

    fn on_pan_gesture(
        &self,
        dev: DeviceId,
        delta: PhysicalPosition<f32>,
        phase: TouchPhase,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = phase;
        let _ = delta;
        let _ = dev;
        Ok(())
    }

    fn on_double_tap_gesture(&self, dev: DeviceId) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = dev;
        Ok(())
    }

    fn on_rotation_gesture(
        &self,
        dev: DeviceId,
        delta: f32,
        phase: TouchPhase,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = phase;
        let _ = delta;
        let _ = dev;
        Ok(())
    }

    fn on_touchpad_pressure(
        &self,
        dev: DeviceId,
        pressure: f32,
        stage: i64,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = stage;
        let _ = pressure;
        let _ = dev;
        Ok(())
    }

    fn on_axis_motion(
        &self,
        dev: DeviceId,
        axis: AxisId,
        value: f64,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = value;
        let _ = axis;
        let _ = dev;
        Ok(())
    }

    fn on_touch(&self, data: Touch) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = data;
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

    fn on_theme_changed(&self, new: Theme) -> Result<(), Box<dyn Error + Send + Sync>> {
        let _ = new;
        Ok(())
    }

    fn on_occluded(&self, new: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
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
