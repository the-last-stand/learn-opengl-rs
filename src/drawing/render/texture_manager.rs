use std::{convert::TryInto, ffi::c_void};

pub struct TextureManager {
    texture: u32,
}

impl TextureManager {
    // set texture filtering mode: NEAREST or LINEAR
    pub fn new() -> Self {
        Self { texture: 0 }
    }

    // generate mipmap
    pub unsafe fn generate_mipmap(&mut self, width: i32, height: i32, data: &Vec<u8>) {
        let mut texture: u32 = 0;
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

         // set the texture wrapping parameters
         gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::MIRRORED_REPEAT as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::MIRRORED_REPEAT as i32,
        );
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB.try_into().unwrap(),
            width,
            height,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        self.texture = texture;
    }

    pub unsafe fn bind_texture(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.texture);
    }

    // apply texture: bindTexture
    // shader use a new keyword called `sampler2D`, and texture function to
    // read color value from texture.
}
