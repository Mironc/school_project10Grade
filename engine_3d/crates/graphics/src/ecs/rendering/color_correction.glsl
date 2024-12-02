#version 430
out vec4 frag_color;
uniform sampler2D color;
uniform float gamma;
uniform float brightness;
uniform float contrast;
in vec2 texture_coordinates;

void main() {
    vec3 hdrColor = texture(color, texture_coordinates).rgb;

    // reinhard tone mapping
    vec3 mapped = hdrColor / (hdrColor + vec3(1.0));
    vec3 color_corrected = min(vec3(1.0), contrast * (mapped - 0.5) + 0.5 +vec3(brightness));
    // gamma correction 
    vec3 out_color = pow(color_corrected, vec3(1.0 / gamma));

    frag_color = vec4(out_color, 1.0);
}