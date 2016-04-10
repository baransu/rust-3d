#version 410 core

// uniform sampler2D texture1;
// uniform sampler2D texture2;

uniform vec3 lightPos;
uniform vec3 viewPos;

out vec4 color;

in vec3 FragPos;
in vec3 Normal;
// in vec2 TexCoords;

float ambientStrength = 0.1f;
float specularStrength = 0.5f;

vec3 lightColor = vec3(1.0, 1.0, 1.0);
vec3 objectColor = vec3(0.5, 0.0, 0.5);

void main() {

  // ambient
  vec3 ambient = ambientStrength * lightColor;

  // diffues
  vec3 norm = normalize(Normal);
  vec3 lightDir = normalize(lightPos - FragPos);
  float diff = max(dot(norm, lightDir), 0.0);
  vec3 diffuse = diff * lightColor;

  // specular
  vec3 viewDir = normalize(viewPos - FragPos);
  vec3 reflectDir = reflect(-lightDir, norm);
  float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32.0);
  vec3 specular = specularStrength * spec * lightColor;

  vec3 result = (ambient + diffuse + specular) * objectColor;
  color = vec4(result, 1.0);
  // vec2 uv = vec2(texCoords.x, 1.0 - texCoords.y);
  // color = mix(texture(texture1, uv), texture(texture2, uv), 0.25);
}
