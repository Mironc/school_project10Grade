#version 430
out vec4 out_color;
uniform vec3 color;
uniform vec3 non_normal_color;
void main(){

    if(!gl_FrontFacing)
        out_color = vec4(color,1.0);
    else
        out_color = vec4(non_normal_color,1.0);
}