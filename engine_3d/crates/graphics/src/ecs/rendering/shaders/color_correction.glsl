#version 430
out vec4 frag_color;
uniform sampler2D color;
uniform float gamma;
uniform float brightness;
uniform float saturation;
uniform float exposure;
uniform float contrast;
uniform float midpoint;
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
    //color correction
    vec3 color = max(vec3(0.0), mapped * exposure);
    color = max(vec3(0.0), contrast * (color - midpoint) + midpoint + vec3(brightness));
    color = max(vec3(0.0), mix(vec3(luminance(color)), color, saturation));
    // gamma correction 
    vec3 out_color = pow(color, vec3(gamma));
    frag_color = vec4(out_color, 1.0);
}