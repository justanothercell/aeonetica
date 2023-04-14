#[description]
Default shader used when no postprocessing shader is set up

#[vertex]
#version 450 core

layout (location = 0) in vec3 a_Position;
layout (location = 1) in vec2 a_FrameCoord;

out vec2 v_FrameCoord;

void main() {
    v_FrameCoord = a_FrameCoord;
    gl_Position = vec4(a_Position, 1.0);
}

#[fragment]
#version 450 core

in vec2 v_FrameCoord;

uniform sampler2D u_Frame;

out vec4 color;

const float offset = 1.0 / 300.0;

void main() {
    vec2 offsets[9] = vec2[](
        vec2(-offset,  offset), // top-left
        vec2( 0.0f,    offset), // top-center
        vec2( offset,  offset), // top-right
        vec2(-offset,  0.0f),   // center-left
        vec2( 0.0f,    0.0f),   // center-center
        vec2( offset,  0.0f),   // center-right
        vec2(-offset, -offset), // bottom-left
        vec2( 0.0f,   -offset), // bottom-center
        vec2( offset, -offset)  // bottom-right    
    );

    float kernel[9] = float[](
        -1, -1, -1,
        -1,  9, -1,
        -1, -1, -1
    );
    
    vec3 sampleTex[9];
    for(int i = 0; i < 9; i++)
    {
        sampleTex[i] = vec3(texture(u_Frame, v_FrameCoord.st + offsets[i]));
    }
    vec3 col = vec3(0.0);
    for(int i = 0; i < 9; i++)
        col += sampleTex[i] * kernel[i];
    
    color = vec4(col, 1.0);
}
