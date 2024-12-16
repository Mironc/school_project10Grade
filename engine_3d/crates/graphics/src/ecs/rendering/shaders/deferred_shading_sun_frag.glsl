#version 430 core
in vec2 texture_coordinates;
//in flat int instance;
uniform int instance;
uniform sampler2D position;
uniform sampler2D normal;
uniform sampler2D color_spec;

uniform vec3 camera_position;
uniform mat4 inv_vp;

uniform vec3 light_color;
uniform vec3 light_direction;
out vec4 frag_color;

float calc_diffuse(in vec3 light_dir,in vec3 normal){
    return max(dot(normal,light_dir),0.0);
}
float calc_specular(in vec3 view_dir,in vec3 light_dir,in vec3 normal,in float specular,in float shininess){
    vec3 reflect_dir = reflect(-light_dir,normal);
    vec3 half_dir = normalize(light_dir + view_dir);
    return specular * pow(clamp(dot(normal,half_dir),0.0,1.0),shininess);
}
vec3 depth_to_world_pos(float depth,vec2 uv){
    float z = depth * 2.0 - 1.0;

    vec4 clipSpacePosition = vec4(uv * 2.0 -1.0, z, 1.0);
    vec4 worldSpacePosition = inv_vp * clipSpacePosition;
    worldSpacePosition /= worldSpacePosition.w;

    return worldSpacePosition.xyz;
}
void main(){
    vec2 uv = texture_coordinates;//(texture_coordinates.xy)/texture_coordinates.w*0.5 + 0.5;

    vec3 frag_position = depth_to_world_pos(texture(position,uv).r,uv);

    //read from textures
    vec4 normal_shininess = texture(normal,uv).rgba;
    vec4 color_specular = texture(color_spec,uv).rgba;

    vec3 frag_normal = normalize(normal_shininess.rgb);
    
    vec3 color = color_specular.rgb;
    float shininess = normal_shininess.a+0.5*256.0;
    float specular = color_specular.a;
    vec3 diffuse_result = vec3(calc_diffuse(-light_direction,frag_normal)) ;

    vec3 view_direction = normalize(camera_position - frag_position);
    vec3 specular_result = vec3(calc_specular(view_direction,-light_direction,frag_normal,specular,shininess));

    frag_color = vec4(( diffuse_result + specular_result ) * light_color * color,1.0);
} 