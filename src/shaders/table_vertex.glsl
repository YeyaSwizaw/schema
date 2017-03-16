#version 330 core

in vec2 vertex;

// Box Parameters
uniform ivec2 position;
uniform ivec2 size;

// Display Parameters
uniform display {
    uvec2 size;
    ivec2 offset;
    float scale;
} display;

out vec2 world_coords;

void main() {
    world_coords = position + vertex * size - vec2(size) / 2;

    mat4 proj = mat4(
        2 * display.scale / float(display.size.x), 0, 0, 0,
        0, -2 * display.scale / float(display.size.y), 0, 0,
        0, 0, 1, 0,
        -1, 1, 1, 1
    );
    gl_Position = proj * vec4(world_coords - display.offset, 0, 1);
}
