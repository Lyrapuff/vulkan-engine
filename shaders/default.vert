#version 450

layout(location = 0) out vec4 o_color;

void main() {
    if (gl_VertexIndex == 0) {
        gl_Position = vec4(-0.5, 0.0, 0.0, 1.0);

        o_color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    if (gl_VertexIndex == 1) {
        gl_Position = vec4(0.0, -1.0, 0.0, 1.0);

        o_color = vec4(0.0, 1.0, 0.0, 1.0);
    }
    if (gl_VertexIndex == 2) {
        gl_Position = vec4(0.5, 0.0, 0.0, 1.0);

        o_color = vec4(0.0, 0.0,1.0, 1.0);
    }
}
