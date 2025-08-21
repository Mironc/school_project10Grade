#version 430
layout(location = 0) in vec2 texture_coordinates;
uniform sampler2D color;
layout(location = 0) out vec4 frag_color;

void main() {
    vec3 color = texture(color, texture_coordinates).rgb;
    frag_color = vec4(color, 1.0);
}