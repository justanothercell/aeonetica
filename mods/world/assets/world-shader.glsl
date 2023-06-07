#[description]
Shader used for rendering the world

#[vertex]
#version 450 core

layout (location = 0) in vec2 a_Position;
layout (location = 1) in vec2 a_FrameCoord;

out vec2 v_FrameCoord;

void main() {
    v_FrameCoord = a_FrameCoord;
    gl_Position = vec4(a_Position, 0.0, 1.0);
}

#[fragment]
#version 450 core

//#define BLOOM_STRENGTH 5

in vec2 v_FrameCoord;

uniform sampler2D u_Frame;
uniform sampler2D u_LightMap;

#ifdef BLOOM_STRENGTH
    uniform float u_BloomWeight[BLOOM_STRENGTH] = float[] (0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);
#endif

layout (location = 0) out vec4 r_Color;

void main() {
    #ifdef BLOOM_STRENGTH
        vec3 glowing = texture(u_LightMap, v_FrameCoord).rgb * u_BloomWeight[0];
        vec2 tex_offset = 1.0 / vec2(1920, 1080);

        for(int i = 1; i < BLOOM_STRENGTH; i++)
        {
            glowing += texture(u_LightMap, v_FrameCoord + vec2(tex_offset.x * i, 0.0)).rgb * u_BloomWeight[i];
            glowing += texture(u_LightMap, v_FrameCoord - vec2(tex_offset.x * i, 0.0)).rgb * u_BloomWeight[i];
            glowing += texture(u_LightMap, v_FrameCoord + vec2(0.0, tex_offset.y * i)).rgb * u_BloomWeight[i];
            glowing += texture(u_LightMap, v_FrameCoord - vec2(0.0, tex_offset.y * i)).rgb * u_BloomWeight[i];
        }
    #else
        vec3 glowing = texture(u_LightMap, v_FrameCoord).rgb * 1.3;
    #endif

    r_Color = texture(u_Frame, v_FrameCoord) + vec4(glowing, 0.0);
}
