#version 150
uniform sampler2D tex;

in vec2 v_texc;
in vec4 v_color;

out vec4 f_color;

void main() {
    float alpha = texture(tex, v_texc).r;
    f_color = v_color * vec4(1.0, 1.0, 1.0, alpha);
}
