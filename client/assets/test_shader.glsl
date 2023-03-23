#[description]
This is a short test shader for the client renderer

#[vertex]
#version 330 core

layout (location = 0) in vec3 a_Position;
layout (location = 1) in vec2 a_TexCoord;

uniform mat4 u_ViewProjection;
uniform mat4 u_Transform;

out vec2 v_TexCoord;

void main() {
    v_TexCoord = a_TexCoord;
    gl_Position = u_ViewProjection * u_Transform * vec4(a_Position, 1.0);
}

#[fragment]
#version 330 core

uniform sampler2D u_Texture;
uniform float u_TilingFactor;
uniform vec4 u_Color;

in vec2 v_TexCoord;

out vec4 color;

void main() {
    color = texture(u_Texture, v_TexCoord * u_TilingFactor) * u_Color;
}
