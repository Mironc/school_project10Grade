#version 430 core

uniform vec3 color;
uniform sampler2D main_texture;
uniform vec3 camera_position;
uniform float ambient;
uniform float specular;
uniform float shininess;

struct LightProp{
    mat4 model;
    vec4 light_color;
    vec4 light_position;
    float light_power;
};

layout(std430, binding = 1) readonly buffer lights
{
    LightProp light_props[];
};
uniform int light_count;

in vec3 normal;
in vec3 position;
in vec2 texture_coordinates;
out vec4 frag_color;

float calc_diffuse(in vec3 light_dir,in vec3 normal){
    return max(dot(normal,light_dir),0.0);
}
float calc_specular(in vec3 view_dir,in vec3 light_dir,in vec3 normal,in float specular,in float shininess){
    vec3 reflect_dir = reflect(-light_dir,normal);
    vec3 half_dir = normalize(light_dir + view_dir);
    return specular * pow(clamp(dot(normal,half_dir),0.0,1.0),shininess);
}
void main() {
    vec3 norm = normalize(normal);
    vec3 object_color = texture(main_texture,texture_coordinates).rgb * color;

    vec3 ambient_result = ambient * vec3(1.0);
    vec3 result = ambient_result;     
    for(int i = 0; i < light_count;++i){

        vec3 light_direction = normalize(light_props[i].light_position.xyz-position);

        float D = length(light_props[i].light_position.xyz-position);
        float attenuation = (1.0/(D*D)) * light_props[i].light_power;

        vec3 diffuse_result = vec3(calc_diffuse(light_direction,norm)) ;
        
        vec3 view_direction = normalize(camera_position - position);
        vec3 specular_result = vec3(calc_specular(view_direction,light_direction,norm,specular,shininess));

        result += ( diffuse_result + specular_result ) * (light_props[i].light_color.rgb*attenuation)* object_color;   
    }
	frag_color = vec4(result,1.0);
}
