#version 330 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 norm;
layout (location = 2) in vec2 texture_coord;
out vec3 normal;
out vec3 position;
out vec2 texture_coordinates;
uniform mat4 view;
uniform mat4 transformation;
uniform mat4 projection;
void main()
{
	position = vec3(transformation * vec4(pos, 1.0));
	gl_Position = projection * view * transformation * vec4(pos,1.0f);
	normal = normalize(mat3(transpose(inverse(transformation))) * norm); 
	texture_coordinates = texture_coord;
}
