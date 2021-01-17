extern crate math;

use self::math::mat4::Mat4;
use self::math::vec3::Vec3;
use model::Model;
use shader::ShaderProgram;
use transform::Transform;

pub struct DirLight {
    pub direction: Vec3,

    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
}

impl DirLight {
    pub fn new(direction: Vec3, ambient: Vec3, diffuse: Vec3, specular: Vec3) -> DirLight {
        DirLight {
            direction: direction,
            ambient: ambient,
            diffuse: diffuse,
            specular: specular,
        }
    }
}

pub struct PointLight {
    pub position: Vec3,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,

    // transform: Transform,
    model: Model,
    shader: ShaderProgram,
}

impl PointLight {
    pub fn new(
        position: Vec3,
        linear: f32,
        quadratic: f32,
        ambient: Vec3,
        diffuse: Vec3,
        specular: Vec3,
    ) -> PointLight {
        let shader = ShaderProgram::new("res/lightShader.vert", "res/lightShader.frag");
        let model = Model::from_obj("res/models/cube.obj");

        // setup model and shaders for rendering
        PointLight {
            position: position,

            constant: 1.0,
            linear: linear,
            quadratic: quadratic,

            ambient: ambient,
            diffuse: diffuse,
            specular: specular,

            // transform: transform,
            shader: shader,
            model: model,
        }
    }

    pub fn render(&self, projection: Mat4, view: Mat4) {
        self.shader.bind();

        let transform = Transform::new(
            self.position,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.25, 0.25, 0.25),
        );

        // ligth color
        self.shader.set_uniform_3f("lightColor", self.diffuse);

        self.shader.set_uniform_matrix4fv("projection", projection);
        self.shader.set_uniform_matrix4fv("view", view);
        self.shader
            .set_uniform_matrix4fv("model", transform.get_model_matrix());

        self.model.render();

        self.shader.unbind();
    }
}
