#version 330 core
    
in vec3 f_color;

out vec4 fragColor;

void main(){
    fragColor = vec4(f_color, 1.0);
}