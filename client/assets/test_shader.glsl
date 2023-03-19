#[description]
This is a short test shader for the client renderer

#[vertex]
#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 in_color;

smooth out vec4 vertex_color;

void main() {
    gl_Position = vec4(position, 1.0);
    vertex_color = vec4(in_color, 1.0);
}

#[fragment]
#version 330 core

smooth in vec4 vertex_color;
out vec4 color;

void main() {
    color = vertex_color;
}
