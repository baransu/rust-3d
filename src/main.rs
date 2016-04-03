extern crate gl;
extern crate glutin;
// extern crate nalgebra as na;

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;

use glutin::*;

use std::ffi::CString;
// use na::*;
// use na;

// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [
     0.0,  0.5,
     0.5, -0.5,
    -0.5, -0.5
];

// Shader sources
static VS_SRC: &'static str =
   "#version 330 core\n\
    layout (location = 0) in vec2 position;\n\
    // uniform mat4 model;\n\
    // uniform mat4 view;\n\
    uniform mat4 projection;\n\
    void main() {\n\
        gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static FS_SRC: &'static str =
   "#version 330 core\n\
    out vec4 color;
    void main() {\n\
        color = vec4(1.0, 0.0, 0.0, 1.0);\n\
    }";

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    unsafe {
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
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
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

fn main() {

    let window = Window::new().unwrap();

    // title resolution and stuff
    window.set_title("rust-3d");
    window.set_inner_size(640, 480);

    // It is essential to make the context current before calling `gl::load_with`.
    unsafe { window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    // TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Create GLSL shaders
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, mem::transmute(&VERTEX_DATA[0]), gl::STATIC_DRAW);

        // Use shader program
        gl::UseProgram(program);

        // Specify the layout of the vertex data
        // let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
    }

    'running: loop {
        for event in window.wait_events() {

            unsafe {
                // Clear the screen to black
                gl::ClearColor(1.0, 0.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);


                // let perspective_matrix: Persp3<GLfloat> = Persp3::new(1280.0/720.0, 60.0, 0.0, 100.0);
                // let model_matrix: Mat4<GLfloat> = Mat4::new_identity(1);
                // translate(&model_matrix, Vec3::new(6.0, 0.0, 0.0));
                // model_matrix.transformation(Vec3::new(-6.0, 0.0, 0.0));

                // uniform matrixes
                // let projection_location = gl::GetUniformLocation(program, CString::new("projection").unwrap().as_ptr());
                // // let model_location = gl::GetUniformLocation(program, CString::new("model").unwrap().as_ptr());
                // // let view_location = gl::GetUniformLocation(program, CString::new("view").unwrap().as_ptr());
                //
                // let left = 0.0;
                // let right = 640.0;
                // let top = 0.0;
                // let bottom = 480.0;
                // let near = 0.0;
                // let far = 100.0;
                //
                // let pm: [GLfloat; 16] = [
                // 2.0/(right - left), 0.0,                0.0,                (left + right)/(left - right),
                // 0.0,                2.0/(top - bottom), 0.0,                (bottom + top)/(bottom - top),
                // 0.0,                0.0,                2.0/(near - far),   (far + near)/(far - near),
                // 0.0,                0.0,                0.0,                1.0,
                // ];
                //
                // println!("{:?}", pm);
                //
                // gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, &pm[0]);

                // Draw a triangle from the 3 vertices
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }

            window.swap_buffers().unwrap();

            match event {
                glutin::Event::Closed => break'running,
                _ => (),
            }
        }
    }

    // Cleanup - OpenGL memory dealocation
    unsafe {
        gl::DeleteProgram(program);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}
