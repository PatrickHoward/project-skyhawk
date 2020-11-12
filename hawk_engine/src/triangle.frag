#version 330 core

in VS_OUTPUT {
    vec3 Color;
    vec2 TexCord;
} IN;

out vec4 Color;

uniform sampler2D Textr;

void main() {
    Color = texture(Textr, IN.TexCord);
}