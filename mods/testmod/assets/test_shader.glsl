#[description]
This is a short test shader for the client renderer

#[vertex]
#version 330 core

layout (location = 0) in vec3 a_Position;
layout (location = 1) in vec4 a_Color;
layout (location = 2) in vec2 a_TexCoord;

uniform mat4 u_ViewProjection;

out vec4 v_Color;

void main() {
    v_Color = a_Color;
    gl_Position = u_ViewProjection * vec4(a_Position, 1.0);
}

#[fragment]
#version 330 core

in vec4 v_Color;

out vec4 color;

void main() {
    color = v_Color;
}
