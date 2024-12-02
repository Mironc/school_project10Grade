#version 330
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 uv;
out vec3 v_position;
out vec3 v_normal;
out vec2 v_uv;
uniform mat4 view;
uniform mat4 transformation;
uniform mat4 projection;
void main()
{
	v_position = vec3(transformation * vec4(position, 1.0));
	v_normal = normalize(mat3(transpose(inverse(transformation))) * normal); 
	v_uv = uv;
	gl_Position = projection * view * transformation * vec4(position,1.0f);
}
