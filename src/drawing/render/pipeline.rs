use std::{mem, os::raw::c_void, path::Path, ptr};

use gl::types::GLfloat;

use super::{
    shader_manager::ShaderManager, texture::Texture, texture_manager::TextureManager,
    vertex_manager::VertexManager,
};

pub(crate) struct Pipeline {
    vao: u32,
    texture_manager: TextureManager,
}

const RECTANGLE_VERTICES: [f32; 24] = [
    // positions         // colors // texture coords
    0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.5, 1.0, // bottom right
    -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom left
    0.0, 0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // top
];

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            vao: 0,
            texture_manager: TextureManager::new(),
        }
    }

    pub fn init(&mut self) {
        unsafe {
            let shader_manager = ShaderManager::new(
                Path::new("./src/shaders/texture.vs"),
                Path::new("./src/shaders/texture.fs"),
            );
            shader_manager.use_program();
            self.vao = self.bind_buffer();
        }
    }

    unsafe fn bind_buffer(&self) -> u32 {
        VertexManager::new()
            .bind_vertex_buffer(&RECTANGLE_VERTICES)
            .enable_vertex_attri(8, 0, 3, ptr::null())
            .enable_vertex_attri(8, 1, 3, (3 * mem::size_of::<GLfloat>()) as *const c_void)
            .enable_vertex_attri(8, 2, 2, (6 * mem::size_of::<GLfloat>()) as *const c_void)
            .build()
    }

    pub fn render(&self) {
        unsafe {
            self.texture_manager.bind_texture();
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    pub(crate) fn load_image(&mut self, image_path: &Path) {
        let image = Texture::from(image_path);
        unsafe {
            self.texture_manager.generate_mipmap(
                image.width.try_into().unwrap(),
                image.height.try_into().unwrap(),
                &image.data,
            );
        }
    }
}
