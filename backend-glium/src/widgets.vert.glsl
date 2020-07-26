#version 100
precision highp float;

uniform vec2 vp_size;

attribute vec2 pos;
attribute vec4 color;
attribute vec2 texc;

varying vec4 v_color;
varying vec2 v_texc;

void main() {
    vec2 scaled = vec2(2.0, -2.0) * pos / vp_size + vec2(-1.0, 1.0);
    gl_Position = vec4(scaled, 0.0, 1.0);
    v_color = color;
    v_texc = texc;
}
