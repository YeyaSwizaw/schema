#version 330 core

in vec2 world_coords;

// Box Parameters
uniform vec2 position;
uniform uvec2 size;

out vec4 colour;

void main() {
    vec2 x0y0 = position - float(size) / 2;
    vec2 x1y1 = x0y0 + float(size);

    colour = vec4(0.5, 0.5, 0.5, 1);

    float xd = min(abs(world_coords.x - x0y0.x), abs(world_coords.x - x1y1.x));
    float yd = min(abs(world_coords.y - x0y0.y), abs(world_coords.y - x1y1.y));

    if(xd < 10 && yd < 10) {
        colour.xyz = vec3(min(xd, yd) / 20);
    } else if(xd < 10) {
        colour.xyz = vec3(xd / 20);
    } else if(yd < 10) {
        colour.xyz = vec3(yd / 20);
    }
}
