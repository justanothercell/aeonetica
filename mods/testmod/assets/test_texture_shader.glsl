#[description]
This is a short test shader for textures

#[vertex]
#version 330 core

layout (location = 0) in vec3 a_Position;
layout (location = 1) in vec2 a_TexCoord;

uniform mat4 u_ViewProjection;

out vec2 v_TexCoord;

void main() {
    v_TexCoord = a_TexCoord;
    gl_Position = u_ViewProjection * vec4(a_Position, 1.0);
}

#[fragment]
#version 330 core

in vec2 v_TexCoord;

uniform sampler2D u_Textures[16];

out vec4 color;

void main() {
    color = texture(u_Textures[0], v_TexCoord);
}
