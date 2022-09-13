use std::{ffi::CString, io::Error, mem, ptr, os::raw::c_void};

use gl::types::{GLfloat, GLsizei, GLsizeiptr};

pub(crate) struct Pipeline {
    program: u32,
    vao: u32
}

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

const TRIANGLE_VERTICES: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            program: 0,
            vao: 0
        }
    }

    pub fn init(&mut self) {
        unsafe {
            let (v, f) = self.create_shaders().unwrap();
            self.program = self.link_shaders(v, f);
            self.vao = self.bind_buffer();
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    unsafe fn create_shaders(&self) -> Result<(VertexShader, FragmentShader), Error> {
        let (vertex_shader, fragment_shader) = {
            let v = VertexShader::from(VERTEX_SHADER_SOURCE);
            let f = FragmentShader::from(FRAGMENT_SHADER_SOURCE);
            (v, f)
        };

        Ok((vertex_shader, fragment_shader))
    }

    unsafe fn link_shaders(&self, v: VertexShader, f: FragmentShader) -> u32 {
        let program = gl::CreateProgram();
        gl::AttachShader(program, v.shader);
        gl::AttachShader(program, f.shader);
        gl::LinkProgram(program);
        program
    }

    unsafe fn bind_buffer(&self) -> u32 {
        let (mut vbo, mut vao) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (TRIANGLE_VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &TRIANGLE_VERTICES[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);
        vao
    }
}

pub struct VertexShader {
    shader: u32,
}

impl From<&str> for VertexShader {
    fn from(source: &str) -> Self {
        unsafe {
            let shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(source.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(shader);
            VertexShader { shader }
        }
    }
}

impl Drop for VertexShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader);
        }
    }
}

pub struct FragmentShader {
    shader: u32,
}

impl From<&str> for FragmentShader {
    fn from(source: &str) -> Self {
        unsafe {
            let shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_vert = CString::new(source.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(shader);
            FragmentShader { shader }
        }
    }
}

impl Drop for FragmentShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader);
        }
    }
}
