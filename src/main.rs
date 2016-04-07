extern crate glutin;
extern crate time;
extern crate image;

extern crate cgmath;

extern crate engine;

extern crate opengl as gl;

use gl::types::*;

use std::mem;
use std::ptr;
use std::str;
use std::cmp;

use glutin::*;

use cgmath::*;

use std::ffi::CString;

use std::fs::File;

use engine::shader::Shader;

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
    out vec2 texCoords;\n\
    void main() {\n\
        gl_Position = projection * model * vec4(position, 1.0f);\n\
        col = color;\n\
        texCoords = uvs;\n\
    }";

static FS_SRC: &'static str =
   "#version 330 core\n\
    out vec4 color;
    in vec3 col;
    in vec2 texCoords;\n\
    uniform sampler2D texture1;\n\
    uniform sampler2D texture2;\n\
    void main() {\n\
        vec2 uv = vec2(texCoords.x, 1.0 - texCoords.y);\n\
        color = mix(texture(texture1, uv), texture(texture2, uv), 0.25);\n\
        //vec4(col, 1.0);\n\
    }";

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

fn main() {

    let window = WindowBuilder::new()
        .with_title("rust-3d".to_string())
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .with_gl(GlRequest::Specific(Api::OpenGl, (3 as u8, 3 as u8)))
        // .with_multisampling(16)
        .with_vsync()
        .build()
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    unsafe { window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    // TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
    gl::load_with(|symbol| {
        // println!("{:?}", symbol);
        window.get_proc_address(symbol) as *const _
    });

    let mut vao = 0;
    let mut vbo = 0;
    // let mut ebo = 0;
    let mut texture_id1 = 0;
    let mut texture_id2 = 0;

    let shader = Shader::new("res/vshader.vert", "res/fshader.frag");
    shader.bind();

    unsafe {


        gl::Enable(gl::DEPTH_TEST);
        // gl::Enable(gl::CULL_FACE);
        // gl::FrontFace(gl::CW);
        // gl::CullFace(gl::FRONT_AND_BACK);

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

        // ############################################
        //                   TEXTURE1
        // ############################################
        gl::GenTextures(1, &mut texture_id1);
        gl::BindTexture(gl::TEXTURE_2D, texture_id1);

        // texture wrapping
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        // texture filtering
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let tex_data1 = image::open("res/ground_diffuse.png").expect("Opening image failed");
        let tex_data1 = tex_data1.to_rgb();

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            tex_data1.width() as i32,
            tex_data1.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            mem::transmute(&tex_data1.into_raw()[0])
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);

        let mut max_anisotropy = 0.0;
        gl::GetFloatv(gl::MAX_TEXTURE_MAX_ANISOTROPY_EXT, &mut max_anisotropy);

        if max_anisotropy > 4.0 {
            max_anisotropy = 4.0;
        }
        println!("max_anisotropy {:?}", max_anisotropy);

        gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MAX_ANISOTROPY_EXT, max_anisotropy);

        gl::BindTexture(gl::TEXTURE_2D, 0);

        // ############################################
        //                   TEXTURE2
        // ############################################

        gl::GenTextures(1, &mut texture_id2);
        gl::BindTexture(gl::TEXTURE_2D, texture_id2);

        // texture wrapping
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        // texture filtering
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let tex_data2 = image::open("res/rust_logo.png").expect("Opening image failed");
        let tex_data2 = tex_data2.to_rgb();

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            tex_data2.width() as i32,
            tex_data2.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            mem::transmute(&tex_data2.into_raw()[0])
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MAX_ANISOTROPY_EXT, max_anisotropy);

        gl::BindTexture(gl::TEXTURE_2D, 0);

    }

    let mut time = 0.0;
    'running: loop {

        // shader.bind();

        time += 0.16;
        // let ts = time::get_time();
        // println!("{:?}", ts.sec as f64);
        // let angle: f64 = ts.sec as f64 + ts.nsec as f64/1000000000.0;
        // println!("{:?}", time);

        unsafe {
            // shader.bind();

            // Clear the screen to black
            gl::ClearColor(56.0/255.0, 142.0/255.0, 60.0/255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // near - as big as posible (0.1)
            // far - as small as posible (100 - far and small enought)
            // let perspective_matrix = Mat4::new_perspective();
            let projection_matrix: Matrix4<f32> = perspective(deg(45.0), WIDTH/HEIGHT, 0.1, 100.0);
            // let projection_matrix: Matrix4<f32> = ortho(-WIDTH/2.0, WIDTH/2.0, -HEIGHT/2.0, HEIGHT/2.0, 0.1, 100.0);

            // opengl forward is -z;
            let object_pos = Vector3::new(0.0, 0.0, -3.0);

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

            shader.set_uniform_matrix4fv("projection", &projection_matrix);
            shader.set_uniform_matrix4fv("model", &(model_matrix as Matrix4<f32>));
            shader.set_uniform_matrix4fv("view", &view_matrix);


            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id1);
            shader.set_uniform_1i("texture1", 0);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture_id2);
            shader.set_uniform_1i("texture2", 1);

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
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}
