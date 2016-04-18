extern crate math;
extern crate tobj;
extern crate opengl as gl;

use self::math::vec3::Vec3;
use self::math::vec2::Vec2;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
// use std::str;
use std::mem;
use std::ptr;
use std::collections::HashMap;

use self::gl::types::*;
use self::tobj::*;

use texture::Texture;

pub struct Modell {
    meshes: Vec<Meshh>,
    textures: HashMap<String, Texture>,
    // materials and other stuff
}

struct Meshh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    vao: u32,
    vbo: u32,
    ebo: u32,
    albedo: Option<String>,
    specular: Option<String>,
    normal: Option<String>,
    // materials and stuff
}

// #[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub texcoord: Vec2,
    pub normal: Vec3,
    pub tangent: Vec3,
    pub binormal: Vec3,
}

impl Modell {
    pub fn new(folder_path: &str, file_path: &str) -> Modell {
        let mut model_path = String::new();
        model_path.push_str(folder_path);
        model_path.push_str(file_path);
        let path = Path::new(model_path.as_str());

        let object = tobj::load_obj(path);
        let (models, materials) = object.unwrap();

        let mut meshes: Vec<Meshh> = Vec::new();

        let mut textures: HashMap<String, Texture> = HashMap::new();

        for m in materials.iter() {
            let material = m.clone();

            // diffuse
            if material.diffuse_texture.len() > 0 {

                let texture = match textures.get(&material.diffuse_texture) {
                    Some(_) => None,
                    None => {
                        let mut path = String::new();
                        path.push_str(folder_path);
                        path.push_str(material.diffuse_texture.as_str());
                        Some(Texture::new(path.as_str(), 4.0))
                    },
                };

                match texture {
                    Some(t) => {
                        textures.insert(material.diffuse_texture, t);
                    },
                    None => (),
                };

            }

            // specular
            if material.specular_texture.len() > 0 {
                let texture = match textures.get(&material.specular_texture) {
                    Some(_) => None,
                    None => {
                        let mut path = String::new();
                        path.push_str(folder_path);
                        path.push_str(material.specular_texture.as_str());
                        Some(Texture::new(path.as_str(), 4.0))
                    },
                };

                match texture {
                    Some(t) => {
                        textures.insert(material.specular_texture, t);
                    },
                    None => (),
                };

            }

            let mut normal_path = String::new();
            for (k, v) in &material.unknown_param {
                if k == "map_Bump" {
                    normal_path = v.clone();
                    break;
                }
            }

            // normal
            // bump
            if normal_path.len() > 0 {
                let texture = match textures.get(&normal_path) {
                    Some(_) => None,
                    None => {
                        let mut path = String::new();
                        path.push_str(folder_path);
                        path.push_str(normal_path.as_str());
                        Some(Texture::new(path.as_str(), 4.0))
                    },
                };

                match texture {
                    Some(t) => {
                        textures.insert(normal_path, t);
                    },
                    None => (),
                };
            } else if material.normal_texture.len() > 0 {
                let texture = match textures.get(&material.normal_texture) {
                    Some(_) => None,
                    None => {
                        let mut path = String::new();
                        path.push_str(folder_path);
                        path.push_str(material.normal_texture.as_str());
                        Some(Texture::new(path.as_str(), 4.0))
                    },
                };

                match texture {
                    Some(t) => {
                        textures.insert(material.normal_texture, t);
                    },
                    None => (),
                };
            }
        }


        for j in 0..models.len() {
            let mesh = &models[j].mesh;

            let mut container: Vec<Vertex> = Vec::new();

            println!("{:?}, {:?}, {:?}, {:?}", mesh.positions.len(), mesh.texcoords.len(), mesh.normals.len(), mesh.indices.len());

            for i in 0..mesh.positions.len()/3 as usize {
                // pos = [x, y, z]
                let pos = Vec3::new(mesh.positions[i * 3], mesh.positions[i * 3 + 1], mesh.positions[i * 3 + 2]);

                // uv = [x, y]
                let mut tex = Vec2::new(0.0, 0.0);
                if !mesh.texcoords.is_empty() {
                    tex = Vec2::new(mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]);
                }

                // normal = [x, y, z]
                let norm = Vec3::new(mesh.normals[i * 3], mesh.normals[i * 3 + 1], mesh.normals[i * 3 + 2]);

                container.push(Vertex {
                    position: pos,
                    texcoord: tex,
                    normal: norm,
                    tangent: Vec3::new(0.0, 0.0, 0.0),
                    binormal: Vec3::new(0.0, 0.0, 0.0)
                });
            }

            for i in 0..mesh.indices.len()/3 {

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
            let mat_id: usize = match mesh.material_id {
                Some(id) => id,
                None => { panic!("no material_id"); },
            };
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

            let mut m = Meshh {
                vertices: container,
                indices: mesh.indices.clone(),
                vao: 0,
                vbo: 0,
                ebo: 0,
                albedo: diffuse,
                specular: specular,
                normal: normal
            };

            m.init();
            meshes.push(m);
        }

        // for (i, m) in materials.iter().enumerate() {
        //     println!("material[{}].name = \'{}\'", i, m.name);
        //     println!("    material.Ka = ({}, {}, {})", m.ambient[0], m.ambient[1], m.ambient[2]);
        //     println!("    material.Kd = ({}, {}, {})", m.diffuse[0], m.diffuse[1], m.diffuse[2]);
        //     println!("    material.Ks = ({}, {}, {})", m.specular[0], m.specular[1], m.specular[2]);
        //     println!("    material.Ns = {}", m.shininess);
        //     println!("    material.d = {}", m.dissolve);
        //     println!("    material.map_Ka = {}", m.ambient_texture);
        //     println!("    material.map_Kd = {}", m.diffuse_texture);
        //     println!("    material.map_Ks = {}", m.specular_texture);
        //     println!("    material.map_Ns = {}", m.normal_texture);
        //     println!("    material.map_d = {}", m.dissolve_texture);
        //     for (k, v) in &m.unknown_param {
        //         println!("    material.{} = {}", k, v);
        //     }
        // }

        Modell { meshes: meshes, textures: textures}

        // let f = match File::open(&path) {
        //     Ok(file) => file,
        //     Err(err) => panic!("Coudn not open {}: {}", path.display(), Error::description(&err)),
        // };
        //
        // let file = BufReader::new(&f);
        //
        // let mut vertices: Vec<Vec3> = Vec::new();
        // // let mut norm: Vec<Vec3> = Vec::new();
        // let mut indices: Vec<u32> = Vec::new();
        // // let mut elements: Vec<u32> = Vec::new();
        // let mut normals: Vec<Vec3> = Vec::new();
        //
        // for l in file.lines() {
        //     let line = l.unwrap();
        //
        //     if &line[0..2] == "v " {
        //         let l = &line[2..];
        //
        //         let splits: Vec<&str> = l.split(' ').collect();
        //
        //         let x: f32 = splits[0].to_string().parse().unwrap();
        //         let y: f32 = splits[1].to_string().parse().unwrap();
        //         let z: f32 = splits[2].to_string().parse().unwrap();
        //
        //         vertices.push(Vec3::new(x, y, z));
        //
        //     // } else if &line[0..3] == "vn " {
        //     //     let l = &line[3..];
        //     //
        //     //     let splits: Vec<&str> = l.split(' ').collect();
        //     //
        //     //     let x: f32 = splits[0].to_string().parse().unwrap();
        //     //     let y: f32 = splits[1].to_string().parse().unwrap();
        //     //     let z: f32 = splits[2].to_string().parse().unwrap();
        //     //
        //     //     norm.push(Vec3::new(x, y, z));
        //
        //
        //     } else if &line[0..2] == "f " {
        //
        //         let l = &line[2..];
        //
        //         let splits: Vec<&str> = l.split(' ').collect();
        //
        //         let mut a = Modell::handle_split(splits[0]);
        //         let mut b = Modell::handle_split(splits[0]);
        //         let mut c = Modell::handle_split(splits[0]);
        //
        //         a -= 1;
        //         b -= 1;
        //         c -= 1;
        //
        //         indices.push(a); indices.push(b); indices.push(c);
        //     }
        // }
        //
        // normals.resize(vertices.len(), Vec3::new(0.0, 0.0, 0.0));
        //
        // for i in 0..indices.len()/3 {
        //
        //     // println!("{:?}, {:?}, {:?}", i * 3, i * 3 + 1, i * 3 + 2);
        //
        //     let ia: usize = indices[i * 3] as usize;
        //     let ib: usize = indices[i * 3 + 1] as usize;
        //     let ic: usize = indices[i * 3 + 2] as usize;
        //
        //     // let mut normal: Vec3 = vertices[ib] - vertices[ia];
        //     let normal = Vec3::cross(vertices[ib] - vertices[ia], vertices[ic] - vertices[ia]);
        //     // glm::vec3 normal = glm::normalize(glm::cross( glm::vec3(vertices[ib]) - glm::vec3(vertices[ia]), glm::vec3(vertices[ic]) - glm::vec3(vertices[ia])));
        //     normals[ia] = normal;
        //     normals[ib] = normal;
        //     normals[ic] = normal;
        // }
        //
        // let mut v: Vec<Vertex> = Vec::new();
        // for i in 0..vertices.len() {
        //     let vertex = Vertex { position: vertices[i], normal: normals[i] };
        //     v.push(vertex);
        // }
        //
        // Modell {vertices: v, indices: indices }
    }
    // fn handle_split(split: &str) -> u32 {
    //     let mut slashes = 0;
    //
    //     let mut string1 = String::new();
    //     let mut string2 = String::new();
    //
    //     // println!("{:?}", split);
    //
    //     for c in split.chars() {
    //         if slashes == 0 && c != '/'{
    //             string1.push(c);
    //         } else if slashes == 2 {
    //             string2.push(c);
    //         } else if c == '/'{
    //             slashes += 1;
    //         }
    //     }
    //
    //     // println!("{:?}, {:?}", string1, string2);
    //
    //     let a: u32 = string1.parse().unwrap();
    //     // let b: u32 = string2.parse().unwrap();
    //     a
    // }

    pub unsafe fn draw(&self) {
        for i in 0..self.meshes.len() {
            //TODO: shaders

            // textures
            match self.meshes[i].albedo {
                Some(ref albedo) => {
                    match self.textures.get(albedo) {
                        Some(texture) => texture.bind(gl::TEXTURE0),
                        None => (),
                    }
                },
                None => (),
            };

            match self.meshes[i].specular {
                Some(ref specular) => {
                    match self.textures.get(specular) {
                        Some(texture) => texture.bind(gl::TEXTURE1),
                        None => (),
                    }
                },
                None => (),
            };

            match self.meshes[i].normal {
                Some(ref normal) => {
                    match self.textures.get(normal) {
                        Some(texture) => texture.bind(gl::TEXTURE2),
                        None => (),
                    }
                },
                None => (),
            };

            self.meshes[i].draw();
        }
    }

    pub unsafe fn draw_without_textures(&self) {
        for i in 0..self.meshes.len() {
            //TODO: shaders
            self.meshes[i].draw();
        }
    }

}

impl Drop for Meshh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

impl Meshh {
    fn init(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);

            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (self.vertices.len() * mem::size_of::<Vertex>()) as GLsizeiptr, mem::transmute(&self.vertices[0]), gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (self.indices.len() * mem::size_of::<u32>()) as GLsizeiptr, mem::transmute(&self.indices[0]), gl::STATIC_DRAW);

            // pos
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE as GLboolean, mem::size_of::<Vertex>() as i32, ptr::null());
            // uvs
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE as GLboolean, mem::size_of::<Vertex>() as i32, mem::transmute(3 * mem::size_of::<f32>()));
            // normal
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE as GLboolean, mem::size_of::<Vertex>() as i32, mem::transmute(5 * mem::size_of::<f32>()));
            // tangent
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(3, 3, gl::FLOAT, gl::FALSE as GLboolean, mem::size_of::<Vertex>() as i32, mem::transmute(8 * mem::size_of::<f32>()));
            // binormal
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribPointer(4, 3, gl::FLOAT, gl::FALSE as GLboolean, mem::size_of::<Vertex>() as i32, mem::transmute(11 * mem::size_of::<f32>()));

            gl::BindVertexArray(0);
        }

    }

    unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
        gl::BindVertexArray(0);
    }
}
