extern crate math;
extern crate opengl as gl;

use self::gl::types::*;
use std::mem;
use std::ptr;
use shader::Shader;

#[derive(Debug)]
pub struct Framebuffer {
    shader: Shader,
    fbo: u32,
    rbo: u32,
    fbo_quad_vao: u32,
    fbo_quad_vbo: u32,
    texture: u32,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Framebuffer {
      let shader = Shader::new("res/framebuffer.vert", "res/framebuffer.frag");

      shader.bind();
      shader.set_uniform_1i("screenTexture", 0);

      let mut fbo = 0;        
      let mut rbo = 0;

      let mut fbo_quad_vao = 0;
      let mut fbo_quad_vbo = 0;

      let mut texture = 0;

      unsafe {

        // setting framebuffer

        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        // texture
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            width,
            height,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            texture,
            0,
        );

        // renderbuffer

        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);

        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH24_STENCIL8,
            width,
            height,
        );

        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            rbo,
        );

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            panic!("Framebuffer is not complete!");
        }

        // gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
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
      }

      Framebuffer { shader, fbo, rbo, fbo_quad_vao, fbo_quad_vbo, texture }
  }

  pub unsafe fn bind(&self) {
    gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
  }

  pub unsafe fn draw(&self) {
      self.shader.bind();
      gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

      gl::Disable(gl::DEPTH_TEST);
      gl::ClearColor(1.0, 1.0, 1.0, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);

      gl::BindVertexArray(self.fbo_quad_vao);
      gl::ActiveTexture(gl::TEXTURE0);
      gl::BindTexture(gl::TEXTURE_2D, self.texture);
      gl::DrawArrays(gl::TRIANGLES, 0, 6);
      gl::BindVertexArray(0);
  }
}

impl Drop for Framebuffer {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteFramebuffers(1, &self.fbo);
      gl::DeleteFramebuffers(1, &self.rbo);
      gl::DeleteBuffers(1, &self.fbo_quad_vao);
      gl::DeleteBuffers(1, &self.fbo_quad_vbo);
    }
  }
}


static QUAD_VERTICES: [f32; 24] = [
    // NOTE: Those are boken because of formatting
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
