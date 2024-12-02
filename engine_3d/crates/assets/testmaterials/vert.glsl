#version 330 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec3 texture_coord;
out vec3 normal;
out vec3 position;
uniform mat4 view;
uniform mat4 transformation;
uniform mat4 projection;
void main()
{
	position = vec3(transformation * vec4(pos, 1.0)) ;
	gl_Position = projection * view * transformation * vec4(pos,1.0f);
	normal =  mat3(transpose(inverse(transformation))) * normal; 
}
