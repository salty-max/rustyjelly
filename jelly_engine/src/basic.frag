#version 410

precision mediump float;

uniform vec4 u_tint;

uniform sampler2D u_diffuse;

in vec2 v_tex_coord;

out vec4 fragColor;

void main()
{
    fragColor = u_tint * texture(u_diffuse, v_tex_coord);
}