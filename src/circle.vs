#version 330 core
    layout (location = 0) in vec3 aPos;

    out vec3 v_pos;

    void main() {
        v_pos = aPos;
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }