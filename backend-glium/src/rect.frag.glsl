#version 150
uniform sampler2D tex;
uniform sampler2D font_tex;

in vec2 v_texc;
in vec4 v_color_mul;
in vec4 v_color_add;
in vec4 v_font_col;

out vec4 f_color;

void main() {
    vec4 rect_c = texture(tex, v_texc) * v_color_mul + v_color_add;
    vec4 text_c = vec4(1.0, 1.0, 1.0, texture(font_tex, v_texc).r);
    f_color = text_c * v_font_col + rect_c;
}
