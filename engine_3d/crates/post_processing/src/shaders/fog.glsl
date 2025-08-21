#version 430 core
#define PI 3.1415926538

in vec2 texture_coordinates;
uniform sampler2D color;
uniform sampler2D depth;
uniform float strength;
uniform vec3 fog_color;
uniform float offset;
uniform float far;
uniform float near;
out vec4 out_color;

void main() {
    vec2 uv = texture_coordinates;
    vec3 color = texture(color, uv).rgb;
    ///convert depth value to range [0.0,far]
    float d = (2.0 * near * far) / (far + near - (texture(depth,uv).r *2.0-1.0) * (far - near));
    ///i guess this is easy to understand
    float fogFactor = (strength / sqrt(log(2.0))) * max(0.0, d - offset);
    ///exponentiate
    fogFactor = exp2(-fogFactor * fogFactor);
    //out_color = vec4(vec3(d), 1.0);
    out_color = vec4(mix(fog_color,color,vec3(min(1.0,fogFactor))),1.0);
}
