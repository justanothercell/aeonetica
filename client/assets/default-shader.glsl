#[description]
Default shader used when no postprocessing shader is set up

#[vertex]
#version 400 core

layout (location = 0) in vec3 a_Position;
layout (location = 1) in vec2 a_FrameCoord;

out vec2 v_FrameCoord;

void main() {
    v_FrameCoord = a_FrameCoord;
    gl_Position = vec4(a_Position, 1.0);
}

#[fragment]
#version 400 core

in vec2 v_FrameCoord;

uniform sampler2D u_Frame;

out vec4 color;

void main() {
    color = texture(u_Frame, v_FrameCoord);
}
