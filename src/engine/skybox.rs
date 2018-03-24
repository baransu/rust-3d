extern crate math;
extern crate opengl as gl;
extern crate image;

use self::gl::types::*;
use self::math::mat4::Mat4;
use std::mem;
use shader::Shader;
use model::Model;

pub struct Skybox {
  shader: Shader,
  model: Model,
  texture: u32
}

impl Skybox {
  pub fn new(faces: Vec<&str>) -> Skybox {

    let mut model = Model::new("res/models/cube.obj");
    let mut shader = Shader::new("res/skybox.vert", "res/skybox.frag");

    let mut texture = 0;

      // SKYBOX TEXTURE
      unsafe { 
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
      }

      for (i, face) in faces.iter().enumerate() {
        let texture_data = image::open(face).expect("Opening image for texture failed").to_rgba();

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

    Skybox { model, shader, texture }
  }

  pub fn draw(&self, projection_matrix: Mat4, view_matrix: Mat4) {
    self.shader.bind();
    self.shader.set_uniform_matrix4fv("projection", projection_matrix);
    self.shader.set_uniform_matrix4fv("view", view_matrix);
    self.shader.set_uniform_1i("skybox", 0);

    unsafe {   
      gl::DepthMask(gl::FALSE);
      gl::ActiveTexture(gl::TEXTURE0);
      gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.texture);
    }

    unsafe { 
      self.model.draw();

      gl::DepthMask(gl::TRUE);
    }
  }
}