#version 330 core

in VS_OUTPUT {
    vec4 Color;
    vec2 TexCoord;
} IN;

out vec4 Color;

void main()
{
    Color =IN.Color;
}