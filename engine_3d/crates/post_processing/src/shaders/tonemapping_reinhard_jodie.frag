#version 430
out vec4 frag_color;
uniform sampler2D color;
in vec2 texture_coordinates;

float luminance(vec3 color) {
    return dot(color, vec3(0.299, 0.587, 0.114));
}
void main() {
    vec3 hdrColor = texture(color, texture_coordinates).rgb;
    // reinhard jodie tone mapping
    float l = luminance(hdrColor);
    vec3 tv = hdrColor / (1.0 + hdrColor);
    vec3 mapped = mix(hdrColor / (1.0 + l), tv, tv);
    vec3 out_color = mapped;
    frag_color = vec4(out_color, 1.0);
}