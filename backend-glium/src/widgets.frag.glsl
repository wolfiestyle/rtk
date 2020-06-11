#version 100
precision mediump float;

uniform sampler2D tex;

varying vec4 v_color;
varying vec2 v_texc;

void main() {
    gl_FragColor = v_color * texture2D(tex, v_texc);
}
