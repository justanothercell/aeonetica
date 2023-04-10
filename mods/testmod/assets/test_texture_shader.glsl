#[description]
This is a short test shader for textures

#[vertex]
#version 400 core

layout (location = 0) in vec3 a_Position;
layout (location = 1) in vec2 a_TexCoord;
layout (location = 2) in int a_TexIdx;

uniform mat4 u_ViewProjection;

out vec2 v_TexCoord;
flat out int v_TexIdx;

void main() {
    v_TexCoord = a_TexCoord;
    v_TexIdx = a_TexIdx;
    gl_Position = u_ViewProjection * vec4(a_Position, 1.0);
}

#[fragment]
#version 400 core

in vec2 v_TexCoord;
flat in int v_TexIdx;

uniform sampler2D u_Textures[16];

out vec4 color;

void main() {
    color = texture(u_Textures[v_TexIdx], v_TexCoord);
}
