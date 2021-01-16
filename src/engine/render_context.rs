extern crate opengl as gl;

use camera::Camera;
use entity::Entity;
use framebuffer::Framebuffer;
use lights::{DirLight, PointLight};
use skybox::Skybox;

pub struct RenderContext {
  framebuffer: Framebuffer,
  skybox: Skybox,
}

impl RenderContext {
  // TODO: it should adjust based on resizes
  pub fn new(width: i32, height: i32, skybox: Skybox) -> RenderContext {
    let framebuffer = Framebuffer::new(width, height);

    RenderContext {
      framebuffer,
      skybox,
    }
  }

  pub fn render(
    &self,
    camera: &Camera,
    point_light: &PointLight,
    dir_light: &DirLight,
    entities: &Vec<&Entity>,
  ) {
    self.framebuffer.bind();

    unsafe {
      // gl::Enable(gl::CULL_FACE);
      // gl::FrontFace(gl::CW);
      // gl::CullFace(gl::FRONT_AND_BACK);
      gl::ClearColor(44.0 / 255.0, 44.0 / 255.0, 44.0 / 255.0, 1.0);
      gl::Enable(gl::DEPTH_TEST);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    let view_matrix = camera.get_look_at_matrix();

    self.skybox.render(&camera);

    // render all models
    entities
      .iter()
      .for_each(|entity| (**entity).render(&camera, &point_light, &dir_light));

    point_light.render(camera.projection_matrix, view_matrix);

    self.framebuffer.draw();
  }
}
