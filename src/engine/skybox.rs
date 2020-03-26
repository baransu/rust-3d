extern crate image;
extern crate math;
extern crate opengl as gl;

use camera::Camera;
use model::Model;
use shader::ShaderProgram;
use std::mem;

pub struct Skybox {
  shader: SkyboxShader,
  model: Model,
  texture: u32,
}

impl Skybox {
  pub fn new(faces: Vec<&str>) -> Skybox {
    let model = Model::from_obj("res/models/cube.obj");

    let mut texture = 0;

    // SKYBOX TEXTURE
    unsafe {
      gl::GenTextures(1, &mut texture);
      gl::BindTexture(gl::TEXTURE_2D, texture);
    }

    for (i, face) in faces.iter().enumerate() {
      let texture_data = image::open(face)
        .expect("Opening image for texture failed")
        .to_rgba();

      unsafe {
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
    }

    println!("Loaded skybox faces: {:?}", faces);

    unsafe {
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

    Skybox {
      model,
      shader: SkyboxShader::new(),
      texture,
    }
  }

  pub fn render(&self, camera: &Camera) {
    self.shader.bind(&camera);

    unsafe {
      gl::DepthMask(gl::FALSE);
      gl::ActiveTexture(gl::TEXTURE0);
      gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.texture);
    }

    self.model.render();

    unsafe {
      gl::DepthMask(gl::TRUE);
    }
  }
}

struct SkyboxShader {
  shader: ShaderProgram,
}

impl SkyboxShader {
  pub fn new() -> SkyboxShader {
    SkyboxShader {
      shader: ShaderProgram::new("res/skybox.vert", "res/skybox.frag"),
    }
  }

  pub fn bind(&self, camera: &Camera) {
    self.shader.bind();
    self
      .shader
      .set_uniform_matrix4fv("projection", camera.projection_matrix);
    self
      .shader
      .set_uniform_matrix4fv("view", camera.get_look_at_matrix());
    self.shader.set_uniform_1i("skybox", 0)
  }
}
