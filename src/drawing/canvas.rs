use super::Color;
use glfw::Window;

pub struct Canvas;

impl Canvas {
    pub fn draw_background(&self, color: Color) {
        unsafe {
            gl::ClearColor(color.red, color.green, color.blue, color.alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

impl From<&mut Window> for Canvas {
    fn from(window: &mut Window) -> Self {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        Canvas
    }
}
