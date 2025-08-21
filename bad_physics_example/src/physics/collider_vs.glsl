#version 430
layout(location = 0) in vec3 position;
uniform mat4 transform;
uniform mat4 mv;
void main() {
    vec4 new_pos = mv * transform * vec4(position,1.0);
    gl_Position = new_pos;
}