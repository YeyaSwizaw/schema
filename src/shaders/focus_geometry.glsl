#version 330 core

layout(points) in;
layout(triangle_strip, max_vertices = 4) out;

uniform ivec2 position;
uniform ivec2 size;
uniform int z;

layout(std140) uniform display_block {
    uvec2 size;
    ivec2 offset;
    float scale;
} display;

void main() {
    vec2 size = vec2(size.x + (size.x < 0 ? -20 : 20), size.y + (size.y < 0 ? -20 : 20));
    vec2 position = vec2(position.x - (size.x < 0 ? -10 : 10), position.y - (size.y < 0 ? -10 : 10));

    vec2 vertex = gl_in[0].gl_Position.xy;
    vec2 world_coords = position + vertex * size;

    mat4 proj = mat4(
        2 * display.scale / float(display.size.x), 0, 0, 0,
        0, -2 * display.scale / float(display.size.y), 0, 0,
        0, 0, -0.01, 0,
        -1, 1, 0, 1
    );

    gl_Position = proj * vec4(world_coords - display.offset + vec2(-10, -10), z, 1);
    EmitVertex();

    gl_Position = proj * vec4(world_coords - display.offset + vec2(-10, 10), z, 1);
    EmitVertex();

    gl_Position = proj * vec4(world_coords - display.offset + vec2(10, -10), z, 1);
    EmitVertex();

    gl_Position = proj * vec4(world_coords - display.offset + vec2(10, 10), z, 1);
    EmitVertex();

    EndPrimitive();
}
