use std::collections::HashMap;

use specs::world::Entity;

use crate::light::{area_light::AreaLight, directional_light::DirectionalLight, shadow_atlas::ShadowMapStorage, shadowmap::ShadowMap, spotlight::SpotLight, LightProperties};
pub struct LightStorage{
    area_lights:HashMap<Entity,(AreaLight,LightProperties,Option<ShadowMap>)>,
    spot_lights:HashMap<Entity,(SpotLight,LightProperties,Option<ShadowMap>)>,
    shadow_map_atlas:ShadowMapStorage,
    directional_light:(DirectionalLight,LightProperties,Option<ShadowMap>)
}
impl LightStorage {
    
}