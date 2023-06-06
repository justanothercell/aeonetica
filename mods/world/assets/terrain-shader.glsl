#[description]
Shader for normal world terrain

#[vertex]
#version 450 core

const uint MAX_LIGHT_SOURCE_COUNT = 20;

layout (location = 0) in vec2 a_Position;
layout (location = 1) in vec2 a_TexCoord;
layout (location = 2) in int a_TexIdx;

uniform mat4 u_ViewProjection;
uniform vec2 u_LightPosition[MAX_LIGHT_SOURCE_COUNT];
uniform uint u_NumLights = MAX_LIGHT_SOURCE_COUNT;

out vec2 v_TexCoord;
out vec3 v_ToLightVector[MAX_LIGHT_SOURCE_COUNT];
flat out int v_TexIdx;

void main() {
    v_TexCoord = a_TexCoord;
    v_TexIdx = a_TexIdx;
    gl_Position = u_ViewProjection * vec4(a_Position, 0.0, 1.0);

    for(int i = 0; i < u_NumLights; i++) {
        v_ToLightVector[i] = vec3(u_LightPosition[i], 1.0) - gl_Position.xyz;
    }
}

#[fragment]
#version 450 core

const uint MAX_LIGHT_SOURCE_COUNT = 20;

in vec2 v_TexCoord;
in vec3 v_ToLightVector[MAX_LIGHT_SOURCE_COUNT];
flat in int v_TexIdx;

uniform sampler2D u_Textures[16];

uniform uint u_NumLights = MAX_LIGHT_SOURCE_COUNT;
uniform vec3 u_Attenuation = vec3(1, 0.1, 0.05);
uniform vec3 u_LightColor = vec3(0.8, 0.8, 0.7);
uniform float u_AmbientLightStrength;

layout (location = 0) out vec4 r_Color;
layout (location = 1) out vec4 r_LightMap;

void main() {
    vec3 unit_normal = normalize(vec3(0.0, 0.0, 1.0));
    vec3 total_diffuse = vec3(0.0);

    for(int i = 0; i < u_NumLights; i++) {
        float dist = length(v_ToLightVector[i]);
        float att_factor = u_Attenuation.x + (u_Attenuation.y * dist) + (u_Attenuation.z * dist * dist);
        
        vec3 unit_light_vector = normalize(v_ToLightVector[i]);

        float n_dot_1 = dot(unit_normal, unit_light_vector);
        float brightness = max(n_dot_1, u_AmbientLightStrength);

        vec3 diffuse = brightness * u_LightColor;

        total_diffuse += (brightness * u_LightColor) / att_factor;
    }

    r_Color = texture(u_Textures[v_TexIdx], v_TexCoord) * vec4(total_diffuse, 1.0);
    r_LightMap = vec4(0.0);
}
