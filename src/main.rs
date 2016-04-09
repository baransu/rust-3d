extern crate glutin;
extern crate time;
extern crate image;
extern crate rand;

// local
extern crate math;
extern crate engine;
extern crate opengl as gl;

use gl::types::*;

use std::mem;
use std::ptr;
// use std::str;
// use std::cmp;

use glutin::*;

use rand::Rng;

// use cgmath::*;

// use std::ffi::CString;
//
// use std::fs::File;


// local
use engine::shader::Shader;
use engine::texture::Texture;
use engine::transform::Transform;

use math::mat4::Mat4;
use math::vec3::Vec3;

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

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

fn main() {

    let window = WindowBuilder::new()
        .with_title("rust-3d".to_string())
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        // .with_gl(GlRequest::Specific(Api::OpenGl, (3 as u8, 3 as u8)))
        // .with_multisampling(16)
        .with_vsync()
        .build()
        .unwrap();

    // println!("Window creation ended");
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
    // let mut texture_id1 = 0;
    // let mut texture_id2 = 0;

    let shader = Shader::new("res/vshader.vert", "res/fshader.frag");
    shader.bind();

    let texture1 = Texture::new("res/ground_diffuse.png", 4.0);
    let texture2 = Texture::new("res/rust_logo.png", 4.0);

    let mut entities = Vec::new();

    for _ in 0..1000 {

        // x e<-5, 5>
        let pos_x = rand::thread_rng().gen_range(-5.0, 6.0);
        // y e<-5, 5>
        let pos_y = rand::thread_rng().gen_range(-5.0, 6.0);
        // z e<-10, 0>
        let pos_z = rand::thread_rng().gen_range(-5.0, 6.0);

        // rotaion e(1, 360)
        let rot_x = rand::thread_rng().gen_range(1.0, 360.0);
        let rot_y = rand::thread_rng().gen_range(1.0, 360.0);
        let rot_z = rand::thread_rng().gen_range(1.0, 360.0);

        // scale e<0.25, 1>
        let scale = rand::thread_rng().gen_range(0.25, 1.25);

        entities.push(Transform::new(Vec3::new(pos_x, pos_y, pos_z), Vec3::new(rot_x , rot_y, rot_z), Vec3::new(scale, scale, scale)));
    }

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
            gl::ClearColor(56.0/255.0, 142.0/255.0, 60.0/255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // near - as big as posible (0.1)
            // far - as small as posible (100 - far and small enought)
            let projection_matrix = Mat4::from_perspective(45.0, WIDTH/HEIGHT, 0.1, 100.0);

            let radius = 20.0;
            let cam_x = angle.cos() * radius;
        	let cam_z = angle.sin() * radius;

            // opengl forward is -z;
            let view_matrix = Mat4::from_look_at(&Vec3::new(cam_x as f32, 0.0, cam_z as f32), &Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 1.0, 0.0));

            texture1.bind(gl::TEXTURE0);
            shader.set_uniform_1i("texture1", 0);

            texture2.bind(gl::TEXTURE1);
            shader.set_uniform_1i("texture2", 1);

            shader.set_uniform_matrix4fv("projection", projection_matrix);
            shader.set_uniform_matrix4fv("view", view_matrix);

            gl::BindVertexArray(vao);

            for entity in &mut entities {

                entity.rotation.x += 10.0 * 0.16;
                entity.rotation.z += 5.0 * 0.16;

                shader.set_uniform_matrix4fv("model", entity.get_model_matrix());
                // Draw cube
                gl::DrawArrays(gl::TRIANGLES, 0, 36);

            }

            gl::BindVertexArray(0);

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
