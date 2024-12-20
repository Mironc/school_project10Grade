#version 430 core
layout(early_fragment_tests) in;
in vec4 texture_coordinates;
//in flat int instance;
uniform int instance;
uniform sampler2D position;
uniform sampler2D normal;
uniform sampler2D color_spec;

uniform vec3 camera_position;
uniform mat4 inv_vp;
uniform mat4 inv_proj;
uniform mat4 inv_view;
uniform float clip_far;
uniform float clip_near;

struct LightProp {
    mat4 model;
    vec4 light_color;
    vec4 light_position;
    float light_power;
};
layout(std430, binding = 1) readonly buffer lights {
LightProp light_props[];
};


out vec4 frag_color;

float calc_diffuse(in vec3 light_dir, in vec3 normal) {
    return max(dot(normal, light_dir), 0.0);
}
float calc_specular(in vec3 view_dir, in vec3 light_dir, in vec3 normal, in float specular, in float shininess) {
    vec3 reflect_dir = reflect(- light_dir, normal);
    vec3 half_dir = normalize(light_dir + view_dir);
    return specular * pow(clamp(dot(normal, half_dir), 0.0, 1.0), shininess);
}
vec3 decode_normal(vec2 enc) {
    const float kPI = 3.1415926536;
    vec2 ang = enc * 2 - 1;
    vec2 scth = vec2(sin(ang.x * kPI), cos(ang.x * kPI));
    vec2 scphi = vec2(sqrt(1.0 - ang.y * ang.y), ang.y);
    return vec3(scth.y * scphi.x, scth.x * scphi.x, scphi.y);

        /* vec2 fenc = enc*4.0-2.0;
        float f = dot(fenc,fenc);
        float g = sqrt(1.0-f/4.0);
        vec3 n;
        n.xy = fenc*g;
        n.z = 1.0-f/2.0;
        return n; */
}
vec3 depth_to_world_pos(float depth, vec2 uv) {
    float z = depth * 2.0 - 1.0;

    vec4 clipSpacePosition = vec4(uv * 2.0 - 1.0, z, 1.0);
    vec4 worldSpacePosition = inv_vp * clipSpacePosition;
        //vec4 worldSpacePosition = inv_view * inv_proj * clipSpacePosition;

        // Perspective division
    worldSpacePosition /= worldSpacePosition.w;

    return worldSpacePosition.xyz;
}
void main() {
    vec2 uv = (texture_coordinates.xy) / texture_coordinates.w * 0.5 + 0.5;
        //float proj_a = clip_far / (clip_far - clip_near);
        //float proj_b = (-clip_far * clip_near) / (clip_far - clip_near);
        //float depth = texture(position,uv).r * 2.0 - 1.0;

    vec3 frag_position = depth_to_world_pos(texture(position, uv).r, uv);//texture(position,uv).rgb;
        //vec3 frag_normal = texture(normal,uv).rgb;

        //read from textures
    vec4 normal_shininess = texture(normal, uv).rgba;
    vec4 color_specular = texture(color_spec, uv).rgba;

    vec3 light_direction = normalize(light_props[instance].light_position.xyz - frag_position);

    float D = length(light_props[instance].light_position.xyz - frag_position);
    float attenuation = (1.0 / (D * D)) * light_props[instance].light_power;

        //unpack
        //vec3 frag_normal = normalize(decode_normal(normal_shininess.rg));
    vec3 frag_normal = normalize(normal_shininess.rgb);

    vec3 color = color_specular.rgb;
    float shininess = normal_shininess.a + 0.5 * 256.0;
    float specular = color_specular.a;

    vec3 diffuse_result = vec3(calc_diffuse(light_direction, frag_normal));

    vec3 view_direction = normalize(camera_position - frag_position);
    vec3 specular_result = vec3(calc_specular(view_direction, light_direction, frag_normal, specular, shininess));

    frag_color = vec4((diffuse_result + specular_result) * (light_props[instance].light_color.rgb * attenuation) * color, 1.0);
}