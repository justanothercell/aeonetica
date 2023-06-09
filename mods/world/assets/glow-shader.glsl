#[description]
shader for the GlowTexture material

#[vertex]
#version 450 core

layout (location = 0) in vec2 a_Position;
layout (location = 1) in vec2 a_TexCoord;
layout (location = 2) in int  a_TexIdx;
layout (location = 3) in vec4 a_GlowColor;

uniform mat4 u_ViewProjection;

out vec2 v_TexCoord;
out vec4 v_GlowColor;
flat out int v_TexIdx;

void main() {
    v_TexCoord = a_TexCoord;
    v_TexIdx = a_TexIdx;
    v_GlowColor = a_GlowColor;
    gl_Position = u_ViewProjection * vec4(a_Position, 0.0, 1.0);
}

#[fragment]
#version 450 core

in vec2 v_TexCoord;
in vec4 v_GlowColor;
flat in int v_TexIdx;

uniform sampler2D u_Textures[16];

layout (location = 0) out vec4 r_FragColor;
layout (location = 1) out vec4 r_WaterDepthMap;

void main() {
    r_FragColor = texture(u_Textures[v_TexIdx], v_TexCoord) * vec4(1.3, 1.3, 1.3, 1.0);
    r_WaterDepthMap = vec4(0.0, 0.0, 0.0, r_FragColor.a);
}
