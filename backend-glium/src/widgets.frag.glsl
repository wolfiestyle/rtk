#version 100
precision mediump float;

uniform sampler2D tex;

varying vec4 v_color_mul;
varying vec4 v_color_add;
varying vec2 v_texc;

void main() {
    gl_FragColor = texture2D(tex, v_texc) * v_color_mul + v_color_add;
}
