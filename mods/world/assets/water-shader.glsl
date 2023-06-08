#[description]
Shader for reflective surfaces e.g. water

#[vertex]
#version 450 core

layout (location = 0) in vec2 a_Position;
layout (location = 1) in vec2 a_TexCoord;
layout (location = 2) in int a_TexIdx;

uniform mat4 u_ViewProjection;

out vec2 v_TexCoord;
flat out int v_TexIdx;
flat out float v_WaveOffset;

void main() {
    v_TexCoord = a_TexCoord;
    v_TexIdx = a_TexIdx;
    v_WaveOffset = (a_TexCoord.x * 2.0 - 1.0) * 0.1;
    gl_Position = u_ViewProjection * vec4(a_Position, 0.0, 1.0);
}

#[fragment]
#version 450 core

#define ALPHA_BLEND 0.5
#define PI 3.1415926538

in vec2 v_TexCoord;
flat in int v_TexIdx;
flat in float v_WaveOffset;

uniform sampler2D u_Textures[16];
uniform float u_AmbientLightStrength = 0.1;
uniform float u_Time;

layout (location = 0) out vec4 r_Color;
layout (location = 1) out vec4 r_LightMap;

void main() {
    vec4 tex_color = texture(u_Textures[v_TexIdx], v_TexCoord + vec2(0.0, v_WaveOffset * sin(u_Time + v_TexCoord.x * PI)));
    float alpha = min(tex_color.a, ALPHA_BLEND);
    r_Color = vec4(tex_color.xyz * u_AmbientLightStrength, alpha);
    r_LightMap = vec4(0.0, 0.0, 0.0, alpha);
}

