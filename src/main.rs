extern crate gl;
extern crate glutin;
extern crate time;
// extern crate image;

extern crate cgmath;

// extern crate math;

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;

use glutin::*;

use cgmath::*;

use std::ffi::CString;

use std::fs::File;

// use math::mat4::Mat4;
// use math::vec3::Vec3;

static VERTEX_DATA: [GLfloat; 8 * 36] = [
                                //color
    -0.5, -0.5, -0.5,  0.0, 0.0, 1.0, 0.0, 0.0,
    0.5, -0.5, -0.5,  1.0, 0.0, 1.0, 0.0, 0.0,
    0.5,  0.5, -0.5,  1.0, 1.0, 1.0, 0.0, 0.0,
    0.5,  0.5, -0.5,  1.0, 1.0, 1.0, 0.0, 0.0,
    -0.5,  0.5, -0.5,  0.0, 1.0, 1.0, 0.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, 0.0, 1.0, 0.0, 0.0,

    -0.5, -0.5,  0.5,  0.0, 0.0, 1.0, 1.0, 0.0,
    0.5, -0.5,  0.5,  1.0, 0.0, 1.0, 1.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 1.0, 1.0, 1.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 1.0, 1.0, 1.0, 0.0,
    -0.5,  0.5,  0.5,  0.0, 1.0, 1.0, 1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, 0.0, 1.0, 1.0, 0.0,

    -0.5,  0.5,  0.5,  1.0, 0.0, 0.0, 1.0, 0.0,
    -0.5,  0.5, -0.5,  1.0, 1.0, 0.0, 1.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, 1.0, 0.0, 1.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, 1.0, 0.0, 1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, 0.0, 0.0, 1.0, 0.0,
    -0.5,  0.5,  0.5,  1.0, 0.0, 0.0, 1.0, 0.0,

    0.5,  0.5,  0.5,  1.0, 0.0, 0.0, 1.0, 1.0,
    0.5,  0.5, -0.5,  1.0, 1.0, 0.0, 1.0, 1.0,
    0.5, -0.5, -0.5,  0.0, 1.0, 0.0, 1.0, 1.0,
    0.5, -0.5, -0.5,  0.0, 1.0, 0.0, 1.0, 1.0,
    0.5, -0.5,  0.5,  0.0, 0.0, 0.0, 1.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 0.0, 0.0, 1.0, 1.0,

    -0.5, -0.5, -0.5,  0.0, 1.0, 1.0, 0.0, 1.0,
    0.5, -0.5, -0.5,  1.0, 1.0, 1.0, 0.0, 1.0,
    0.5, -0.5,  0.5,  1.0, 0.0, 1.0, 0.0, 1.0,
    0.5, -0.5,  0.5,  1.0, 0.0, 1.0, 0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0, 1.0, 0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0, 1.0, 0.0, 1.0,

    -0.5,  0.5, -0.5,  0.0, 1.0, 0.0, 0.0, 1.0,
    0.5,  0.5, -0.5,  1.0, 1.0, 0.0, 0.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 0.0, 0.0, 0.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 0.0, 0.0, 0.0, 1.0,
    -0.5,  0.5,  0.5,  0.0, 0.0, 0.0, 0.0, 1.0,
    -0.5,  0.5, -0.5,  0.0, 1.0, 0.0, 0.0, 1.0

];

// static VERTEX_DATA: [f32; 5 * 4] = [
//    // Positions          // Texture Coords
//     0.5,  0.5, 0.0,   1.0, 1.0, // Top Right
//     0.5, -0.5, 0.0,   1.0, 0.0, // Bottom Right
//    -0.5, -0.5, 0.0,   0.0, 0.0, // Bottom Left
//    -0.5,  0.5, 0.0,   0.0, 1.0  // Top Left
// ];

// static INDICES: [i32; 6] = [ // Note that we start from 0!
//    0, 1, 3, // First Triangle
//    1, 2, 3  // Second Triangle
// ];

// static VERTEX_DATA: [f32; 8 * 3]= [
//     -0.5, -0.5, 0.5,//Bottom Let
//      0.5, -0.5, 0.5, //Bottom Right
//      0.5, 0.5, 0.5,  //Top Right
//     -0.5, 0.5, 0.5, //Top let
//
//     -0.5, -0.5, -0.5,//Bottom Let
//      0.5, -0.5, -0.5, //Bottom Right
//      0.5, 0.5, -0.5,  //Top Right
//     -0.5, 0.5, -0.5 //Top let
// ];
//
// static INDICES: [u32; 36] = [ // Note that we start from 0!
//     0,1,2, 0,2,3,//front
//     0,3,7, 0,7,4,//Left
//     0,1,5, 0,5,4,//Bottom
//
//     6,7,4, 6,4,5,//Back
//     6,7,3, 6,3,2,//top
//     6,2,1, 6,1,5//right
// ];


// Shader sources
static VS_SRC: &'static str =
   "#version 330 core\n\
    layout (location = 0) in vec3 position;\n\
    layout (location = 1) in vec2 uvs;\n\
    layout (location = 2) in vec3 color;\n\
    uniform mat4 model;\n\
    uniform mat4 view;\n\
    uniform mat4 projection;\n\
    out vec3 col;\n\
    void main() {\n\
        gl_Position = projection * model * vec4(position, 1.0f);\n\
        col = color;\n\
    }";

static FS_SRC: &'static str =
   "#version 330 core\n\
    out vec4 color;
    in vec3 col;
    void main() {\n\
        color = vec4(col, 1.0);\n\
    }";

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;

fn main() {

    let window = WindowBuilder::new()
        .with_title("rust-3d".to_string())
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .with_vsync()
        // .with_gl(GlRequest::Specific(Api::OpenGl, (3 as u8, 3 as u8)))
        .build()
        .unwrap();

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
    // let mut ebo = 0;

    // custom matrix tests
    // let mat = Mat4x4::new_identity();
    // println!("{:?}", mat);

    // let texture_img = image::open("resources/groundD.png").expect("Opening image failed");

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        // gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (VERTEX_DATA.len() * mem::size_of::<f32>()) as GLsizeiptr, mem::transmute(&VERTEX_DATA[0]), gl::STATIC_DRAW);
        //
        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        // gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (INDICES.len() * mem::size_of::<u32>()) as GLsizeiptr, mem::transmute(&INDICES[0]), gl::STATIC_DRAW);
        //

        // Specify the layout of the vertex data
        // let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE as GLboolean, (8 * mem::size_of::<GLfloat>()) as i32, ptr::null());

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE as GLboolean, (8 * mem::size_of::<GLfloat>()) as i32, mem::transmute(3 * mem::size_of::<GLfloat>()));

        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE as GLboolean, (8 * mem::size_of::<GLfloat>()) as i32, mem::transmute(5 * mem::size_of::<GLfloat>()));

        gl::BindVertexArray(0);

        gl::Enable(gl::DEPTH_TEST);
        // gl::Enable(gl::CULL_FACE);
        // gl::FrontFace(gl::CW);
        // gl::CullFace(gl::FRONT_AND_BACK);
        // Use shader program
        gl::UseProgram(program);
    }

    let mut time = 0.0;
    'running: loop {

        time += 0.16;
        let ts = time::get_time();
        // println!("{:?}", ts.sec as f64);
        let angle: f64 = ts.sec as f64 + ts.nsec as f64/1000000000.0;
        // println!("{:?}", time);

        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // near - as big as posible (0.1)
            // far - as small as posible (100 - far and small enought)
            // let perspective_matrix = Mat4::new_perspective();
            let projection_matrix: Matrix4<f32> = perspective(deg(45.0), WIDTH/HEIGHT, 0.1, 100.0);
            // let projection_matrix: Matrix4<f32> = ortho(-WIDTH/2.0, WIDTH/2.0, -HEIGHT/2.0, HEIGHT/2.0, 0.1, 100.0);

            // opengl forward is -z;
            let object_pos = Vector3::new(0.0, 0.0, -5.0);

            // let view_matrix = Matrix4::identity();
            let view_matrix: Matrix4<f32> = Matrix4::look_at(Point3::new(0.0, 0.0, 1.0), Point3::new(object_pos.x, object_pos.y, object_pos.z), Vector3::new(0.0, 1.0, 0.0));

            // let translation = Matrix4::identity();
            let translation: Matrix4<f32> = Matrix4::from_translation(object_pos);
            let t: f32 = time/10.0;
            let quat: Quaternion<f32> = Rotation3::from_euler(rad(0.5 * t), rad(1.0 * t), rad(0.0));
            let rotation: Matrix4<f32> = Matrix4::from(quat);
            // let scale: Matrix4<f32> = Matrix4::from_scale((angle/10.0).sin() as f32);

            let model_matrix = translation * rotation;

            // println!("{:?}", model_matrix);

            // uniform matrixes
            let projection_location = gl::GetUniformLocation(program, CString::new("projection").unwrap().as_ptr());
            let model_location = gl::GetUniformLocation(program, CString::new("model").unwrap().as_ptr());
            let view_location = gl::GetUniformLocation(program, CString::new("view").unwrap().as_ptr());

            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, projection_matrix.as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, (model_matrix as Matrix4<f32>).as_ptr());
            gl::UniformMatrix4fv(view_location, 1, gl::FALSE, view_matrix.as_ptr());

            gl::BindVertexArray(vao);
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
            // // Draw a triangle from the 3 vertices
        }


        window.swap_buffers().unwrap();

        for event in window.poll_events() {
            match event {
                Event::Closed => break'running,
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
