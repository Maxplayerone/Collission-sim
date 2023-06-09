#version330 core
layout(location = 0) in vec2 a_pos;
layout(location = 1) in vec3 a_color;

uniform mat4 u_world_mat;

out vec3 f_color;

void main(){
	f_color = a_color;
	gl_Position = u_world_mat * vec4(a_pos.x, a_pos.y, 1.0, 1.0);
}