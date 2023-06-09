#[description]
Shader for reflective surfaces e.g. water

#[vertex]
#version 450 core

layout (location = 0) in vec2 a_Position;
layout (location = 1) in vec2 a_TexCoord;
layout (location = 2) in int a_TexIdx;
layout (location = 3) in float a_ReflectionY;

uniform mat4 u_ViewProjection;

out vec2 v_TexCoord;
out vec2 v_Reflection;
flat out int v_TexIdx;

void main() {
    v_TexCoord = a_TexCoord;
    v_TexIdx = a_TexIdx;
    v_Reflection = vec2(a_Position.x, (u_ViewProjection * vec4(a_Position.x, a_ReflectionY, 0.0, 1.0)).y * 0.5 + 0.5);
    gl_Position = u_ViewProjection * vec4(a_Position, 0.0, 1.0);
}

#[fragment]
#version 450 core

#define ALPHA_BLEND 0.7
#define PI 3.1415926538

in vec2 v_TexCoord;
in vec2 v_Reflection;
flat in int v_TexIdx;

uniform sampler2D u_Textures[16];
uniform float u_AmbientLightStrength;

layout (location = 0) out vec4 r_Color;
layout (location = 1) out vec4 r_WaterDepthMap;

void main() {
    r_Color = vec4(texture(u_Textures[v_TexIdx], v_TexCoord).xyz * u_AmbientLightStrength, ALPHA_BLEND);
    r_WaterDepthMap = vec4(1.0, v_Reflection, 1.0);
}
