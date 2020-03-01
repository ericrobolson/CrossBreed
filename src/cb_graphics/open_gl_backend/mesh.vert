#version 330 core

layout (location = 0) in vec3 myPosition;
layout (location = 1) in vec3 myColor;
layout (location = 2) in vec3 myNormal;

uniform mat4 MVP;

out vec3 cbMeshVertexColor;
out vec3 cbFragPos;
out vec3 cbNormal;

void main()
{
    cbFragPos = vec3(MVP * vec4(myPosition, 1));
    gl_Position = MVP * vec4(myPosition, 1);
    cbMeshVertexColor = myColor;    
    cbNormal = myNormal;    
}
