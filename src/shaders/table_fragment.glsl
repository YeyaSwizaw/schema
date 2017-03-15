#version 330 core

in vec2 world_coords;

// Box Parameters
uniform ivec2 position;
uniform uvec2 size;

out vec4 colour;

void main() {
    float border = 15;

    vec2 x0y0 = position - vec2(size) / 2;
    vec2 x1y1 = x0y0 + size;

    colour = vec4(0.5, 0.5, 0.5, 1);

    float xd = min(abs(world_coords.x - x0y0.x), abs(world_coords.x - x1y1.x));
    float yd = min(abs(world_coords.y - x0y0.y), abs(world_coords.y - x1y1.y));

    if(xd < border && yd < border) {
        xd = border - xd;
        yd = border - yd;
        float d = sqrt(xd * xd + yd * yd);
        if(d < border) {
            colour.rgb = vec3((border - d) / (border * 2));
        } else {
            colour.a = 0;
        }
    } else if(xd < border) {
        colour.rgb = vec3(xd / (border * 2));
    } else if(yd < border) {
        colour.rgb = vec3(yd / (border * 2));
    }
}
