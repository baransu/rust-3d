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
use engine::model::{ Modell };
use engine::camera::Camera;
use engine::lights::{ PointLight, DirLight };

use math::mat4::Mat4;
use math::vec4::Vec4;
use math::vec3::Vec3;
use math::vec2::Vec2;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;

static QUAD_VERTICES: [f32; 24] = [
    // Positions   // TexCoords
    -1.0,  1.0,  0.0, 1.0,
    -1.0, -1.0,  0.0, 0.0,
     1.0, -1.0,  1.0, 0.0,

    -1.0,  1.0,  0.0, 1.0,
     1.0, -1.0,  1.0, 0.0,
     1.0,  1.0,  1.0, 1.0
];

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

    let shader = Shader::new("res/advanced_lightning.vert", "res/advanced_lightning.frag");
    // let normal_map = Texture::new("res/mouse/mouseNormal.png", 4.0);
    // let diffuse_map = Texture::new("res/mouse/mouseAlbedo.png", 4.0);
    // let specular_map = Texture::new("res/mouse/mouseRoughness.png", 4.0);

    let mut entities = Vec::new();

    // let model = Mod::new("res/models/", "susanne_lowpoly.obj");
    // let model = Mod::new("res/models/", "susanne_highpoly.obj");
    // let model = Modell::new("res/models/mouse/", "mouselowpoly.obj");
    // let model = Modell::new("res/ves/", "Ves.obj");
    let model = Modell::new("res/models/", "sphere.obj");

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

    for i in 0..10 {
        entities.push(Transform::new(Vec3::new(0.0, -5.0, -2.5 * i as f32), Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)));

    }

    // dir_light
    let dir_light = DirLight::new(
        Vec3::new(-0.2, -1.0, -0.3), //direction

        Vec3::new(0.1, 0.1, 0.1), //ambient
        Vec3::new(0.25, 0.25, 0.25), //diffuse
        Vec3::new(0.2, 0.2, 0.2) //specular
    );

    let mut point_light = PointLight::new(
        Vec3::new(0.0, 1.0, 3.0), //position

        0.08, //linear
        0.032, //quadratic

        Vec3::new(0.1, 0.1, 0.1), //ambient
        Vec3::new(1.0, 0.0, 1.0), //diffuse
        Vec3::new(1.0, 1.0, 1.0) //specular
    );

    let mut fbo = 0;
    let mut rbo = 0;

    let mut fbo_quad_vao = 0;
    let mut fbo_quad_vbo = 0;

    let mut fbo_texture = 0;

    let framebuffer_shader = Shader::new("res/framebuffer.vert", "res/framebuffer.frag");

    let skybox = Modell::new("res/models/", "cube.obj");
    let skybox_shader = Shader::new("res/skybox.vert", "res/skybox.frag");

    let u_PreintegratedFG = Texture::new("res/PreintegratedFG.bmp", 4.0);
    let u_AlbedoMap = Texture::new("res/GunMetal_Albedo.tga", 4.0);
    let u_SpecularMap = Texture::new("res/GunMetal_Specular.tga", 4.0);
    let u_GlossMap = Texture::new("res/GunMetal_Gloss.tga", 4.0);
    let u_NormalMap = Texture::new("res/GunMetal_Normal.tga", 4.0);

    let skybox_faces = vec![
        "res/cubemap_right",
        "res/cubemap_left",
        "res/cubemap_top",
        "res/cubemap_bottom",
        "res/cubemap_back",
        "res/cubemap_front"
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
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, WIDTH as i32, HEIGHT as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, fbo_texture, 0);

        // renderbuffer
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);

        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, WIDTH as i32, HEIGHT as i32);
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo);

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
        gl::BufferData(gl::ARRAY_BUFFER, (QUAD_VERTICES.len() * mem::size_of::<f32>()) as GLsizeiptr, mem::transmute(&QUAD_VERTICES[0]), gl::STATIC_DRAW);

        // pos
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE as GLboolean, 4 * mem::size_of::<f32>() as i32, ptr::null());
        // uvs
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE as GLboolean, 4 * mem::size_of::<f32>() as i32, mem::transmute(2 * mem::size_of::<f32>()));

        gl::BindVertexArray(0);

        // SKYBOX TEXTURE
        gl::GenTextures(1, &mut skybox_texture);
        gl::BindTexture(gl::TEXTURE_2D, skybox_texture);

        for i in 0..skybox_faces.len() {
            let mut s = String::new();
            s.push_str(skybox_faces[i]);
            s.push_str("0.png");
            println!("loading: {:?}", s);
            let texture_data = image::open(s).expect("Opening image for texture failed");
            let texture_data = texture_data.to_rgba();

            gl::TexImage2D(
                gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                0,
                gl::RGBA as i32,
                texture_data.width() as i32,
                texture_data.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                mem::transmute(&texture_data.into_raw()[0])
            );
            // for j in 1..10 {
            //     let mut s = String::new();
            //     s.push_str(skybox_faces[i]);
            //     s.push_str(&j.to_string()[..]);
            //     s.push_str(".png");
            //     println!("loading: {:?}", s);
            //     let texture_data = image::open(s).expect("Opening image for texture failed");
            //     let texture_data = texture_data.to_rgba();
            //     gl::TexSubImage2D(
            //         gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
            //         j,
            //         0,
            //         0,
            //         texture_data.width() as i32,
            //         texture_data.height() as i32,
            //         gl::RGBA,
            //         gl::UNSIGNED_BYTE,
            //         mem::transmute(&texture_data.into_raw()[0])
            //     ); // Copy data to the second mipmap
            // }
        }

        gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        // gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);

        gl::BindTexture(gl::TEXTURE_2D, 0);

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

            // bind offscreen framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            gl::ClearColor(44.0/255.0, 44.0/255.0, 44.0/255.0, 1.0);
            gl::Enable(gl::DEPTH_TEST);
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

            // uniforms for vertex shader
            shader.set_uniform_matrix4fv("sys_ProjectionMatrix", projection_matrix);
            shader.set_uniform_matrix4fv("sys_ViewMatrix", view_matrix);

            shader.set_uniform_3f("sys_CameraPosition", camera.position);


            // uniforms for fragment shader

            // u_PreintegratedFG
            u_PreintegratedFG.bind(gl::TEXTURE0);
            shader.set_uniform_1i("u_PreintegratedFG", 0);

            //enviro map
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_texture);
            shader.set_uniform_1i("u_EnvironmentMap", 1);

            // // u_AlbedoMap;
            // u_AlbedoMap.bind(gl::TEXTURE2);
            // shader.set_uniform_1i("u_AlbedoMap", 2);
            // // u_SpecularMap;
            // u_SpecularMap.bind(gl::TEXTURE3);
            // shader.set_uniform_1i("u_SpecularMap", 3);
            // // u_GlossMap;
            // u_GlossMap.bind(gl::TEXTURE4);
            // shader.set_uniform_1i("u_GlossMap", 4);
            // // u_NormalMap;
            // u_NormalMap.bind(gl::TEXTURE5);
            // shader.set_uniform_1i("u_NormalMap", 5);


            // u_AlbedoColor;
            // Iron	(0.560, 0.570, 0.580)
            // Silver	(0.972, 0.960, 0.915)
            // Aluminum	(0.913, 0.921, 0.925)
            // Gold	(1.000, 0.766, 0.336)
            // Copper	(0.955, 0.637, 0.538)
            // Chromium	(0.550, 0.556, 0.554)
            // Nickel	(0.660, 0.609, 0.526)
            // Titanium	(0.542, 0.497, 0.449)
            // Cobalt	(0.662, 0.655, 0.634)
            // Platinum	(0.672, 0.637, 0.585)
            shader.set_uniform_4f("u_AlbedoColor", Vec4::new(0.0, 0.0, 0.0, 1.0));

            // u_SpecularColor;
            shader.set_uniform_3f("u_SpecularColor", Vec3::new(0.4, 0.4, 0.4));

            // u_UsingAlbedoMap;
            shader.set_uniform_1f("u_UsingAlbedoMap", 0.0);
            // u_UsingSpecularMap;
            shader.set_uniform_1f("u_UsingSpecularMap", 0.0);
            // u_UsingGlossMap;
            shader.set_uniform_1f("u_UsingGlossMap", 0.0);
            // u_UsingNormalMap;
            shader.set_uniform_1f("u_UsingNormalMap", 0.0);

            // directional light
            // shader.set_uniform_3f("dirLight.direction", dir_light.direction);
            // shader.set_uniform_3f("dirLight.ambient", dir_light.ambient);
            // shader.set_uniform_3f("dirLight.diffuse", dir_light.diffuse);
            // shader.set_uniform_3f("dirLight.specular", dir_light.specular);
            //
            //
            // // point light
            shader.set_uniform_4f("u_Light.color", Vec4::new(0.0, 0.0, 0.0, 1.0));
            shader.set_uniform_3f("u_Light.position", point_light.position);

            shader.set_uniform_1f("u_Light.p0", 0.0);
            shader.set_uniform_1f("u_Light.p1", 1.0);

            shader.set_uniform_1f("u_Light.intensity", 0.0);

            shader.set_uniform_3f("u_Light.direction", dir_light.direction);

            for (i, entity) in &mut entities.iter().enumerate() {
                //
                // entity.rotation.y += 5.0 * 0.16;
                // entity.rotation.z += 5.0 * 0.16;

                // u_GlossColor;
                shader.set_uniform_1f("u_GlossColor", 1.0 - (i as f32 / 10.0));


                shader.set_uniform_matrix4fv("sys_ModelMatrix", entity.get_model_matrix());
                model.draw_without_textures();
            }
            //
            // if forward && point_light.position.z > -25.0 {
            //     point_light.position.z -= 5.0 * 0.016;
            // } else if point_light.position.z < -25.0 {
            //     forward = false;
            // }
            //
            // if !forward && point_light.position.z < 0.0 {
            //     point_light.position.z += 5.0 * 0.016;
            // } else if point_light.position.z > 0.0 {
            //     forward = true;
            // }
            //
            // point_light.draw(projection_matrix, view_matrix);

            // bind default framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            // draw offscreen texture
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Disable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            framebuffer_shader.bind();
            gl::BindVertexArray(fbo_quad_vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, fbo_texture);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
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

    unsafe {
        gl::DeleteFramebuffers(1, &mut fbo);
        gl::DeleteFramebuffers(1, &mut rbo);

        gl::DeleteFramebuffers(1, &mut fbo_quad_vao);
        gl::DeleteFramebuffers(1, &mut fbo_quad_vbo);
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
