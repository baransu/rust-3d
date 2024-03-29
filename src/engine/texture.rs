extern crate image;
extern crate opengl as gl;

use std::mem;

#[derive(Copy, Clone, Debug)]
pub struct Texture {
    texture_id: u32,
}

// TODO: do we have memory leaks without this?
// impl Drop for Texture {
//     fn drop(&mut self) {
//         unsafe { gl::DeleteTextures(1, &mut self.texture_id) } ;
//     }
// }

impl Texture {
    pub fn new(texture_path: &str, anisotropy: f32) -> Texture {
        let mut texture_id = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // texture wrapping
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            // texture filtering
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            println!("Opening image: {}", texture_path);
            let texture_data = image::open(texture_path)
                .expect("Opening image for texture failed")
                .to_rgba();

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                texture_data.width() as i32,
                texture_data.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                mem::transmute(&texture_data.into_raw()[0]),
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);

            let mut max_anisotropy = 16.0;
            gl::GetFloatv(gl::MAX_TEXTURE_MAX_ANISOTROPY_EXT, &mut max_anisotropy);

            let current_anisotropy = if anisotropy > max_anisotropy {
                max_anisotropy
            } else if anisotropy < 0.0 {
                0.0
            } else {
                anisotropy
            };

            println!(
                "Current anisotropy for {:?}: {:?}",
                texture_path, current_anisotropy
            );

            gl::TexParameterf(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAX_ANISOTROPY_EXT,
                current_anisotropy,
            );

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture { texture_id }
    }

    pub fn bind(&self, location: u32) {
        unsafe {
            gl::ActiveTexture(location);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }
}
