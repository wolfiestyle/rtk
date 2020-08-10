#version 150
uniform vec2 vp_size;

in vec4 rect;
in vec4 texr;
in vec4 color;

out vec2 v_texc;
out vec4 v_color;

void main() {
    vec2 pos = vec2(0.0);
    switch (gl_VertexID) {
        case 0:
            pos = rect.xy;
            v_texc = texr.xy;
            break;
        case 1:
            pos = rect.zy;
            v_texc = texr.zy;
            break;
        case 2:
            pos = rect.xw;
            v_texc = texr.xw;
            break;
        case 3:
            pos = rect.zw;
            v_texc = texr.zw;
            break;
    }

    vec2 scaled = vec2(2.0, -2.0) * pos / vp_size + vec2(-1.0, 1.0);
    gl_Position = vec4(scaled, 0.0, 1.0);
    v_color = color;
}
