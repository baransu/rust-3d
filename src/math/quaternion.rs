use vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Quaternion {
    pub s: f32,
    pub v: Vec3,
}

impl Quaternion {
    #[inline]
    pub fn new(w: f32, x1: f32, yj: f32, zk: f32) -> Quaternion {
        Quaternion::from_sv(w, Vec3::new(x1, yj, zk))
    }

    #[inline]
    pub fn from_sv(s: f32, v: Vec3) -> Quaternion {
        Quaternion { s: s, v: v }
    }
}
