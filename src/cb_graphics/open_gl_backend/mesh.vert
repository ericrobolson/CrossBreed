#version 330 core

layout (location = 0) in vec3 myPosition;
layout (location = 1) in vec3 myColor;

uniform mat4 MVP;

out vec3 meshVertexColor;

void main()
{
    gl_Position = MVP * vec4(myPosition, 1);
    meshVertexColor = myColor;    
}
