#[description]
default shader for the FlatColor material

#[vertex]
#version 450 core

layout (location = 0) in vec2 a_Position;
layout (location = 1) in vec4 a_Color;

uniform mat4 u_ViewProjection;

out vec4 v_Color;

void main() {
    v_Color = a_Color;
    gl_Position = u_ViewProjection * vec4(a_Position, 0.0, 1.0);
}

#[fragment]
#version 450 core

in vec4 v_Color;

layout (location = 0) out vec4 r_Color;

void main() {
    r_Color = v_Color;
}
