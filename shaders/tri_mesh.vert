#version 450

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec3 color;

layout (location = 0) out vec3 out_color;

void main() {
    gl_Position = vec4(pos, 1.0f);
    out_color = color;
}
