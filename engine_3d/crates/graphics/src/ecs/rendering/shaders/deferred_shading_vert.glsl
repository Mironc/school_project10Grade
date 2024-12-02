#version 430 core
layout(location = 0) in vec3 position;
uniform vec3 camera_position;
out vec4 texture_coordinates;
//out flat int instance;
uniform int instance;

struct LightProp{
    mat4 model;
    vec4 light_color;
    vec4 light_position;
    float light_power;
};
layout(std430, binding = 1) readonly buffer lights
{
    LightProp light_props[];
};
uniform mat4 vp;
void main(){
    //instance = gl_InstanceID; 
    vec4 pos = vp * light_props[instance].model * vec4(position,1.0);
    gl_Position = pos;
    texture_coordinates = pos;
}