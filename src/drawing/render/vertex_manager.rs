use std::{mem, os::raw::c_void, path::Path, ptr};

use gl::types::{GLfloat, GLsizei, GLsizeiptr};

use crate::utils::logger::logcat;

pub struct VertexManager {
    vertex_array: u32,
}

impl VertexManager {
    // Vertex arrays
    pub fn new() -> Self {
        unsafe {
            // TODO: 不太理解为什么这里只需要一个 u32, Gen 和 Bind 是分两步的。
            let mut arrays: u32 = 0;
            gl::GenVertexArrays(1 /*count*/, &mut arrays);
            gl::BindVertexArray(arrays);
            logcat!(format!(r#"vertex_arrays: {}"#, arrays));
            Self {
                vertex_array: arrays,
            }
        }
    }

    pub unsafe fn bind_vertex_buffer(self, vertices: &[f32]) -> Self {
        let mut vbo: u32 = 0;
        let glfloat_size = mem::size_of::<GLfloat>();
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * glfloat_size) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );
        self
    }

    pub unsafe fn enable_vertex_attri(
        self,
        index: u32,
        size: i32,
        offset_pointer: *const c_void,
    ) -> Self {
        let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(index, size, gl::FLOAT, gl::FALSE, stride, offset_pointer);
        gl::EnableVertexAttribArray(index);
        self
    }

    pub unsafe fn build(self) -> u32 {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        self.vertex_array
    }
}
