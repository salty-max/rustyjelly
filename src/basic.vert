
#version 330 core

layout (location = 0) in vec3 a_position;
// layout (location = 1) in vec3 Color;

// out VS_OUTPUT {
//     vec3 Color;
// } OUT;

void main()
{
    gl_Position = vec4(a_position, 1.0);
    // OUT.Color = Color;
}