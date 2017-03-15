#version 330 core

in vec2 vertex;

// Box Parameters
uniform vec2 position;
uniform uvec2 size;

// Display Parameters
uniform vec2 off;
uniform uvec2 display;
uniform float scale;

out vec2 world_coords;

void main() {
    world_coords = position + vertex * float(size) - float(size) / 2;

    mat4 proj = mat4(
        2 * scale / float(display.x), 0, 0, 0,
        0, -2 * scale / float(display.y), 0, 0,
        0, 0, 1, 0,
        -1, 1, 1, 1
    );

    gl_Position = proj * vec4(world_coords - off, 0, 1);
}
