extern crate math;
extern crate tobj;

use self::math::vec3::Vec3;
use self::math::vec2::Vec2;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
// use std::str;

use self::tobj::*;

pub struct Mod {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    // materials and other stuff
}

// #[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub texcoord: Vec2,
    pub normal: Vec3,
    pub tangent: Vec3,
    pub binormal: Vec3,
}

impl Mod {
    pub fn new(path_str: &str) -> Mod {
        let path = Path::new(path_str);

        let object = tobj::load_obj(path);
        let (models, materials) = object.unwrap();

        let mesh = &models[0].mesh;

        let mut container: Vec<Vertex> = Vec::new();

        println!("{:?}, {:?}, {:?}, {:?}", mesh.positions.len(), mesh.texcoords.len(), mesh.normals.len(), mesh.indices.len());

        for i in 0..mesh.positions.len()/3 as usize {
            // pos = [x, y, z]
            let pos = Vec3::new(mesh.positions[i * 3], mesh.positions[i * 3 + 1], mesh.positions[i * 3 + 2]);
            // uv = [x, y]
            let tex = Vec2::new(mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]);
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

        Mod {vertices: container, indices: mesh.indices.clone() }

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
        //         let mut a = Mod::handle_split(splits[0]);
        //         let mut b = Mod::handle_split(splits[0]);
        //         let mut c = Mod::handle_split(splits[0]);
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
        // Mod {vertices: v, indices: indices }
    }

    fn handle_split(split: &str) -> u32 {
        let mut slashes = 0;

        let mut string1 = String::new();
        let mut string2 = String::new();

        // println!("{:?}", split);

        for c in split.chars() {
            if slashes == 0 && c != '/'{
                string1.push(c);
            } else if slashes == 2 {
                string2.push(c);
            } else if c == '/'{
                slashes += 1;
            }
        }

        // println!("{:?}, {:?}", string1, string2);

        let a: u32 = string1.parse().unwrap();
        // let b: u32 = string2.parse().unwrap();
        a
    }
}
