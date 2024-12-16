#version 430
layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 tex_coord;

out vec2 texture_coordinates;
void main(){
    texture_coordinates = tex_coord;
    gl_Position = vec4(pos,0.0,1.0);
}