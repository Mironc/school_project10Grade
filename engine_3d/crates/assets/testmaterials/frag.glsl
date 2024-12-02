#version 330 core

uniform vec3 object_color;
uniform vec3 camera_position;
uniform float ambient;
uniform float specular;
uniform vec3 light_position;
uniform vec3 light_color;
in vec3 normal;
in vec3 position;
out vec4 frag_color;
void main()
{
    vec3 ambient_result = ambient * light_color;
    
    vec3 light_direction = normalize(light_position - position);
    vec3 diffuse_result = max(dot(normal,light_direction),0.0) * light_color;
    
    vec3 view_direction = normalize(camera_position - position);
    vec3 reflect_direction = reflect(-light_direction,normal);
    vec3 specular_result = specular * pow(max(dot(view_direction,reflect_direction),0.0),64) * light_color;
    
	vec3 result = (ambient_result + diffuse_result + specular_result) * object_color; 
	frag_color = vec4(result,1.0);
}
