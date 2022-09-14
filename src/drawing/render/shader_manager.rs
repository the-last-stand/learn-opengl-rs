use std::{ffi::CStr, path::Path};

use cgmath::{Vector3, Matrix4, Matrix, Array};

use super::shader::{Shader, ShaderType};

pub struct ShaderManager {
    program: u32,
}
impl ShaderManager {
    pub(crate) fn new(vs_path: &Path, fs_path: &Path) -> Self {
        let (vs, fs) = {
            let v = Shader::form_path(vs_path, ShaderType::VertexShader);
            let f = Shader::form_path(fs_path, ShaderType::FragmentShader);
            (v, f)
        };
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs.shader_id);
            gl::AttachShader(program, fs.shader_id);
            gl::LinkProgram(program);
            ShaderManager { program }
        }
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.program);
    }
}

/// utility uniform functions
#[allow(unused)]
impl ShaderManager {
    /// ------------------------------------------------------------------------
    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.program, name.as_ptr()), value as i32);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.program, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.program, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_vector3(&self, name: &CStr, value: &Vector3<f32>) {
        gl::Uniform3fv(
            gl::GetUniformLocation(self.program, name.as_ptr()),
            1,
            value.as_ptr(),
        );
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_vec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.program, name.as_ptr()), x, y, z);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_mat4(&self, name: &CStr, mat: &Matrix4<f32>) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.program, name.as_ptr()),
            1,
            gl::FALSE,
            mat.as_ptr(),
        );
    }
}