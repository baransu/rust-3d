#version 330 core

uniform sampler2D normalMap;
uniform sampler2D diffuseMap;
uniform sampler2D specularMap;

uniform vec3 lightPos;

out vec4 color;

in VS_OUT {
    vec3 FragPos;
    vec2 TexCoords;
    vec3 Normal;
    vec3 TangentLightPos;
    vec3 TangentViewPos;
    vec3 TangentFragPos;
} fs_in;

float ambientStrength = 0.1f;
float specularStrength = 0.5f;
//
// vec3 lightColor = vec3(1.0, 1.0, 1.0);
// vec3 objectColor = vec3(0.5, 0.0, 0.5);

const float constant = 1.0f;
const float linear = 0.09f;
const float quadratic = 0.032f;

const float Pi = 3.14159265;
const float shininess = 32.0;

void main() {
  // Obtain normal map from texture [0, 1]
  vec3 normal = texture(normalMap, fs_in.TexCoords).rgb;
  // convert to [-1, 1]
  normal = normalize(normal * 2.0 - 1.0);

  // diffuse color from texture
  vec3 col = texture(diffuseMap, fs_in.TexCoords).rgb;

  // ambient
  vec3 ambient = ambientStrength * col;

  // diffuse
  vec3 lightDir = normalize(fs_in.TangentLightPos - fs_in.TangentFragPos);
  float diff = max(dot(lightDir, normal), 0.0);
  vec3 diffuse = diff * col;

  // specular
  vec3 viewDir = normalize(fs_in.TangentViewPos - fs_in.TangentFragPos);
  vec3 reflectDir = reflect(-lightDir, normal);
  vec3 halfwayDir = normalize(lightDir + viewDir);
  float spec = pow(max(dot(normal, halfwayDir), 0.0), shininess);

  // vec3 specular = vec3(specularStrength) * spec;
  vec3 specular = vec3(specularStrength) * spec * vec3(texture(specularMap, fs_in.TexCoords));

  // float distance = length(lightPos - fs_in.FragPos);
  // float attenuation = 1.0f / (constant + linear * distance + quadratic * (distance * distance));
  //
  // ambient *= attenuation;
  // diffuse *= attenuation;
  // specular *= attenuation;

  vec3 result = ambient + diffuse + specular;
  color = vec4(result, 1.0);

}
