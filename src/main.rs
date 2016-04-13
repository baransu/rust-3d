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
use engine::model:: { Vertex, Mod };
use engine::camera::Camera;

use math::mat4::Mat4;
use math::vec3::Vec3;
use math::vec2::Vec2;

// static VERTEX_DATA: [GLfloat; 8 * 36] = [
//                                 //color
//     -0.5, -0.5, -0.5,  0.0, 0.0, 1.0, 0.0, 0.0,
//     0.5, -0.5, -0.5,  1.0, 0.0, 1.0, 0.0, 0.0,
//     0.5,  0.5, -0.5,  1.0, 1.0, 1.0, 0.0, 0.0,
//     0.5,  0.5, -0.5,  1.0, 1.0, 1.0, 0.0, 0.0,
//     -0.5,  0.5, -0.5,  0.0, 1.0, 1.0, 0.0, 0.0,
//     -0.5, -0.5, -0.5,  0.0, 0.0, 1.0, 0.0, 0.0,
//
//     -0.5, -0.5,  0.5,  0.0, 0.0, 1.0, 1.0, 0.0,
//     0.5, -0.5,  0.5,  1.0, 0.0, 1.0, 1.0, 0.0,
//     0.5,  0.5,  0.5,  1.0, 1.0, 1.0, 1.0, 0.0,
//     0.5,  0.5,  0.5,  1.0, 1.0, 1.0, 1.0, 0.0,
//     -0.5,  0.5,  0.5,  0.0, 1.0, 1.0, 1.0, 0.0,
//     -0.5, -0.5,  0.5,  0.0, 0.0, 1.0, 1.0, 0.0,
//
//     -0.5,  0.5,  0.5,  1.0, 0.0, 0.0, 1.0, 0.0,
//     -0.5,  0.5, -0.5,  1.0, 1.0, 0.0, 1.0, 0.0,
//     -0.5, -0.5, -0.5,  0.0, 1.0, 0.0, 1.0, 0.0,
//     -0.5, -0.5, -0.5,  0.0, 1.0, 0.0, 1.0, 0.0,
//     -0.5, -0.5,  0.5,  0.0, 0.0, 0.0, 1.0, 0.0,
//     -0.5,  0.5,  0.5,  1.0, 0.0, 0.0, 1.0, 0.0,
//
//     0.5,  0.5,  0.5,  1.0, 0.0, 0.0, 1.0, 1.0,
//     0.5,  0.5, -0.5,  1.0, 1.0, 0.0, 1.0, 1.0,
//     0.5, -0.5, -0.5,  0.0, 1.0, 0.0, 1.0, 1.0,
//     0.5, -0.5, -0.5,  0.0, 1.0, 0.0, 1.0, 1.0,
//     0.5, -0.5,  0.5,  0.0, 0.0, 0.0, 1.0, 1.0,
//     0.5,  0.5,  0.5,  1.0, 0.0, 0.0, 1.0, 1.0,
//
//     -0.5, -0.5, -0.5,  0.0, 1.0, 1.0, 0.0, 1.0,
//     0.5, -0.5, -0.5,  1.0, 1.0, 1.0, 0.0, 1.0,
//     0.5, -0.5,  0.5,  1.0, 0.0, 1.0, 0.0, 1.0,
//     0.5, -0.5,  0.5,  1.0, 0.0, 1.0, 0.0, 1.0,
//     -0.5, -0.5,  0.5,  0.0, 0.0, 1.0, 0.0, 1.0,
//     -0.5, -0.5, -0.5,  0.0, 1.0, 1.0, 0.0, 1.0,
//
//     -0.5,  0.5, -0.5,  0.0, 1.0, 0.0, 0.0, 1.0,
//     0.5,  0.5, -0.5,  1.0, 1.0, 0.0, 0.0, 1.0,
//     0.5,  0.5,  0.5,  1.0, 0.0, 0.0, 0.0, 1.0,
//     0.5,  0.5,  0.5,  1.0, 0.0, 0.0, 0.0, 1.0,
//     -0.5,  0.5,  0.5,  0.0, 0.0, 0.0, 0.0, 1.0,
//     -0.5,  0.5, -0.5,  0.0, 1.0, 0.0, 0.0, 1.0
//
// ];

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

fn main() {

    let window = WindowBuilder::new()
        .with_title("rust-3d".to_string())
        // .with_fullscreen(get_primary_monitor())
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        // .with_gl(GlRequest::Specific(Api::OpenGl, (3 as u8, 3 as u8)))
        // .with_multisampling(16)
        .with_vsync()
        .build()
        .unwrap();

    // window.set_cursor_position(WIDTH as i32/2, HEIGHT as i32/2);

    // It is essential to make the context current before calling `gl::load_with`.
    unsafe { window.make_current() }.unwrap();
    // Load the OpenGL function pointers
    // TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
    gl::load_with(|symbol| {
        // println!("{:?}", symbol);
        window.get_proc_address(symbol) as *const _
    });

    // input stuff
    let mut pressed_keys: [bool; 1024] = [false; 1024];

    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 20.0), Vec3::new(0.0, 0.0, -90.0));

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    let shader = Shader::new("res/vshader.vert", "res/fshader.frag");
    shader.bind();

    let normal_map = Texture::new("res/mouse/mouseNormal.png", 4.0);
    let diffuse_map = Texture::new("res/mouse/mouseAlbedo.png", 4.0);
    let specular_map = Texture::new("res/mouse/mouseRoughness.png", 4.0);

    let mut entities = Vec::new();

    // let model = Mod::new("res/susanne_lowpoly.obj");
    // let model = Mod::new("res/susanne_highpoly.obj");
    let model = Mod::new("res/mouse/mouselowpoly.obj");

    // for _ in 0..1 {
    //
    //     // x e<-5, 5>
    //     let pos_x = rand::thread_rng().gen_range(-5.0, 6.0);
    //     // y e<-5, 5>
    //     let pos_y = rand::thread_rng().gen_range(-5.0, 6.0);
    //     // z e<-10, 0>
    //     let pos_z = rand::thread_rng().gen_range(-5.0, 6.0);
    //
    //     // rotaion e(1, 360)
    //     let rot_x = rand::thread_rng().gen_range(1.0, 360.0);
    //     let rot_y = rand::thread_rng().gen_range(1.0, 360.0);
    //     let rot_z = rand::thread_rng().gen_range(1.0, 360.0);
    //
    //     // scale e<0.25, 1>
    //     let scale = rand::thread_rng().gen_range(0.25, 1.25);
    //
    //     entities.push(Transform::new(Vec3::new(pos_x, pos_y, pos_z), Vec3::new(rot_x , rot_y, rot_z), Vec3::new(scale, scale, scale)));
    // }

    entities.push(Transform::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)));

    unsafe {


        gl::Enable(gl::DEPTH_TEST);
        // gl::Enable(gl::CULL_FACE);
        // gl::FrontFace(gl::CW);
        // gl::CullFace(gl::FRONT_AND_BACK);

        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (model.vertices.len() * mem::size_of::<Vertex>()) as GLsizeiptr, mem::transmute(&model.vertices[0]), gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (model.indices.len() * mem::size_of::<u32>()) as GLsizeiptr, mem::transmute(&model.indices[0]), gl::STATIC_DRAW);

        // Specify the layout of the vertex data
        // let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
        println!("vertex size {:?} | 14f32 size {:?}", mem::size_of::<Vertex>(), 14 * mem::size_of::<GLfloat>());

        // pos
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE as GLboolean, mem::size_of::<Vertex>() as i32, ptr::null());
        // uvs
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE as GLboolean, mem::size_of::<Vertex>() as i32, mem::transmute(3 * mem::size_of::<f32>()));
        // normal
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE as GLboolean, mem::size_of::<Vertex>() as i32, mem::transmute(5 * mem::size_of::<f32>()));
        // tangent
        gl::EnableVertexAttribArray(3);
        gl::VertexAttribPointer(3, 3, gl::FLOAT, gl::FALSE as GLboolean, mem::size_of::<Vertex>() as i32, mem::transmute(8 * mem::size_of::<f32>()));
        // binormal
        gl::EnableVertexAttribArray(4);
        gl::VertexAttribPointer(4, 3, gl::FLOAT, gl::FALSE as GLboolean, mem::size_of::<Vertex>() as i32, mem::transmute(11 * mem::size_of::<f32>()));


        // gl::EnableVertexAttribArray(1);
        // gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE as GLboolean, (8 * mem::size_of::<GLfloat>()) as i32, mem::transmute(3 * mem::size_of::<GLfloat>()));
        //
        // gl::EnableVertexAttribArray(2);
        // gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE as GLboolean, (8 * mem::size_of::<GLfloat>()) as i32, mem::transmute(5 * mem::size_of::<GLfloat>()));

        // gl::BindVertexArray(0);

    }

    let mut time = 0.0;
    'running: loop {

        // process input
        input(&pressed_keys, &mut camera);

        time += 0.16;
        let ts = time::get_time();
        // println!("{:?}", ts.sec as f64);
        let angle: f64 = ts.sec as f64 + ts.nsec as f64/1000000000.0;
        // println!("{:?}", time);

        unsafe {

            // Clear the screen to black
            gl::ClearColor(66.0/255.0, 66.0/255.0, 66.0/255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // near - as big as posible (0.1)
            // far - as small as posible (100 - far and small enought)
            let projection_matrix = Mat4::from_perspective(45.0, WIDTH/HEIGHT, 0.1, 100.0);

            // opengl forward is -z;
            // let radius = 20.0;
            // camera.position.x = (angle.cos() * radius) as f32;
            // camera.position.z = (angle.sin() * radius) as f32;
            // let view_matrix = camera.get_look_at_target_matrix(Vec3::new(0.0, 0.0, 0.0));
            let view_matrix = camera.get_look_at_matrix();

            normal_map.bind(gl::TEXTURE0);
            shader.set_uniform_1i("normalMap", 0);

            diffuse_map.bind(gl::TEXTURE1);
            shader.set_uniform_1i("diffuseMap", 1);

            specular_map.bind(gl::TEXTURE2);
            shader.set_uniform_1i("specularMap", 2);


            shader.set_uniform_matrix4fv("projection", projection_matrix);
            shader.set_uniform_matrix4fv("view", view_matrix);

            let ligh_pos = Vec3::new(0.0, 5.0, 5.0);
            shader.set_uniform_3f("lightPos", ligh_pos);
            shader.set_uniform_3f("viewPos", camera.position);

            gl::BindVertexArray(vao);

            for entity in &mut entities {

                // entity.rotation.y += 5.0 * 0.16;
                // entity.rotation.z += 5.0 * 0.16;

                shader.set_uniform_matrix4fv("model", entity.get_model_matrix());
                // Draw cube
                // gl::DrawArrays(gl::TRIANGLES, 0, 36);
                gl::DrawElements(gl::TRIANGLES, model.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());

            }

            gl::BindVertexArray(0);

        }

        window.swap_buffers().unwrap();

        let mut first_mouse = true;
        let mut last_x = 0.0;
        let mut last_y = 0.0;

        for event in window.poll_events() {
            match event {
                Event::Closed => break'running,
                Event::KeyboardInput(ElementState::Pressed, _, Some(x)) => {
                    pressed_keys[x as usize] = true;
                },
                Event::KeyboardInput(ElementState::Released, _, Some(x)) => {
                    pressed_keys[x as usize] = false;
                },
                Event::MouseMoved((x, y)) => {
                    // let  = pos;
                    let x = x as f32;
                    let y = y as f32;
                    if first_mouse {
                		last_x = x;
                		last_y = y;
                		first_mouse = false;
                	}
                	let mut xoffset = x - last_x;
                	let mut yoffset = last_y - y; // Reversed since y-coordinates range from bottom to top
                	last_x = x;
                	last_y = y;

                	let sensitivity = 0.10;
                	xoffset *= sensitivity;
                	yoffset *= sensitivity;
                	camera.rotation.z += xoffset;
                	camera.rotation.y += yoffset;
                	if camera.rotation.y > 89.0 {
                        camera.rotation.y = 89.0;
                    } else if camera.rotation.y < -89.0 {
                        camera.rotation.y = -89.0;
                    }

                    // window.set_cursor(MouseCursor::NoneCursor);
                    let _ = window.set_cursor_position(WIDTH as i32/2, HEIGHT as i32/2);

                },
                _ => (),
            }
        }

    }

    // Cleanup - OpenGL memory dealocation
    unsafe {
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
        gl::DeleteVertexArrays(1, &vao);
    }
}

fn input(pressed_keys: &[bool; 1024], camera: &mut Camera) {

    let camera_speed = 2.0 * 0.16;
    let temp_cam_front = Vec3::new(camera.forward.x, 0.0, camera.forward.z);

    if pressed_keys[VirtualKeyCode::A as usize] {
        camera.position = camera.position - Vec3::cross(camera.forward, camera.up).normalize() * Vec3::new(camera_speed, camera_speed, camera_speed);
    }

    if pressed_keys[VirtualKeyCode::D as usize] {
        camera.position = camera.position + Vec3::cross(camera.forward, camera.up).normalize() * Vec3::new(camera_speed, camera_speed, camera_speed);
    }

    if pressed_keys[VirtualKeyCode::W as usize] {
        camera.position = camera.position + temp_cam_front * Vec3::new(camera_speed, camera_speed, camera_speed);
    }

    if pressed_keys[VirtualKeyCode::S as usize] {
        camera.position = camera.position - temp_cam_front * Vec3::new(camera_speed, camera_speed, camera_speed);
    }

    if pressed_keys[VirtualKeyCode::Q as usize] {
        camera.position = camera.position - Vec3::new(0.0, camera_speed, 0.0);
    }

    if pressed_keys[VirtualKeyCode::E as usize] {
        camera.position = camera.position + Vec3::new(0.0, camera_speed, 0.0);
    }
}
