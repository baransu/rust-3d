extern crate math;

use self::math::mat4::Mat4;
use camera::Camera;
use lights::{DirLight, PointLight};
use model::Model;
use shader::ShaderProgram;
use transform::Transform;

pub struct Entity<'a> {
  shader: EntityShader,
  pub transform: Transform,
  model: &'a Model,
}

impl<'a> Entity<'a> {
  pub fn new(transform: Transform, model: &'a Model) -> Entity<'a> {
    Entity {
      shader: EntityShader::new(),
      transform,
      model,
    }
  }

  pub fn render(&self, camera: &Camera, point_light: &PointLight, dir_light: &DirLight) {
    let model_matrix = self.transform.get_model_matrix();
    self
      .shader
      .bind(&model_matrix, camera, point_light, dir_light);

    self.model.render();
  }
}

struct EntityShader {
  shader: ShaderProgram,
}

impl EntityShader {
  pub fn new() -> EntityShader {
    EntityShader {
      shader: ShaderProgram::new("res/vshader.vert", "res/fshader.frag"),
    }
  }

  pub fn bind(
    &self,
    model_matrix: &Mat4,
    camera: &Camera,
    point_light: &PointLight,
    dir_light: &DirLight,
  ) {
    self.shader.bind();

    // diffuse_map.bind(gl::TEXTURE0);
    self.shader.set_uniform_1i("diffuseMap", 0);

    // specular_map.bind(gl::TEXTURE1);
    self.shader.set_uniform_1i("specularMap", 1);

    // normal_map.bind(gl::TEXTURE2);
    self.shader.set_uniform_1i("normalMap", 2);

    self
      .shader
      .set_uniform_matrix4fv("projection", camera.projection_matrix);

    self
      .shader
      .set_uniform_matrix4fv("view", camera.get_look_at_matrix());

    self.shader.set_uniform_3f("lightPos", point_light.position);
    self.shader.set_uniform_3f("viewPos", camera.position);

    // directional light
    self
      .shader
      .set_uniform_3f("dirLight.direction", dir_light.direction);
    self
      .shader
      .set_uniform_3f("dirLight.ambient", dir_light.ambient);
    self
      .shader
      .set_uniform_3f("dirLight.diffuse", dir_light.diffuse);
    self
      .shader
      .set_uniform_3f("dirLight.specular", dir_light.specular);

    self
      .shader
      .set_uniform_3f("pointLight.position", point_light.position);

    self
      .shader
      .set_uniform_3f("pointLight.ambient", point_light.ambient);
    self
      .shader
      .set_uniform_3f("pointLight.diffuse", point_light.diffuse);
    self
      .shader
      .set_uniform_3f("pointLight.specular", point_light.specular);

    self
      .shader
      .set_uniform_1f("pointLight.constant", point_light.constant);
    self
      .shader
      .set_uniform_1f("pointLight.linear", point_light.linear);
    self
      .shader
      .set_uniform_1f("pointLight.quadratic", point_light.quadratic);

    self.shader.set_uniform_matrix4fv("model", *model_matrix);
  }
}
