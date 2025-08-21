#version 430
out vec2 texture_coordinates;
vec3 temp;
void main(){
    temp.x = int(gl_VertexID << (1 & 0x1F)) & 2u;
    temp.z = uint(gl_VertexID) & 2u;

    gl_Position.xy = temp.xz * vec2(2.0,2.0) + vec2(-1.0,-1.0);
    texture_coordinates = temp.xz;
    gl_Position.zw = vec2(-1.0,1.0);
}