#version 150
layout(location = 0) in vec3 v_pos;
uniform mat4 mvp;

void main(){
    gl_Position = mvp * vec4(v_pos,1.0);
}