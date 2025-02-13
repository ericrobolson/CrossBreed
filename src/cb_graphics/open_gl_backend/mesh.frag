#version 330 core

uniform vec3 cbLightPos;  

in vec3 cbMeshVertexColor;
in vec3 cbFragPos;
in vec3 cbNormal;

out vec4 FragColor;

void main()
{
    vec3 lightColor = vec3(1.0, 0.96078431372, 0.85098039215);

    vec3 lightDir = normalize(cbLightPos - cbFragPos);  

    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    vec3 norm = normalize(cbNormal);

    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;
    


    vec3 result = (ambient + diffuse) * cbMeshVertexColor;

    FragColor = vec4(result, 1.0);
}