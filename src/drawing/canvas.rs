use std::path::Path;

use super::{pipeline::Pipeline, Color};
use glfw::Window;

pub struct Canvas {
    pipeline: Pipeline,
}

/// 新增一个绘制流程的话，除了调整对应的 shader，还需要调整对应的 buffer 的结构
impl Canvas {
    pub fn draw_background(&self, color: Color) {
        unsafe {
            gl::ClearColor(color.red, color.green, color.blue, color.alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_triangle(&self) {
        self.pipeline.render();
    }

    pub fn draw_image(&mut self, image_path: &Path) {
        self.pipeline.load_image(image_path);
        self.pipeline.render();
    }
}

impl From<&mut Window> for Canvas {
    fn from(window: &mut Window) -> Self {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        let mut pipeline = Pipeline::new();
        pipeline.init();
        Canvas { pipeline }
    }
}
