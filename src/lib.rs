// lib.rs
//
// Copyright (c) 2022 Jeremiah LaRocco <jeremiah_larocco@fastmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

pub mod obj {
    use std::collections::HashMap;
    use std::vec::Vec;
    use std::fs::File;
    use std::string::String;
    use std::io::{prelude::*, BufReader};
    use std::num::ParseIntError;
    use std::num::ParseFloatError;
    use std::path::Path;

    /// An OBJ file material, from a .mtl file
    /// A name, and a dictionary of material attributes
    #[derive(Debug)]
    pub struct ObjMaterial {
        pub file_name : String,
        pub name : String,
        pub attributes : HashMap<String, MtlAttribute>
    }

    /// An OBJ file material attribute.
    /// An Integer, Float, or RGBA color
    #[derive(Debug)]
    pub enum MtlAttribute {
        Integer(i32),
        Float(f32),
        String(String),
        Color {
            red : f32,
            green : f32,
            blue : f32,
            alpha : f32
        },
    }

    /// Indices into an ObjObject's vertex data
    #[derive(Debug)]
    pub enum Index {
        Vertex{v: u32},
        VertexNormal{v: u32,
                     n: u32},
        VertexTexture{v: u32,
                      t: u32},
        VertexTextureNormal{v: u32,
                            t: u32,
                            n: u32}
    }

    #[derive(Debug)]
    pub struct ObjGroup {
        pub name : String,
        pub smoothing_group : u32,
        pub material : String,
        pub faces : Vec<Index>,
        pub lines : Vec<Index>,
        pub points : Vec<u32>
    }

    #[derive(Debug)]
    pub struct ObjObject {
        pub name : String,
        pub vertices : Vec<f32>,
        pub normals : Vec<f32>,
        pub textures : Vec<f32>,
        pub parameters : Vec<f32>,
        pub groups : Vec<ObjGroup>
    }

    #[derive(Debug)]
    pub struct ObjFile {
        pub file_name : String,
        pub objects : Vec<ObjObject>,
        pub materials : HashMap<String, ObjMaterial>,
    }

    fn read_v( obj : &mut ObjFile , line : &str) {
        for pt in line.split_whitespace() {
            let value : f32 = pt.parse().unwrap();
            obj.objects.last_mut().unwrap().vertices.push(value);
        }
    }

    fn read_vn(obj : &mut ObjFile, line : &str) {
        for pt in line.split_whitespace() {
            let value : f32 = pt.parse().unwrap();
            obj.objects.last_mut().unwrap().normals.push(value);
        }
    }

    fn read_vp(obj : &mut ObjFile, line : &str) {
        for pt in line.split_whitespace() {
            let value : f32 = pt.parse().unwrap();
            obj.objects.last_mut().unwrap().parameters.push(value);
        }
    }

    fn read_vt(obj : &mut ObjFile, line : &str) {
        for pt in line.split_whitespace() {
            let value : f32 = pt.parse().unwrap();
            obj.objects.last_mut().unwrap().textures.push(value);
        }
    }

    fn read_group(obj : &mut ObjFile, line : &str) {
        obj.objects.last_mut().unwrap().groups.push(ObjGroup{ name: line.to_string(),
                                                              smoothing_group : 0,
                                                              material: "".to_string(),
                                                              faces : Vec::new(),
                                                              lines : Vec::new(),
                                                              points : Vec::new()});
    }

    fn read_usemtl(obj : &mut ObjFile, line : &str) {
        obj.objects.last_mut().unwrap().groups.last_mut().unwrap().material = line.to_string();
    }

    fn read_face(obj : &mut ObjFile, line : &str) {

        let tri_verts : Vec<Vec<Result<u32, ParseIntError>>> =
            line.split(char::is_whitespace).map(
                | x |
                x.split("/").map(
                    | y |
                    y.parse()).collect()).collect();

        if tri_verts.len() != 3 {
            return ();
        }
        for next_vert in tri_verts {
            match &next_vert[..] {

                [Ok(vert)] =>
                    obj.objects.last_mut().unwrap().groups.last_mut().unwrap().faces.push(Index::Vertex{v: *vert}),
                [Ok(vert), Ok(text)] =>
                    obj.objects.last_mut().unwrap().groups.last_mut().unwrap().faces.push(Index::VertexTexture{v: *vert,
                                                                                                               t: *text}),
                [Ok(vert), Err(_), Ok(norm)] =>
                    obj.objects.last_mut().unwrap().groups.last_mut().unwrap().faces.push(Index::VertexNormal{v: *vert,
                                                                                                              n: *norm}),
                [Ok(vert), Ok(text), Ok(norm)] =>
                    obj.objects.last_mut().unwrap().groups.last_mut().unwrap().faces.push(Index::VertexTextureNormal{v: *vert,
                                                                                                                     t: *text,
                                                                                                                     n: *norm}),
                _ => {
                    return ();
                }
            }
        }
    }

    fn read_line(obj : &mut ObjFile, line : &str) {
        let verts : Vec<Result<u32, ParseIntError>> = line.split("/").map(| x | x.parse()).collect();

        match &verts[..] {
            [Ok(vert)] =>
                obj.objects.last_mut().unwrap().groups.last_mut().unwrap().lines.push(Index::Vertex{v: *vert}),

            [Ok(vert), Ok(text)] =>
                obj.objects.last_mut().unwrap().groups.last_mut().unwrap().faces.push(Index::VertexTexture{v: *vert,
                                                                                                          t: *text}),

            _ => {
                return ();
            }
        }
    }

    fn read_point(obj : &mut ObjFile, line : &str) {
        let verts : Vec<Result<u32, ParseIntError>> = line.split(char::is_whitespace).map(| x | x.parse()).collect();
        match &verts[..] {
            [Ok(vert)] =>
                obj.objects.last_mut().unwrap().groups.last_mut().unwrap().points.push(*vert),
            _ => {
                return ();
            }
        }
    }

    fn read_smoothing_group(obj : &mut ObjFile, line : &str) {
        let value : u32 = line.parse().unwrap();
        obj.objects.last_mut().unwrap().groups.last_mut().unwrap().smoothing_group = value;
    }

    fn remove_comments(line : &str) -> &str {
        let comment_start = line.find("#").unwrap_or(line.len());
        return line[0..comment_start].trim();
    }

    fn handle_mtl_entry(mat : &mut ObjMaterial, key : &str, value : &str) {
        if key == "newmtl" {
            mat.name = value.to_string();
        }
        else if ["illum"].contains(&key) {
            mat.attributes.insert(key.to_string(),
                                  MtlAttribute::Integer(value.parse().unwrap_or(0)));
        }
        else if ["Ns", "d", "Tr", "Ni", ].contains(&key) {
            mat.attributes.insert(key.to_string(),
                                  MtlAttribute::Float(value.parse().unwrap_or(0.0)));
        }
        else if ["Kd", "Ka", "Ks", "Ke", "Tf"].contains(&key) {
            let as_color : Vec<Result<f32, ParseFloatError>> =
                value.split(char::is_whitespace).map(| x |
                                                     x.parse()).collect();

            match &as_color[..] {
                [Ok(red), Ok(green), Ok(blue)] =>
                    mat.attributes.insert(key.to_string(),
                                          MtlAttribute::Color{red: *red, green: *green, blue: *blue, alpha: 1.0}),
                [Ok(red), Ok(green), Ok(blue), Ok(alpha)] =>
                    mat.attributes.insert(key.to_string(),
                                          MtlAttribute::Color{red: *red, green: *green, blue: *blue, alpha: *alpha}),
                _ => {
                    mat.attributes.insert(key.to_string(),
                                          MtlAttribute::Color{red:1.0, green:1.0, blue:1.0, alpha:1.0})
                }
            };
        }
        else {
            mat.attributes.insert(key.to_string(),
                                  MtlAttribute::String(value.to_string()));
        }
    }

    fn read_mtllib(obj : &mut ObjFile, line : &str) {
        if let Some(directory) = Path::new(&obj.file_name).parent() {
            let full_name = directory.join(line);
            if let Some(mat) = mtl_from_file(full_name.as_path()) {
                let tmp : String = mat.name[..].to_string();
                obj.materials.insert(tmp, mat);
            } else {
                return ();
            }
        }
        ();
    }

    fn read_comment(_obj : &mut ObjFile, _line : &str) {
        // Ignore.  This is probably never called...
    }

    fn read_object(obj : &mut ObjFile, line : &str) {
        obj.objects.push(ObjObject{name: line.to_string(),
                                   vertices : Vec::new(),
                                   normals : Vec::new(),
                                   textures : Vec::new(),
                                   parameters : Vec::new(),
                                   groups : Vec::new()
        });
    }

    type ObjParser = fn(&mut ObjFile, &str) -> ();

    pub fn mtl_from_file(full_name : &Path) -> Option<ObjMaterial> {
        let mut mat = ObjMaterial {
            file_name: full_name.to_string_lossy().to_string(),
            name: "".to_string(),
            attributes : HashMap::new()
        };

        if let Ok(file) = File::open(full_name) {
            let reader = BufReader::new(file);
            for rline in reader.lines() {
                let line = rline.unwrap();
                if line.starts_with("#") { continue }
                let mut tokens = line.splitn(2, char::is_whitespace);
                if let Some(key) = tokens.next() {
                    if let Some(value) = tokens.next() {
                        handle_mtl_entry(&mut mat, key, remove_comments(&value));
                    }
                }
            }
            return Some(mat);
        }
        return None;
    }

    pub fn obj_from_file(file_name : String) -> Option<ObjFile> {
        let mut obj_file = ObjFile {
            file_name: file_name,
            objects: Vec::new(),
            materials: HashMap::new()
        };
        let parsers = HashMap::from(
            [("v".to_string(), read_v as ObjParser),
             ("vn".to_string(), read_vn as ObjParser),
             ("vp".to_string(), read_vp as ObjParser),
             ("#".to_string(), read_vp as ObjParser),
             ("vt".to_string(), read_vt as ObjParser),
             ("g".to_string(), read_group as ObjParser),
             ("usemtl".to_string(), read_usemtl as ObjParser),
             ("f".to_string(), read_face as ObjParser),
             ("l".to_string(), read_line as ObjParser),
             ("p".to_string(), read_point as ObjParser),
             ("o".to_string(), read_object as ObjParser),
             ("s".to_string(), read_smoothing_group as ObjParser),
             ("#".to_string(), read_comment as ObjParser),
             ("mtllib".to_string(), read_mtllib  as ObjParser),
            ]);


        if let Ok(file) = File::open(Path::new(&obj_file.file_name)) {
            let reader = BufReader::new(file);

            for rline in reader.lines() {
                let line = rline.unwrap();
                if line.starts_with("#") { continue }
                let mut tokens = line.splitn(2, char::is_whitespace);
                if let Some(opcode) = tokens.next() {
                    if let Some(parser) = parsers.get(&String::from(opcode)) {
                        if let Some(rest) = &tokens.next() {
                            parser(&mut obj_file, remove_comments(&rest));
                        }
                    }
                }
            }
        }
        return Some(obj_file);
    }
}
