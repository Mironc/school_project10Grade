#version 330
in vec3 v_position;
in vec3 v_normal;
in vec2 v_uv;

uniform vec3 color;
uniform sampler2D main_texture;
uniform vec3 camera_position;
uniform float ambient;
uniform float specular;
uniform float shininess;

//layout(location = 0) out vec3 position;
layout(location = 0) out vec3 normal;
layout(location = 1) out vec4 ColorSpec;

vec2 encode_normal(vec3 normal){
    /* 
    float f = sqrt(8.0*normal.z+8.0);
    return normal.xy / f + 0.5; */
    
    const float kPI = 3.1415926536f;
    return vec4(
      (vec2(atan(normal.y,normal.x)/kPI, normal.z)+1.0)*0.5,
      0,0).xy;
}
void main(){
    //position = v_position;
    //normal = vec4(normalize(v_normal)*0.5+0.5,shininess/256.0);
    normal = vec3(encode_normal(v_normal),shininess/1024.0);
    ColorSpec = vec4(color*texture(main_texture,v_uv).rgb,specular);
}