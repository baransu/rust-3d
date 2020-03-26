extern crate math;

use self::math::mat4::Mat4;
use self::math::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    pub position: Vec3,
    // euler angles
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Transform {
        Transform {
            position: position,
            rotation: rotation,
            scale: scale,
        }
    }

    pub fn get_model_matrix(self) -> Mat4 {
        let scale_matrix = Mat4::from_scale(&self.scale);
        let rotation_matrix = Mat4::from_rotation(&self.rotation);
        let translation_matrix = Mat4::from_translation(&self.position);

        scale_matrix * rotation_matrix * translation_matrix
    }
}
