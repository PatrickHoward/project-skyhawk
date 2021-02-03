#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;
layout (location = 2) in vec2 TexCord;
layout (location = 3) in vec3 Normal;

out VS_OUTPUT {
    vec3 Color;
    vec3 Normal;
    vec2 TexCord;
} OUT;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * model * vec4(Position, 1.0);
    OUT.Color = Color;
    OUT.TexCord = TexCord;
    OUT.Normal = Normal;
}
