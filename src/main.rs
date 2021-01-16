extern crate engine;
extern crate glutin;
extern crate image;
extern crate math;
extern crate opengl as gl;
extern crate rand;
extern crate time;

use glutin::*;

// local
use engine::camera::Camera;
use engine::entity::Entity;
use engine::lights::{DirLight, PointLight};
use engine::model::Model;
use engine::render_context::RenderContext;
use engine::skybox::Skybox;
use engine::transform::Transform;

use math::vec3::Vec3;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("rust-3d".to_string())
        .with_dimensions(WIDTH as u32, HEIGHT as u32);

    let context = glutin::ContextBuilder::new().with_vsync(true);

    let window = glutin::GlWindow::new(window_builder, context, &events_loop).unwrap();

    window
        .set_cursor_position(WIDTH as i32 / 2, HEIGHT as i32 / 2)
        .unwrap();

    window
        .set_cursor_state(CursorState::Grab)
        .ok()
        .expect("could not grab mouse cursor");

    // It is essential to make the context current before calling `gl::load_with`.
    unsafe { window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    // TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // input stuff
    let mut pressed_keys: [bool; 1024] = [false; 1024];

    let mut camera = Camera::new(
        WIDTH,
        HEIGHT,
        Vec3::new(0.0, 0.0, 20.0),
        Vec3::new(0.0, 0.0, -90.0),
    );

    let mut forward = true;

    let mut tiles = vec![];
    for x in -5..5 {
        for z in -5..5 {
            tiles.push(Vec3::new(-2.0 * x as f32, -1.1, -2.0 * z as f32));
        }
    }

    let mut positions = vec![];
    for x in -5..5 {
        for z in -5..5 {
            positions.push(Vec3::new(-2.0 * x as f32, 0.0, -2.0 * z as f32));
        }
    }

    // dir_light
    let dir_light = DirLight::new(
        Vec3::new(-0.2, -1.0, -0.3), //direction
        Vec3::new(0.6, 0.6, 0.6),    //ambient
        Vec3::new(0.25, 0.25, 0.25), //diffuse
        Vec3::new(0.2, 0.2, 0.2),    //specular
    );

    let mut point_light = PointLight::new(
        Vec3::new(0.0, 1.0, 3.0), //position
        0.08,                     //linear
        0.032,                    //quadratic
        Vec3::new(0.1, 0.1, 0.1), //ambient
        Vec3::new(1.0, 0.0, 1.0), //diffuse
        Vec3::new(1.0, 1.0, 1.0), //specular
    );

    let mut running = true;

    let ves = Model::from_obj("res/models/ves/Ves.obj");
    let floor = Model::from_obj("res/models/plane.obj");

    let mut characters: Vec<Entity> = positions
        .iter()
        .map(|position| {
            let transform = Transform::new(
                *position,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 1.0, 1.0),
            );

            Entity::new(transform, &ves)
        })
        .collect();

    let ground: Vec<Entity> = tiles
        .iter()
        .map(|position| {
            let transform = Transform::new(
                *position,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 1.0, 1.0),
            );

            Entity::new(transform, &floor)
        })
        .collect();

    let render_context = RenderContext::new(
        WIDTH as i32,
        HEIGHT as i32,
        Skybox::new(vec![
            "res/cubemap_right.png",
            "res/cubemap_left.png",
            "res/cubemap_top.png",
            "res/cubemap_bottom.png",
            "res/cubemap_back.png",
            "res/cubemap_front.png",
        ]),
    );

    let delta_time: f32 = 16.0 / 1000.0;

    while running {
        // Process input
        input(&pressed_keys, &mut camera);

        characters
            .iter_mut()
            .for_each(|entity| entity.transform.rotation.y += 5.0 * delta_time);

        let entities: Vec<&Entity> = characters.iter().chain(ground.iter()).collect();

        render_context.render(&camera, &point_light, &dir_light, &entities);

        // Update light position
        if forward && point_light.position.z > -25.0 {
            point_light.position.z -= 5.0 * delta_time;
        } else if point_light.position.z < -25.0 {
            forward = false;
        }

        if !forward && point_light.position.z < 25.0 {
            point_light.position.z += 5.0 * delta_time;
        } else if point_light.position.z > 25.0 {
            forward = true;
        }

        window.swap_buffers().unwrap();

        events_loop.poll_events(|event| match event {
            glutin::Event::DeviceEvent { event, .. } => match event {
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
            },

            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => running = false,
                glutin::WindowEvent::Resized(w, h) => window.resize(w, h),

                WindowEvent::KeyboardInput {
                    input:
                        glutin::KeyboardInput {
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
                    input:
                        glutin::KeyboardInput {
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
            },
            _ => {}
        });
    }
}

fn input(keys: &[bool; 1024], camera: &mut Camera) {
    let camera_speed = 2.0 * 0.16;

    if is_pressed(keys, VirtualKeyCode::A) {
        camera.position = camera.position
            - Vec3::cross(camera.forward, camera.up).normalize()
                * Vec3::new(camera_speed, camera_speed, camera_speed);
    }

    if is_pressed(keys, VirtualKeyCode::D) {
        camera.position = camera.position
            + Vec3::cross(camera.forward, camera.up).normalize()
                * Vec3::new(camera_speed, camera_speed, camera_speed);
    }

    if is_pressed(keys, VirtualKeyCode::W) {
        camera.position =
            camera.position + camera.forward * Vec3::new(camera_speed, camera_speed, camera_speed);
    }

    if is_pressed(keys, VirtualKeyCode::S) {
        camera.position =
            camera.position - camera.forward * Vec3::new(camera_speed, camera_speed, camera_speed);
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
