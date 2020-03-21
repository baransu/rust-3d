extern crate math;
extern crate opengl as gl;
extern crate tobj;

use self::gl::types::*;
use self::math::vec2::Vec2;
use self::math::vec3::Vec3;
use self::tobj::*;
use itertools::Itertools;
use texture::Texture;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::mem;
use std::path::{Path, PathBuf};
use std::ptr;

#[derive(Eq, Clone, Debug, PartialEq)]
struct TextureRequest {
    key: String,
    path: String,
}

pub struct Model {
    meshes: Vec<Mesh>,
    textures: HashMap<String, Texture>,
}

struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    vao: u32,
    vbo: u32,
    ebo: u32,
    albedo: Option<String>,
    specular: Option<String>,
    normal: Option<String>,
}

pub struct Vertex {
    pub position: Vec3,
    pub texcoord: Vec2,
    pub normal: Vec3,
    pub tangent: Vec3,
    pub binormal: Vec3,
}

fn get_texture_path(path: &Path, file_name: &str) -> String {
    let mut path_buf = path.to_path_buf();
    path_buf.set_file_name(file_name);
    let raw = path_buf.to_str().unwrap();
    String::from(raw)
}

fn create_texture_request(path: &Path, texture_name: String) -> TextureRequest {
    let key = texture_name.clone();
    let file_name = texture_name.as_str();
    let path = get_texture_path(&path, file_name);
    TextureRequest { key, path }
}

impl Model {
    pub fn new(file_path: &str) -> Model {
        let path = Path::new(file_path);
        let object = tobj::load_obj(path);
        let (models, materials) = object.unwrap();

        let mut meshes: Vec<Mesh> = Vec::new();

        let mut textures: HashMap<String, Texture> = HashMap::new();

        let mut texture_requests: Vec<TextureRequest> = materials
            .iter()
            .flat_map(|m| {
                let material = m.clone();
                let mut acc: Vec<TextureRequest> = Vec::new();

                // diffuse
                if material.diffuse_texture.len() > 0 {
                    acc.push(create_texture_request(&path, material.diffuse_texture));
                }

                // specular
                if material.specular_texture.len() > 0 {
                    acc.push(create_texture_request(&path, material.specular_texture));
                }

                let mut normal_path = String::new();
                for (k, v) in &material.unknown_param {
                    if k == "map_Bump" {
                        normal_path = v.clone();
                        break;
                    }
                }

                // normal / bump
                if normal_path.len() > 0 {
                    acc.push(create_texture_request(&path, normal_path));
                } else if material.normal_texture.len() > 0 {
                    acc.push(create_texture_request(&path, material.normal_texture));
                }

                acc
            })
            .unique_by(|req| req.key.clone())
            .collect();

        for req in texture_requests.iter() {
            let texture = Texture::new(req.path.clone().as_str(), 4.0);
            textures.insert(req.key.clone(), texture);
        }

        for j in 0..models.len() {
            let mesh = &models[j].mesh;
            let mut container: Vec<Vertex> = Vec::new();

            for i in 0..mesh.positions.len() / 3 as usize {
                // pos = [x, y, z]
                let pos = Vec3::new(
                    mesh.positions[i * 3],
                    mesh.positions[i * 3 + 1],
                    mesh.positions[i * 3 + 2],
                );

                // uv = [x, y]
                let mut tex = Vec2::new(0.0, 0.0);
                if !mesh.texcoords.is_empty() {
                    tex = Vec2::new(mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]);
                }

                // normal = [x, y, z]
                let norm = Vec3::new(
                    mesh.normals[i * 3],
                    mesh.normals[i * 3 + 1],
                    mesh.normals[i * 3 + 2],
                );

                container.push(Vertex {
                    position: pos,
                    texcoord: tex,
                    normal: norm,
                    tangent: Vec3::new(0.0, 0.0, 0.0),
                    binormal: Vec3::new(0.0, 0.0, 0.0),
                });
            }

            for i in 0..mesh.indices.len() / 3 {
                let ia: usize = mesh.indices[i * 3] as usize;
                let ib: usize = mesh.indices[i * 3 + 1] as usize;
                let ic: usize = mesh.indices[i * 3 + 2] as usize;

                let pos1 = container[ia].position;
                let pos2 = container[ib].position;
                let pos3 = container[ic].position;

                let uv1 = container[ia].texcoord;
                let uv2 = container[ib].texcoord;
                let uv3 = container[ic].texcoord;

                let edge1 = pos2 - pos1;
                let edge2 = pos3 - pos1;
                let delta_uv1 = uv2 - uv1;
                let delta_uv2 = uv3 - uv1;

                let f = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv2.x * delta_uv1.y);

                let mut tangent = Vec3::new(0.0, 0.0, 0.0);
                tangent.x = f * (delta_uv2.y * edge1.x - delta_uv1.y * edge2.x);
                tangent.y = f * (delta_uv2.y * edge1.y - delta_uv1.y * edge2.y);
                tangent.z = f * (delta_uv2.y * edge1.z - delta_uv1.y * edge2.z);
                tangent = tangent.normalize();

                let mut binormal = Vec3::new(0.0, 0.0, 0.0);
                binormal.x = f * (-delta_uv2.x * edge1.x + delta_uv1.x * edge2.x);
                binormal.y = f * (-delta_uv2.x * edge1.y + delta_uv1.x * edge2.y);
                binormal.z = f * (-delta_uv2.x * edge1.z + delta_uv1.x * edge2.z);
                binormal = binormal.normalize();

                container[ia].tangent = tangent;
                container[ib].tangent = tangent;
                container[ic].tangent = tangent;

                container[ia].binormal = binormal;
                container[ib].binormal = binormal;
                container[ic].binormal = binormal;
            }

            // textures
            let mat_id: usize = mesh.material_id.unwrap();
            let material = materials[mat_id].clone();

            // TODO: implement match
            let mut diffuse = None;
            if material.diffuse_texture.len() > 0 {
                diffuse = Some(material.diffuse_texture);
            }

            let mut specular = None;
            if material.specular_texture.len() > 0 {
                specular = Some(material.specular_texture);
            }

            let mut normal = None;
            let mut normal_path = String::new();
            for (k, v) in &material.unknown_param {
                if k == "map_Bump" {
                    normal_path = v.clone();
                    break;
                }
            }

            if material.normal_texture.len() > 0 {
                normal = Some(material.normal_texture);
            } else if normal_path.len() > 0 {
                normal = Some(normal_path);
            }

            let mut m = Mesh {
                vertices: container,
                indices: mesh.indices.clone(),
                vao: 0,
                vbo: 0,
                ebo: 0,
                albedo: diffuse,
                specular,
                normal,
            };

            m.init();
            meshes.push(m);
        }

        Model { meshes, textures }
    }

    pub unsafe fn draw(&self) {
        for i in 0..self.meshes.len() {
            //TODO: shaders

            // textures
            if let Some(ref albedo) = self.meshes[i].albedo {
                if let Some(texture) = self.textures.get(albedo) {
                    texture.bind(gl::TEXTURE0)
                };
            };

            if let Some(ref specular) = self.meshes[i].specular {
                if let Some(texture) = self.textures.get(specular) {
                    texture.bind(gl::TEXTURE1)
                };
            };

            if let Some(ref normal) = self.meshes[i].normal {
                if let Some(texture) = self.textures.get(normal) {
                    texture.bind(gl::TEXTURE2)
                };
            };

            self.meshes[i].draw();
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

impl Mesh {
    fn init(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);

            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * mem::size_of::<Vertex>()) as GLsizeiptr,
                mem::transmute(&self.vertices[0]),
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * mem::size_of::<u32>()) as GLsizeiptr,
                mem::transmute(&self.indices[0]),
                gl::STATIC_DRAW,
            );

            // pos
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                mem::size_of::<Vertex>() as i32,
                ptr::null(),
            );
            // uvs
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                mem::size_of::<Vertex>() as i32,
                mem::transmute(3 * mem::size_of::<f32>()),
            );
            // normal
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                mem::size_of::<Vertex>() as i32,
                mem::transmute(5 * mem::size_of::<f32>()),
            );
            // tangent
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(
                3,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                mem::size_of::<Vertex>() as i32,
                mem::transmute(8 * mem::size_of::<f32>()),
            );
            // binormal
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribPointer(
                4,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                mem::size_of::<Vertex>() as i32,
                mem::transmute(11 * mem::size_of::<f32>()),
            );

            gl::BindVertexArray(0);
        }
    }

    unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::DrawElements(
            gl::TRIANGLES,
            self.indices.len() as i32,
            gl::UNSIGNED_INT,
            ptr::null(),
        );
        gl::BindVertexArray(0);
    }
}
