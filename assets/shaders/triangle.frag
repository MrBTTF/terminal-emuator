#version 330 core

in VS_OUTPUT {
    vec3 Color;
    vec2 TexCoord;
} IN;

out vec4 Color;

uniform sampler2D texture1;
uniform sampler2D texture2;

void main()
{
    // Color = mix(texture(texture1, IN.TexCoord), texture(texture2, IN.TexCoord), 0.2);
    Color =texture(texture1, IN.TexCoord);
}