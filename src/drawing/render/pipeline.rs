use std::{mem, os::raw::c_void, path::Path, ptr};

use gl::types::GLfloat;

use crate::utils::logger::logcat;

use super::{shader_manager::ShaderManager, vertex_manager::VertexManager};

pub(crate) struct Pipeline {
    vao: u32,
}

const RECTANGLE_VERTICES: [f32; 18] = [
    // positions         // colors
    0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
    -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
    0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
];

impl Pipeline {
    pub fn new() -> Self {
        Pipeline { vao: 0 }
    }

    pub fn init(&mut self) {
        unsafe {
            let shader_manager = ShaderManager::new(
                Path::new("./src/shaders/hello.vs"),
                Path::new("./src/shaders/hello.fs"),
            );
            shader_manager.use_program();
            self.vao = self.bind_buffer();
        }
    }

    unsafe fn bind_buffer(&self) -> u32 {
        VertexManager::new()
            .bind_vertex_buffer(&RECTANGLE_VERTICES)
            .enable_vertex_attri(0, 3, ptr::null())
            .enable_vertex_attri(1, 3, (3 * mem::size_of::<GLfloat>()) as *const c_void)
            .build()
    }

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
