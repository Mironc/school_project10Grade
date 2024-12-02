use glam::{ vec3, Vec3};

use crate::{define_vertex, objects::vertex::{Vertex,IntoGLenum,ModelVertex}};
use super::Model;
define_vertex!{SimpleVertex,pos,f32,3}
pub fn icosahedron() -> Model<ModelVertex>{
    let t =(1.0 + 5.0f32.sqrt() ) / 2.0; 
    fn unit_to_sphere(vec:Vec3) -> Vec3{
        let len = (vec.x*vec.x + vec.y*vec.y + vec.z*vec.z).sqrt();
        vec3(vec.x/len, vec.y/len, vec.z/len)
    }
    let mut vertices_positions = Vec::new();
    let indicies = Vec::new();
    let vertices = Vec::new();
    vertices_positions.push(unit_to_sphere(vec3(-1.0,t,0.0)));
    vertices_positions.push(unit_to_sphere(vec3(1.0,t,0.0)));
    vertices_positions.push(unit_to_sphere(vec3(-1.0,-t,0.0)));
    vertices_positions.push(unit_to_sphere(vec3(1.0,-t,0.0)));

    vertices_positions.push(unit_to_sphere(vec3(0.0,-1.0,t)));
    vertices_positions.push(unit_to_sphere(vec3(0.0,1.0,t)));
    vertices_positions.push(unit_to_sphere(vec3(0.0,-1.0,-t)));
    vertices_positions.push(unit_to_sphere(vec3(0.0,1.0,-t)));
    
    vertices_positions.push(unit_to_sphere(vec3(t,0.0,-1.0)));
    vertices_positions.push(unit_to_sphere(vec3(t,0.0,1.0)));
    vertices_positions.push(unit_to_sphere(vec3(-t,0.0,-1.0)));
    vertices_positions.push(unit_to_sphere(vec3(-t,0.0,1.0)));

    Model::new(vertices, Some(indicies))
}