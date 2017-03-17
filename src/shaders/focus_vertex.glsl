#version 330 core

in vec2 vertex;

layout(std140) uniform display_block {
    uvec2 size;
    ivec2 offset;
    float scale;
} display;

void main() {
    gl_Position = vec4(vertex, 0, 1);
}

