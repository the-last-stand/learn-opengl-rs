use std::{ffi::CString, ptr};

pub enum ShaderType {
    FragmentShader,
    VertexShader,
}

pub struct Shader {
    pub shader_id: u32,
}

impl Shader {
    pub fn new(source: &str, shader_type: ShaderType) -> Self {
        let type_ = match shader_type {
            ShaderType::FragmentShader => gl::FRAGMENT_SHADER,
            _ => gl::VERTEX_SHADER,
        };
        unsafe {
            let shader_id = gl::CreateShader(type_);
            let c_str_vert = CString::new(source.as_bytes()).unwrap();
            gl::ShaderSource(shader_id, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(shader_id);
            Shader { shader_id }
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader_id);
        }
    }
}