use super::{
    buffers::{Buffer, EBO, VAO, VBO},
    vertex::{ModelVertex, Vertex},
};
pub mod primitives;
use glam::Vec3;
//TODO:Load of material with part drawing 
#[derive(Debug, Clone)]
pub struct Model<V: Vertex> {
    pub verticies: Vec<V>,
    pub indicies: Option<Vec<u32>>,
}
impl<V: Vertex> Model<V> {
    pub fn new(vertexes: Vec<V>, indicies: Option<Vec<u32>>) -> Self {
        Self { verticies: vertexes, indicies }
    }
    pub fn instantiate(&self) -> InstancedModel {
        InstancedModel::new(&self)
    }
}

pub fn from_str(source:&str) -> Option<Model<ModelVertex>> {
    fn calc_normal(a: &[f32; 3], b: &[f32; 3], c: &[f32; 3]) -> [f32; 3] {
        let a_side = Vec3::new(a[0], a[1], a[2]) - Vec3::new(b[0], b[1], b[2]);
        let c_side = Vec3::new(b[0], b[1], b[2]) - Vec3::new(c[0], c[1], c[2]);
        a_side.cross(c_side).normalize().into()
    }
    let mut v_pos = Vec::new();
    let mut v_normal = Vec::new();
    let mut v_uv = Vec::new();
    let mut indicies = Vec::new();
    let mut vertexes = Vec::new();
    for line in source.lines() {
        let mut separated = line.split(" ");
        match separated.next()? {
            "v" => v_pos.push([
                separated.next()?.parse::<f32>().ok()?,
                separated.next()?.parse::<f32>().ok()?,
                separated.next()?.parse::<f32>().ok()?,
            ]),
            "vn" => v_normal.push([
                separated.next()?.parse::<f32>().ok()?,
                separated.next()?.parse::<f32>().ok()?,
                separated.next()?.parse::<f32>().ok()?,
            ]),
            "vt" => v_uv.push([
                separated.next()?.parse::<f32>().ok()?,
                separated.next()?.parse::<f32>().ok()?,
            ]),
            "f" => {
                let mut positions = [0; 3];
                let mut normals = [None; 3];
                let mut uvs = [None; 3];
                for i in 0..3 {
                    let mut vertex_indexes = separated.next()?.split("/");
                    positions[i] = vertex_indexes.next()?.parse::<u32>().ok()? - 1;
                    if let Some(uv) = vertex_indexes.next() {
                        if let Some(idx) = uv.parse::<u32>().ok() {
                            uvs[i] = Some(idx - 1);
                        }
                    }
                    if let Some(normal) = vertex_indexes.next() {
                        if let Some(idx) = normal.parse::<u32>().ok() {
                            normals[i] = Some(idx - 1);
                        }
                    }
                }
                match (true, normals[0].is_some(), uvs[0].is_some()) {
                    (true, true, true) => {
                        for i in 0..3 {
                            vertexes.push(ModelVertex::new(
                                v_pos[positions[i] as usize],
                                v_normal[normals[i].unwrap() as usize],
                                v_uv[uvs[i].unwrap() as usize],
                            ));
                            indicies.push(indicies.len() as u32);
                        }
                    }
                    (true, true, false) => {
                        for i in 0..3 {
                            vertexes.push(ModelVertex::new(
                                v_pos[positions[i] as usize],
                                v_normal[normals[i].unwrap() as usize],
                                [0.0, 0.0],
                            ));
                            indicies.push(indicies.len() as u32);
                        }
                    }
                    (true, false, true) => {
                        let v_normal = calc_normal(
                            &v_pos[positions[0] as usize],
                            &v_pos[positions[1] as usize],
                            &v_pos[positions[2] as usize],
                        );
                        for i in 0..3 {
                            vertexes.push(ModelVertex::new(
                                v_pos[positions[i] as usize],
                                v_normal,
                                v_uv[uvs[i].unwrap() as usize],
                            ));
                            indicies.push(indicies.len() as u32);
                        }
                    }
                    (true, false, false) => {
                        let v_normal = calc_normal(
                            &v_pos[positions[0] as usize],
                            &v_pos[positions[1] as usize],
                            &v_pos[positions[2] as usize],
                        );
                        for i in 0..3 {
                            vertexes.push(ModelVertex::new(
                                v_pos[positions[i] as usize],
                                v_normal,
                                [0.0, 0.0],
                            ));
                            indicies.push(indicies.len() as u32);
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
    return Some(Model::new(vertexes, Some(indicies)))
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstancedModel {
    vao: VAO,
    vbo: Buffer<VBO>,
    ebo: Option<Buffer<EBO>>,
    vertex_count: i32,
}
impl InstancedModel {
    pub fn new<T: Vertex>(model: &Model<T>) -> Self {
        let vao = VAO::new();
        vao.bind();
        let vbo: Buffer<VBO> = Buffer::gen();
        let ebo: Buffer<EBO> = Buffer::gen();
        vbo.bind();
        vbo.set_data(&model.verticies);
        let mut len = model.verticies.len();
        if let Some(indicies) = model.indicies.as_ref(){
            len = indicies.len();
            ebo.bind();
            ebo.set_data(&indicies);
        }
        T::declaration();
        Self {
            vao,
            vbo,
            ebo:Some(ebo),
            vertex_count: len as i32,
        }
    }
    pub fn new_without_vertex(vertex_count:i32) ->Self{
        let vao = VAO::new();
        vao.bind();
        let vbo: Buffer<VBO> = Buffer::gen();
        vbo.bind();
        Self { vao, vbo, ebo: None, vertex_count }
    }
    pub fn draw(&self) {
        unsafe {
            self.vao.bind();
            if self.ebo.is_some(){
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.vertex_count,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            }
            else{
                gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
            }
        }
    }
    pub fn draw_instanced(&self,instance_count:i32) {
        
        unsafe {
            self.vao.bind();
            if self.ebo.is_some(){
                gl::DrawElementsInstanced(
                    gl::TRIANGLES,
                    self.vertex_count,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                    instance_count
                );
            }
            else{
                gl::DrawArraysInstanced(gl::TRIANGLES, 0, self.vertex_count,instance_count);
            }
        }
    }
}
