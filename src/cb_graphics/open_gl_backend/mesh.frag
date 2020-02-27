#version 330 core

in vec3 meshVertexColor;
out vec3 FragColor;

void main()
{
    FragColor = meshVertexColor;
}