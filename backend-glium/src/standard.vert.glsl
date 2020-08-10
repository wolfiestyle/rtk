#version 150
uniform vec2 vp_size;

in vec2 pos;
in vec4 color_mul;
in vec4 color_add;
in vec2 texc;

out vec4 v_color_mul;
out vec4 v_color_add;
out vec2 v_texc;

void main() {
    vec2 scaled = vec2(2.0, -2.0) * pos / vp_size + vec2(-1.0, 1.0);
    gl_Position = vec4(scaled, 0.0, 1.0);
    v_color_mul = color_mul;
    v_color_add = color_add;
    v_texc = texc;
}
