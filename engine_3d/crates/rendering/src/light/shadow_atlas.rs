use graphics::objects::texture::{texture::Texture, texture_type::CubeMapTexture, Texture2D};


pub struct ShadowMapStorage{
    single_maps:Vec<Texture2D>,
    area_maps:Vec<Texture<CubeMapTexture>>,
}
impl ShadowMapStorage {
    
}