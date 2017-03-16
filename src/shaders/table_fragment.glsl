#version 330 core

in vec2 world_coords;

// Box Parameters
uniform ivec2 position;
uniform ivec2 size;
uniform vec4 inner_colour;
uniform vec4 outer_colour;

out vec4 colour;

void main() {
    float border = 15;

    vec2 x0y0 = position - vec2(size) / 2;
    vec2 x1y1 = x0y0 + size;

    vec4 inner = inner_colour;
    vec4 outer = outer_colour;

    vec4 s = (outer - inner) / border;

    float xd = min(abs(world_coords.x - x0y0.x), abs(world_coords.x - x1y1.x));
    float yd = min(abs(world_coords.y - x0y0.y), abs(world_coords.y - x1y1.y));

    if(xd < border && yd < border) {
        xd = border - xd;
        yd = border - yd;
        float d = sqrt(xd * xd + yd * yd);
        if(d < border) {
            colour = inner + s * d;
        } else {
            colour.a = 0;
        }
    } else if(xd < border) {
        colour = inner + s * (border - xd);
    } else if(yd < border) {
        colour = inner + s * (border - yd);
    } else {
        colour = inner;
    }
}
