#version 330 core

// in VS_OUTPUT {
//     vec3 Color;
// } IN;

precision mediump float;
uniform vec4 u_color;

out vec4 Color;

void main()
{
    Color = u_color;
}