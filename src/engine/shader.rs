extern crate math;
extern crate opengl as gl;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::str;
use std::ffi::CString;
use std::ptr;

use self::math::mat4::Mat4;
use self::math::vec3::Vec3;
use self::math::vec4::Vec4;

use self::gl::types::*;

pub struct Shader {
    pub program: u32
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.program) } ;
    }
}

impl Shader {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Shader {
        unsafe {
            // load vshader from file
            let path = Path::new(vertex_shader_path);
            let display = path.display();
            let mut vertex_shader_file = match File::open(&path) {
                Ok(file) => file,
                Err(err) => panic!("Coudn't open {}: {}", display, Error::description(&err)),
            };

            let mut vertex_shader_src = String::new();
            match vertex_shader_file.read_to_string(&mut vertex_shader_src) {
                Ok(_) => { }
                Err(err) => panic!("Coudn't read_to_string {}: {}", display, Error::description(&err)),
            };

            // load fshader from file
            let path = Path::new(fragment_shader_path);
            let display = path.display();
            let mut fragment_shader_file = match File::open(&path) {
                Ok(file) => file,
                Err(err) => panic!("Coudn't open {}: {}", display, Error::description(&err)),
            };
            let mut fragment_shader_src = String::new();
            match fragment_shader_file.read_to_string(&mut fragment_shader_src) {
                Ok(_) => { }
                Err(err) => panic!("Coudn't read_to_string {}: {}", display, Error::description(&err)),
            };


            let vertex_shader = Shader::compile_shader(&vertex_shader_src[..], gl::VERTEX_SHADER);
            let fragment_shader = Shader::compile_shader(&fragment_shader_src[..], gl::FRAGMENT_SHADER);
            let program = Shader::link_program(vertex_shader, fragment_shader);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Shader { program: program }
        }
    }

    // bind
    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.program) } ;
    }

    pub fn unbind(&self) {
        unsafe { gl::UseProgram(0) } ;
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteProgram(self.program) } ;
    }

    // uniforms
    pub unsafe fn get_shader_location(&self, name: &str) -> GLint{
        gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr())
    }

    pub fn set_uniform_matrix4fv(&self, name: &str, matrix: Mat4) {
        unsafe {
            let location = self.get_shader_location(name);
            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
        }
    }

    pub fn set_uniform_1i(&self, name: &str, value: i32) {
        unsafe {
            let location = self.get_shader_location(name);
            gl::Uniform1i(location, value);
        }
    }

    pub fn set_uniform_1f(&self, name: &str, value: f32) {
        unsafe {
            let location = self.get_shader_location(name);
            gl::Uniform1f(location, value);
        }
    }

    pub fn set_uniform_3f(&self, name: &str, value: Vec3) {
        unsafe {
            let location = self.get_shader_location(name);
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }
    pub fn set_uniform_4f(&self, name: &str, value: Vec4) {
        unsafe {
            let location = self.get_shader_location(name);
            gl::Uniform4f(location, value.x, value.y, value.z, value.w);
        }
    }


    // TODO: set camera (projection, view)
    // TODO: set lights (directional, point)

}

impl Shader {
    unsafe fn compile_shader(src: &str, ty: GLenum) -> GLuint {
        let shader = gl::CreateShader(ty);

        // attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // get compilation status
        let mut status = gl::FALSE as GLint;

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(&buf).ok().expect("ShaderIngoLog not valid for utf8"));
        }
        shader
    }

    unsafe fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
        }
        program
    }

}
