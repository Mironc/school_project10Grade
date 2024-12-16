#version 330
in vec3 v_normal;
in vec2 v_uv;

uniform vec3 color;
uniform sampler2D main_texture;
uniform float specular;
uniform float shininess;

//layout(location = 0) out vec3 position;
layout(location = 0) out vec4 normal;
layout(location = 1) out vec4 ColorSpec;

void main(){
    //position = v_position;
    normal = vec4(normalize(v_normal),shininess/256.0-0.5);
    //normal = vec4(encode_normal(v_normal),shininess/1024.0,0.0);
    ColorSpec = vec4(color*texture(main_texture,v_uv).rgb,specular);
}