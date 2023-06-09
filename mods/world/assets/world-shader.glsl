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

#define WAVE_HEIGHT 0.005
#define RIPPLE_STRENGTH 0.0011
#define REFLECTION_STRENGHT 0.7

in vec2 v_FrameCoord;

uniform sampler2D u_Frame;
uniform sampler2D u_WaterDepthMap;
uniform float u_Time;

layout (location = 0) out vec4 r_Color;

void main() {
    vec4 water_info = texture(u_WaterDepthMap, v_FrameCoord);
    vec2 distortion = vec2(
        RIPPLE_STRENGTH * sin(u_Time * 5.0 + water_info.b * 200.0),
        WAVE_HEIGHT * sin(u_Time + water_info.g * 2.0)
    ) * water_info.r;
    r_Color = texture(u_Frame, v_FrameCoord + distortion)
            + texture(u_Frame, vec2(v_FrameCoord.x, water_info.b) - distortion) 
            * water_info.r * REFLECTION_STRENGHT;
}
