#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 texcoord;
layout (location = 2) in vec3 normal;
layout (location = 3) in vec3 tangent;
layout (location = 4) in vec3 binormal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform vec3 lightPos;
uniform vec3 viewPos;

out VS_OUT {
    vec3 FragPos;
    vec2 TexCoords;
    vec3 Normal;
    vec3 TangentLightPos;
    vec3 TangentViewPos;
    vec3 TangentFragPos;
} vs_out;

void main() {
  gl_Position = projection * view * model * vec4(position, 1.0);
  vs_out.FragPos = vec3(model * vec4(position, 1.0));
  vs_out.Normal = mat3(transpose(inverse(model))) * normal;
  vs_out.TexCoords = vec2(texcoord.x, texcoord.y);
  // vs_out.TexCoords = texcoord;

  mat3 normalMatrix = transpose(inverse(mat3(model)));
  vec3 T = normalize(normalMatrix * tangent);
  vec3 B = normalize(normalMatrix * binormal);
  vec3 N = normalize(normalMatrix * normal);
  mat3 TBN = transpose(mat3(T, B, N));
  vs_out.TangentLightPos = TBN * lightPos;
  vs_out.TangentViewPos  = TBN * viewPos;
  vs_out.TangentFragPos  = TBN * vs_out.FragPos;

}
