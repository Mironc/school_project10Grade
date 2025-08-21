use math::Vec3;
use specs::{Component, HashMapStorage, Join, ReadStorage, System, WriteStorage};

use {rendering::mesh_renderer::MeshRenderer, graphics::objects::{model::Model, vertex::{ModelVertex, Vertex}}};

pub struct Morphable<V:Vertex>{
    base_model:Model<V>,
    key_frames:Vec<Model<V>>,
    weights:Vec<f32>,
}

impl<V:Vertex> Morphable<V> {
    pub fn new(base_model:Model<V>,key_frames: Vec<Model<V>>) -> Self {
        let weights = vec![0.0;key_frames.len()];
        Self { base_model,key_frames,weights }
    }
    pub fn frame_count(&self) -> usize{
        self.key_frames.len()
    }
    pub fn get_base_model(&self) -> &Model<V>{
        &self.base_model
    }
    pub fn get_model(&self,frame:usize) -> &Model<V>{
        &self.key_frames[frame]
    }
    pub fn get_weight(&self,frame:usize) -> f32{
        self.weights[frame]
    }
    pub fn set_weight(&mut self,frame:usize,weight:f32){
        self.weights[frame] = weight.clamp(0.0, 1.0)
    }
}
impl<V:Vertex + 'static> Component for Morphable<V> {
    type Storage = HashMapStorage<Self>;
}
pub struct MorphingSystem{
}
impl<'a> System<'a> for MorphingSystem {
    type SystemData = (WriteStorage<'a,MeshRenderer>,ReadStorage<'a,Morphable<ModelVertex>>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut renderer,morph) = data;
        let interpolation = |a:Vec3,b:Vec3,c:f32|{
            return a.lerp(b, c);
        };
        for (renderer,morph) in (&mut renderer,&morph).join() {
            let mut new_model = morph.base_model.clone();
            for (vert_i,vert) in new_model.verticies.iter_mut().enumerate() {
                let mut new_pos = Vec3::from_array(vert.position);
                let mut new_normal = Vec3::from_array(vert.normal);
                for (i,model) in morph.key_frames.iter().enumerate() {
                    if let Some(frame_vert) = model.verticies.get(vert_i){
                        let frame_pos = Vec3::from_array(frame_vert.position);
                        let frame_normal = Vec3::from_array(frame_vert.normal);
                        new_pos = interpolation(new_pos, frame_pos, morph.get_weight(i));
                        new_normal = interpolation(new_normal, frame_normal, morph.get_weight(i));
                    }
                }
                vert.position = new_pos.to_array();
                vert.normal = new_normal.to_array();
            }
            renderer.model.upload_model(new_model);
        }
    }
}