#version 150
uniform sampler2D tex;

in vec4 v_color_mul;
in vec4 v_color_add;
in vec2 v_texc;

out vec4 f_color;

void main() {
    f_color = texture(tex, v_texc) * v_color_mul + v_color_add;
}
