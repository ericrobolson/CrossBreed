#version 330 core

in vec3 cbMeshVertexColor;
in vec3 cbFragPos;
in vec3 cbNormal;

out vec4 FragColor;

void main()
{
    vec3 lightColor = vec3(1.0, 0.96078431372, 0.85098039215);
    vec3 lightPos = vec3(100.0, 100.0, 100.0);

    vec3 lightDir = normalize(lightPos - cbFragPos);  

    float ambientStrength = 0.2;
    vec3 ambient = ambientStrength * lightColor;

    vec3 norm = normalize(cbNormal);

    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;
    


    vec3 result = (ambient + diffuse) * cbMeshVertexColor;

    FragColor = vec4(result, 1.0);
}