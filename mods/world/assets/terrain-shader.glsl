#[description]
Shader for normal world terrain

#[vertex]
#version 450 core

const uint MAX_LIGHT_SOURCE_COUNT = 20;

layout (location = 0) in vec2 a_Position;
layout (location = 1) in vec2 a_TexCoord;
layout (location = 2) in int a_TexIdx;

uniform mat4 u_ViewProjection;
uniform vec2 u_LightPositions[MAX_LIGHT_SOURCE_COUNT];
uniform uint u_NumLights = MAX_LIGHT_SOURCE_COUNT;

out vec2 v_TexCoord;
out float v_LightDistances[MAX_LIGHT_SOURCE_COUNT];
flat out int v_TexIdx;

void main() {
    v_TexCoord = a_TexCoord;
    v_TexIdx = a_TexIdx;
    gl_Position = u_ViewProjection * vec4(a_Position, 0.0, 1.0);

    for(uint i = 0; i < u_NumLights; i++) {
        v_LightDistances[i] = length(vec3(u_LightPositions[i], 1.0) - vec3(a_Position, 0.0));
    }
}

#[fragment]
#version 450 core

const uint MAX_LIGHT_SOURCE_COUNT = 20;

in vec2 v_TexCoord;
in float v_LightDistances[MAX_LIGHT_SOURCE_COUNT];
flat in int v_TexIdx;

uniform sampler2D u_Textures[16];

uniform uint u_NumLights = MAX_LIGHT_SOURCE_COUNT;
uniform vec3 u_LightColors[MAX_LIGHT_SOURCE_COUNT];
uniform float u_LightIntensities[MAX_LIGHT_SOURCE_COUNT];
uniform float u_AmbientLightStrength;

layout (location = 0) out vec4 r_Color;
layout (location = 1) out vec4 r_LightMap;

void main() {
    vec3 total_diffuse = vec3(u_AmbientLightStrength);

    for(uint i = 0; i < u_NumLights; i++) {
        float intensity = u_LightIntensities[i];
        total_diffuse += u_LightColors[i] * (intensity - min(v_LightDistances[i], intensity)) / intensity;
    }

    r_Color = texture(u_Textures[v_TexIdx], v_TexCoord) * vec4(total_diffuse, 1.0);
    r_LightMap = vec4(0.0, 0.0, 0.0, 1.0);
}
