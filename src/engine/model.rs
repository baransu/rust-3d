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
    // pub texcoord: Vec2,
    pub normal: Vec3,
}

impl Mod {
    pub fn new(path_str: &str) -> Mod {
        let path = Path::new(path_str);

        let object = tobj::load_obj(path);
        let (models, materials) = object.unwrap();

        let mesh = &models[0].mesh;

        let mut container: Vec<Vertex> = Vec::new();

        println!("{:?}, {:?}, {:?}", mesh.positions.len(), mesh.normals.len(), mesh.indices.len());

        for i in 0..mesh.positions.len()/3 as usize {
            // pos = [x, y, z]
            let pos = Vec3::new(mesh.positions[i * 3], mesh.positions[i * 3 + 1], mesh.positions[i * 3 + 2]);
            // let tex = Vec2::new(mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]);
            // normal = [x, y, z]
            let norm = Vec3::new(mesh.normals[i * 3], mesh.normals[i * 3 + 1], mesh.normals[i * 3 + 2]);
            container.push(Vertex { position: pos, normal: norm });
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
        // let mut norm: Vec<Vec3> = Vec::new();
        // let mut indicies: Vec<u32> = Vec::new();
        // let mut elements: Vec<u32> = Vec::new();
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
        //     } else if &line[0..3] == "vn " {
        //         let l = &line[3..];
        //
        //         let splits: Vec<&str> = l.split(' ').collect();
        //
        //         let x: f32 = splits[0].to_string().parse().unwrap();
        //         let y: f32 = splits[1].to_string().parse().unwrap();
        //         let z: f32 = splits[2].to_string().parse().unwrap();
        //
        //         norm.push(Vec3::new(x, y, z));
        //
        //
        //     } else if &line[0..2] == "f " {
        //
        //         let l = &line[2..];
        //
        //         let splits: Vec<&str> = l.split(' ').collect();
        //
        //         let mut a = Model::handle_split(splits[0]);
        //         let mut b = Model::handle_split(splits[0]);
        //         let mut c = Model::handle_split(splits[0]);
        //
        //         // let mut was_brake = false;
        //         // let mut string = String::new();
        //         // for c in splits[0].chars() {
        //         //     if c != '/' {
        //         //         string.push(c);
        //         //     } else {
        //         //         break;
        //         //     }
        //         // }
        //         // let mut a: u16 = string.parse().unwrap();
        //         //
        //         // let mut was_brake = false;
        //         // let mut string = String::new();
        //         // for c in splits[1].chars() {
        //         //     if c != '/' {
        //         //         string.push(c);
        //         //     } else {
        //         //         break;
        //         //     }
        //         // }
        //         // let mut b: u16 = string.parse().unwrap();
        //         //
        //         // let mut was_brake = false;
        //         // let mut string = String::new();
        //         // for c in splits[2].chars() {
        //         //     if c != '/' {
        //         //         string.push(c);
        //         //     } else {
        //         //         break;
        //         //     }
        //         // }
        //         // let mut c: u16 = string.parse().unwrap();
        //
        //         a.0 -= 1;
        //         b.0 -= 1;
        //         c.0 -= 1;
        //
        //         // if a[1] != 0 {
        //         //     a[1] -= 1;
        //         //     elements.push(a[1]);
        //         // }
        //         //
        //         // if b[1] != 0 {
        //         //     b[1] -= 1;
        //         //     elements.push(b[1]);
        //         // }
        //         //
        //         // if c[1] != 0 {
        //         //     c[1] -= 1;
        //         //     elements.push(c[1]);
        //         // }
        //
        //         indicies.push(a.0); indicies.push(b.0); indicies.push(c.0);
        //     }
        // }
        //
        // let mut normals: Vec<Vec3> = Vec::new();
        // normals.resize(vertices.len(), Vec3::new(0.0, 0.0, 0.0));
        //
        // // println!("{:?}, {:?}, {:?}", indicies.len(), vertices.len(), normals.len());
        // //
        // // for i in 0..indicies.len()/3 {
        // //
        // //     // println!("{:?}, {:?}, {:?}", i * 3, i * 3 + 1, i * 3 + 2);
        // //
        // //     let ia: usize = indicies[i * 3] as usize;
        // //     let ib: usize = indicies[i * 3 + 1] as usize;
        // //     let ic: usize = indicies[i * 3 + 2] as usize;
        // //
        // //     // let mut normal: Vec3 = vertices[ib] - vertices[ia];
        // //     let normal = Vec3::cross(vertices[ib] - vertices[ia], vertices[ia] - vertices[ia]);
        // //     // glm::vec3 normal = glm::normalize(glm::cross( glm::vec3(vertices[ib]) - glm::vec3(vertices[ia]), glm::vec3(vertices[ic]) - glm::vec3(vertices[ia])));
        // //     normals[ia] = normal;
        // //     normals[ib] = normal;
        // //     normals[ic] = normal;
        // // }

        // let mut v: Vec<Vertex> = Vec::new();
        // for i in 0..vertices.len() {
        //     let vertex = Vertex { position: vertices[i], normal: normals[i] };
        //     v.push(vertex);
        // }
        //
        // Model {vertices: v, indicies: indicies }
    }

    fn handle_split(split: &str) -> (u32, u32) {
        let mut slashes = 0;

        let mut string1 = String::new();
        let mut string2 = String::new();

        println!("{:?}", split);

        for c in split.chars() {
            if slashes == 0 && c != '/'{
                string1.push(c);
            } else if slashes == 2 {
                string2.push(c);
            } else if c == '/'{
                slashes += 1;
            }
        }

        println!("{:?}, {:?}", string1, string2);

        let a: u32 = string1.parse().unwrap();
        let b: u32 = string2.parse().unwrap();
        ( a, b )
    }
}
