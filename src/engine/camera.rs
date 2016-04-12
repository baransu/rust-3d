extern crate math;

use self::math::vec3::Vec3;
use self::math::mat4::Mat4;

pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, rotation: Vec3) -> Camera {
        Camera {
            position: position,
            rotation: rotation,
            forward: Vec3::new(0.0, 0.0, -1.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn get_look_at_target_matrix(&mut self, target: Vec3) -> Mat4 {

        let roll = self.rotation.x;

        self.up = Vec3::new(0.0, 1.0, 0.0);
        self.up.x = roll.to_radians().sin();
        self.up.y = roll.to_radians().cos();
        self.up.z = 0.0;

        Mat4::from_look_at(self.position, target, self.up)
    }

    pub fn get_look_at_matrix(&mut self) -> Mat4 {

        let roll = self.rotation.x;
        let pitch = self.rotation.y;
        let yaw = self.rotation.z;

        self.forward.x = pitch.to_radians().cos() * yaw.to_radians().cos();
        self.forward.y = pitch.to_radians().sin();
        self.forward.z = pitch.to_radians().cos() * yaw.to_radians().sin();
        self.forward = self.forward.normalize();

        self.up = Vec3::new(0.0, 1.0, 0.0);
        self.up.x = roll.to_radians().sin();
        self.up.y = roll.to_radians().cos();
        self.up.z = 0.0;

        self.right = Vec3::cross(self.forward, self.up).normalize();
        self.up = Vec3::cross(self.right, self.forward).normalize();

        Mat4::from_look_at(self.position, self.position + self.forward, self.up)
    }
}
