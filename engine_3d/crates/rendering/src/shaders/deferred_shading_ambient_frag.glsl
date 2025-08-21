#version 430 core
in vec2 texture_coordinates;
uniform sampler2D color_spec;
uniform vec3 ambient;
out vec4 frag_color;

void main(){
    vec2 uv = texture_coordinates;
    vec3 color = texture(color_spec,uv).rgb;
    frag_color = vec4(color*ambient,1.0);
} 