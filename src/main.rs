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
use engine::model::Model;
use engine::camera::Camera;
use engine::lights::{PointLight, DirLight};
use engine::scene::Scene;

use math::mat4::Mat4;
use math::vec3::Vec3;
use math::vec2::Vec2;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

static QUAD_VERTICES: [f32; 24] = [
    // Positions   // TexCoords
    -1.0,
    1.0,

    0.0,
    1.0,

    -1.0,
    -1.0,

    0.0,
    0.0,

    1.0,
    -1.0,

    1.0,
    0.0,

    -1.0,
    1.0,

    0.0,
    1.0,

    1.0,
    -1.0,

    1.0,
    0.0,

    1.0,
    1.0,

    1.0,
    1.0,
];

fn main() {

    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("rust-3d".to_string())
        .with_dimensions(1024, 768);

    let context = glutin::ContextBuilder::new().with_vsync(true);

    let window = glutin::GlWindow::new(window_builder, context, &events_loop).unwrap();

    window.set_cursor_position(WIDTH as i32 / 2, HEIGHT as i32 / 2);

    window.set_cursor_state(CursorState::Grab).ok().expect(
        "could not grab mouse cursor",
    );

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

    let shader = Shader::new("res/vshader.vert", "res/handpainted.frag");
    // let normal_map = Texture::new("res/mouse/mouseNormal.png", 4.0);
    // let diffuse_map = Texture::new("res/mouse/mouseAlbedo.png", 4.0);
    // let specular_map = Texture::new("res/mouse/mouseRoughness.png", 4.0);

    let mut entities = Vec::new();

    // let model = Mod::new("res/models/", "susanne_lowpoly.obj");
    // let model = Mod::new("res/models/", "susanne_highpoly.obj");
    // let model = Model::new("res/models/mouse/", "mouselowpoly.obj");
    // let model = Model::new("res/ves/", "Ves.obj");
    let model = Model::new("res/models/column.obj");

    let mut forward = true;

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

    entities.push(Transform::new(
        Vec3::new(0.0, -5.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    ));
    entities.push(Transform::new(
        Vec3::new(0.0, -5.0, -5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    ));
    entities.push(Transform::new(
        Vec3::new(0.0, -5.0, -10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    ));
    entities.push(Transform::new(
        Vec3::new(0.0, -5.0, -15.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    ));
    entities.push(Transform::new(
        Vec3::new(0.0, -5.0, -20.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    ));
    entities.push(Transform::new(
        Vec3::new(0.0, -5.0, -25.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    ));

    // dir_light
    let dir_light = DirLight::new(
        Vec3::new(-0.2, -1.0, -0.3), //direction

        Vec3::new(0.1, 0.1, 0.1), //ambient
        Vec3::new(0.25, 0.25, 0.25), //diffuse
        Vec3::new(0.2, 0.2, 0.2), //specular
    );

    let mut point_light = PointLight::new(
        Vec3::new(0.0, 1.0, 3.0), //position

        0.08, //linear
        0.032, //quadratic

        Vec3::new(0.1, 0.1, 0.1), //ambient
        Vec3::new(1.0, 0.0, 1.0), //diffuse
        Vec3::new(1.0, 1.0, 1.0), //specular
    );

    let mut fbo = 0;
    let mut rbo = 0;

    let mut fbo_quad_vao = 0;
    let mut fbo_quad_vbo = 0;

    let mut fbo_texture = 0;

    let framebuffer_shader = Shader::new("res/framebuffer.vert", "res/framebuffer.frag");


    let mut skybox = Model::new("res/models/cube.obj");
    let mut skybox_shader = Shader::new("res/skybox.vert", "res/skybox.frag");

    let skybox_faces = vec![
        "res/cubemap_right.png",
        "res/cubemap_left.png",
        "res/cubemap_top.png",
        "res/cubemap_bottom.png",
        "res/cubemap_back.png",
        "res/cubemap_front.png",
    ];

    let mut skybox_texture = 0;

    unsafe {
        // gl::Enable(gl::CULL_FACE);
        // gl::FrontFace(gl::CW);
        // gl::CullFace(gl::FRONT_AND_BACK);

        // setting framebuffer
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        // texture
        gl::GenTextures(1, &mut fbo_texture);
        gl::BindTexture(gl::TEXTURE_2D, fbo_texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            WIDTH as i32,
            HEIGHT as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            fbo_texture,
            0,
        );

        // renderbuffer
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);

        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH24_STENCIL8,
            WIDTH as i32,
            HEIGHT as i32,
        );
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            rbo,
        );

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            panic!("Framebuffer is not complete!");
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        // framebuffer quad
        gl::GenVertexArrays(1, &mut fbo_quad_vao);
        gl::GenBuffers(1, &mut fbo_quad_vbo);

        gl::BindVertexArray(fbo_quad_vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::BindBuffer(gl::ARRAY_BUFFER, fbo_quad_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (QUAD_VERTICES.len() * mem::size_of::<f32>()) as GLsizeiptr,
            mem::transmute(&QUAD_VERTICES[0]),
            gl::STATIC_DRAW,
        );

        // pos
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            4 * mem::size_of::<f32>() as i32,
            ptr::null(),
        );
        // uvs
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            4 * mem::size_of::<f32>() as i32,
            mem::transmute(2 * mem::size_of::<f32>()),
        );

        gl::BindVertexArray(0);

        // SKYBOX TEXTURE
        gl::GenTextures(1, &mut skybox_texture);
        gl::BindTexture(gl::TEXTURE_2D, skybox_texture);

        for i in 0..skybox_faces.len() {
            let texture_data =
                image::open(skybox_faces[i]).expect("Opening image for texture failed");
            let texture_data = texture_data.to_rgba();
            println!("loaded: {:?}", skybox_faces[i]);

            gl::TexImage2D(
                gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                0,
                gl::RGBA as i32,
                texture_data.width() as i32,
                texture_data.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                mem::transmute(&texture_data.into_raw()[0]),
            );
        }

        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_MAG_FILTER,
            gl::LINEAR as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_WRAP_S,
            gl::CLAMP_TO_EDGE as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_WRAP_T,
            gl::CLAMP_TO_EDGE as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_WRAP_R,
            gl::CLAMP_TO_EDGE as i32,
        );


        gl::BindTexture(gl::TEXTURE_2D, 0);

    }

    let mut running = true;
    let mut time = 0.0;
    while running {

        // process input
        input(&pressed_keys, &mut camera);

        time += 0.16;
        let ts = time::get_time();
        // println!("{:?}", ts.sec as f64);
        let angle: f64 = ts.sec as f64 + ts.nsec as f64 / 1000000000.0;
        // println!("{:?}", time);

        unsafe {

            // bind offscreen framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            gl::ClearColor(44.0 / 255.0, 44.0 / 255.0, 44.0 / 255.0, 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);


            // near - as big as posible (0.1)
            // far - as small as posible (100 - far and small enought)
            let projection_matrix = Mat4::from_perspective(45.0, WIDTH / HEIGHT, 0.1, 100.0);

            // opengl forward is -z;
            // let radius = 20.0;
            // camera.position.x = (angle.cos() * radius) as f32;
            // camera.position.z = (angle.sin() * radius) as f32;
            // let view_matrix = camera.get_look_at_target_matrix(Vec3::new(0.0, 0.0, 0.0));
            let view_matrix = camera.get_look_at_matrix();

            gl::DepthMask(gl::FALSE);
            skybox_shader.bind();

            skybox_shader.set_uniform_matrix4fv("projection", projection_matrix);
            skybox_shader.set_uniform_matrix4fv("view", view_matrix);

            gl::ActiveTexture(gl::TEXTURE0);
            skybox_shader.set_uniform_1i("skybox", 0);

            gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_texture);
            skybox.draw();
            gl::DepthMask(gl::TRUE);


            // draw scene
            shader.bind();

            // diffuse_map.bind(gl::TEXTURE0);
            shader.set_uniform_1i("diffuseMap", 0);

            // specular_map.bind(gl::TEXTURE1);
            shader.set_uniform_1i("specularMap", 1);

            // normal_map.bind(gl::TEXTURE2);
            shader.set_uniform_1i("normalMap", 2);

            shader.set_uniform_matrix4fv("projection", projection_matrix);
            shader.set_uniform_matrix4fv("view", view_matrix);

            shader.set_uniform_3f("viewPos", camera.position);

            // directional light
            shader.set_uniform_3f("dirLight.direction", dir_light.direction);
            shader.set_uniform_3f("dirLight.ambient", dir_light.ambient);
            shader.set_uniform_3f("dirLight.diffuse", dir_light.diffuse);
            shader.set_uniform_3f("dirLight.specular", dir_light.specular);

            // point light
            // let ligh_pos = Vec3::new(0.0, 2.0, 2.0);

            shader.set_uniform_3f("pointLight.position", point_light.position);

            shader.set_uniform_3f("pointLight.ambient", point_light.ambient);
            shader.set_uniform_3f("pointLight.diffuse", point_light.diffuse);
            shader.set_uniform_3f("pointLight.specular", point_light.specular);

            shader.set_uniform_1f("pointLight.constant", point_light.constant);
            shader.set_uniform_1f("pointLight.linear", point_light.linear);
            shader.set_uniform_1f("pointLight.quadratic", point_light.quadratic);

            for entity in &mut entities {
                entity.rotation.y += 5.0 * 0.16;
                // entity.rotation.z += 5.0 * 0.16;

                shader.set_uniform_matrix4fv("model", entity.get_model_matrix());
                model.draw();
            }

            if forward && point_light.position.z > -25.0 {
                point_light.position.z -= 5.0 * 0.016;
            } else if point_light.position.z < -25.0 {
                forward = false;
            }

            if !forward && point_light.position.z < 0.0 {
                point_light.position.z += 5.0 * 0.016;
            } else if point_light.position.z > 0.0 {
                forward = true;
            }

            point_light.draw(projection_matrix, view_matrix);


            // bind default framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            // draw offscreen texture
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Disable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            framebuffer_shader.bind();
            gl::BindVertexArray(fbo_quad_vao);
            gl::BindTexture(gl::TEXTURE_2D, fbo_texture);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }

        window.swap_buffers().unwrap();

        events_loop.poll_events(|event| match event {
            glutin::Event::DeviceEvent { event, .. } => {
                match event { 
                    glutin::DeviceEvent::MouseMotion { delta: (x, y) } => {
                        let sensitivity = 0.01;

                        camera.rotation.z += (x as f32) * sensitivity;
                        camera.rotation.y -= (y as f32) * sensitivity;

                        if camera.rotation.y > 89.0 {
                            camera.rotation.y = 89.0;
                        } else if camera.rotation.y < -89.0 {
                            camera.rotation.y = -89.0;
                        }
                    }

                    _ => {}

                }
            }

            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::Resized(w, h) => window.resize(w, h),

                    WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode,
                            state: ElementState::Pressed,
                            ..
                        },
                        ..
                    } => {
                        if let Some(x) = virtual_keycode {
                            pressed_keys[x as usize] = true;
                            if is_pressed(&pressed_keys, VirtualKeyCode::Escape) {
                                running = false;
                            }

                        }
                    }

                    WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode,
                            state: ElementState::Released,
                            ..
                        },
                        ..
                    } => {
                        if let Some(x) = virtual_keycode {
                            pressed_keys[x as usize] = false;
                        }
                    }

                    _ => {}
                }
            } 
            _ => {}
        })
    }

    unsafe {
        gl::DeleteFramebuffers(1, &mut fbo);
        gl::DeleteFramebuffers(1, &mut rbo);

        gl::DeleteFramebuffers(1, &mut fbo_quad_vao);
        gl::DeleteFramebuffers(1, &mut fbo_quad_vbo);
    }
}

fn input(keys: &[bool; 1024], camera: &mut Camera) {
    let camera_speed = 2.0 * 0.16;
    let temp_cam_front = Vec3::new(camera.forward.x, 0.0, camera.forward.z);

    if is_pressed(keys, VirtualKeyCode::A) {
        camera.position = camera.position -
            Vec3::cross(camera.forward, camera.up).normalize() *
                Vec3::new(camera_speed, camera_speed, camera_speed);
    }

    if is_pressed(keys, VirtualKeyCode::D) {
        camera.position = camera.position +
            Vec3::cross(camera.forward, camera.up).normalize() *
                Vec3::new(camera_speed, camera_speed, camera_speed);
    }

    if is_pressed(keys, VirtualKeyCode::W) {
        camera.position = camera.position +
            temp_cam_front * Vec3::new(camera_speed, camera_speed, camera_speed);
    }

    if is_pressed(keys, VirtualKeyCode::S) {
        camera.position = camera.position -
            temp_cam_front * Vec3::new(camera_speed, camera_speed, camera_speed);
    }

    if is_pressed(keys, VirtualKeyCode::Q) {
        camera.position = camera.position - Vec3::new(0.0, camera_speed, 0.0);
    }

    if is_pressed(keys, VirtualKeyCode::E) {
        camera.position = camera.position + Vec3::new(0.0, camera_speed, 0.0);
    }

    /* CAMERA ROTATION VIA KEYBOARD */

    // up rotation
    if is_pressed(keys, VirtualKeyCode::Z) {
        camera.rotation = Vec3::new(
            camera.rotation.x,
            camera.rotation.y + camera_speed,
            camera.rotation.z,
        );
    }

    // down rotation
    if is_pressed(keys, VirtualKeyCode::X) {
        camera.rotation = Vec3::new(
            camera.rotation.x,
            camera.rotation.y - camera_speed,
            camera.rotation.z,
        );
    }

    // left rotation
    if is_pressed(keys, VirtualKeyCode::C) {
        camera.rotation = Vec3::new(
            camera.rotation.x,
            camera.rotation.y,
            camera.rotation.z - camera_speed,
        );
    }

    // right rotation
    if is_pressed(keys, VirtualKeyCode::V) {
        camera.rotation = Vec3::new(
            camera.rotation.x,
            camera.rotation.y,
            camera.rotation.z + camera_speed,
        );
    }
}

fn is_pressed(pressed_keys: &[bool; 1024], key: VirtualKeyCode) -> bool {
    pressed_keys[key as usize]
}
