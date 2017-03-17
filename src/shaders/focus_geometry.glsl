#version 330 core

layout(points) in;
layout(triangle_strip, max_vertices = 6) out;

uniform ivec2 position;
uniform ivec2 size;
uniform int z;

layout(std140) uniform display_block {
    uvec2 size;
    ivec2 offset;
    float scale;
} display;

void main() {
    vec2 mid = position + size / 2;

    vec2 size = vec2(size.x + (size.x < 0 ? -10 : 10), size.y + (size.y < 0 ? -10 : 10));
    vec2 position = vec2(position.x - (size.x < 0 ? -5 : 5), position.y - (size.y < 0 ? -5 : 5));

    vec2 vertex = gl_in[0].gl_Position.xy;
    vec2 world_coords = position + vertex * size;

    mat4 proj = mat4(
        2 * display.scale / float(display.size.x), 0, 0, 0,
        0, -2 * display.scale / float(display.size.y), 0, 0,
        0, 0, -0.01, 0,
        -1, 1, 0, 1
    );

    vec2 shape[6];
    shape[0] = vec2(-10, 20);
    shape[1] = vec2(-5, 20);
    shape[2] = vec2(-10, -10);
    shape[3] = vec2(-5, -5);
    shape[4] = vec2(20, -10);
    shape[5] = vec2(20, -5);

    mat2 rot;
    if(world_coords.x < mid.x && world_coords.y < mid.y) {
        rot = mat2(1);
    } else if(world_coords.x < mid.x) {
        rot = mat2(0, -1, 1, 0);
    } else if(world_coords.y < mid.y) {
        rot = mat2(0, 1, -1, 0);
    } else {
        rot = mat2(-1);
    }

    for(int i = 0; i < 6; i++) {
        gl_Position = proj * vec4(world_coords - display.offset + rot * shape[i], z, 1);
        EmitVertex();
    }

    EndPrimitive();
}
