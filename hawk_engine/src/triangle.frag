#version 330 core

in VS_OUTPUT {
    vec3 Color;
    vec3 Normal;
    vec2 TexCord;
} IN;

out vec4 Color;

uniform sampler2D texture_a;
uniform sampler2D texture_b;

void main() {
    Color = mix(texture(texture_a, IN.TexCord),  texture(texture_b, IN.TexCord), 0.5);
}