use std::{mem, os::raw::c_void, path::Path, ptr};

use gl::types::{GLfloat, GLsizei, GLsizeiptr};

use super::{
    shader_manager::ShaderManager,
};

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

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    unsafe fn bind_buffer(&self) -> u32 {
        let (mut vbo, mut vao) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // vertex buffer
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (RECTANGLE_VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &RECTANGLE_VERTICES[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * mem::size_of::<GLfloat>()) as *const c_void, // offset,
        );
        gl::EnableVertexAttribArray(1);

        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        // gl::BindVertexArray(0);

        vao
    }
}
