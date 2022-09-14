use std::{ffi::CString, fs, path::Path, ptr, str::from_utf8};

use gl::types::{GLchar, GLint};


#[derive(Debug)]
pub enum ShaderType {
    FragmentShader,
    VertexShader,
}

pub struct Shader {
    pub shader_id: u32,
}

impl Shader {
    pub(crate) fn form_path(file_path: &Path, shader_type: ShaderType) -> Self {
        let contents = fs::read_to_string(file_path).unwrap();

        Shader::new(&contents, shader_type)
    }

    fn new(source: &str, shader_type: ShaderType) -> Self {
        let type_ = match shader_type {
            ShaderType::FragmentShader => gl::FRAGMENT_SHADER,
            _ => gl::VERTEX_SHADER,
        };
        unsafe {
            let shader_id = gl::CreateShader(type_);
            let c_str_vert = CString::new(source.as_bytes()).unwrap();
            gl::ShaderSource(shader_id, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(shader_id);
            check_compile_errors(shader_id, shader_type);
            Shader { shader_id }
        }
    }
    
}

unsafe fn check_compile_errors(shader: u32, type_: ShaderType) {
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(1024);
    info_log.set_len(1024 - 1);
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(
            shader,
            1024,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        println!(
            "ERROR::SHADER_COMPILATION_ERROR of type: {:?}\n{}\n \
                      -- --------------------------------------------------- -- ",
            type_,
            from_utf8(&info_log).unwrap()
        );
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader_id);
        }
    }
}
