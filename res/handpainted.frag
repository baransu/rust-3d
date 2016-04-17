#version 330 core

uniform sampler2D diffuseMap;
uniform sampler2D specularMap;
uniform sampler2D normalMap;

uniform vec3 viewPos;

out vec4 color;

in VS_OUT {
    vec3 FragPos;
    vec2 TexCoords;
    vec3 Normal;
    vec3 TangentLightPos;
    vec3 TangentViewPos;
    vec3 TangentFragPos;
} fs_in;

struct DirLight {
    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};
uniform DirLight dirLight;

struct PointLight {
    vec3 position;

    float constant;
    float linear;
    float quadratic;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};
uniform PointLight pointLight;

const float Pi = 3.14159265;
const float shininess = 32.0;

vec3 calcDirLight(DirLight light, vec3 normal, vec3 viewDir) {
  vec3 lightDir = normalize(-light.direction);

  // diffuse
  float diff = max(dot(normal, lightDir), 0.0);

  // specular
  vec3 reflectDir = reflect(-lightDir, normal);
  vec3 halfwayDir = normalize(lightDir + viewDir);
  float spec = pow(max(dot(normal, halfwayDir), 0.0), 32.0);

  // result
  vec3 diffuseColor = texture(diffuseMap, fs_in.TexCoords).rgb;
  vec3 ambient = light.ambient * diffuseColor;
  vec3 diffuse = light.diffuse * diff * diffuseColor;
  vec3 specular = light.specular * spec * texture(specularMap, fs_in.TexCoords).rgb;

  return (ambient + diffuse + specular);
}

vec3 calcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir) {

  vec3 lightDir = normalize(light.position - fragPos);

  // diffuse
  float diff = max(dot(normal, lightDir), 0.0);

  // specular
  vec3 reflectDir = reflect(-lightDir, normal);
  vec3 halfwayDir = normalize(lightDir + viewDir);
  float spec = pow(max(dot(normal, halfwayDir), 0.0), 32.0);

  // attenuation
  float distance = length(light.position - fragPos);
  float attenuation = 1.0f / (light.constant + light.linear * distance +  light.quadratic * (distance * distance));

  // result
  vec3 diffuseColor = texture(diffuseMap, fs_in.TexCoords).rgb;
  vec3 ambient = light.ambient * diffuseColor;
  vec3 diffuse = light.diffuse * diff * diffuseColor;
  vec3 specular = light.specular * spec * texture(specularMap, fs_in.TexCoords).rgb;

  ambient *= attenuation;
  diffuse *= attenuation;
  specular *= attenuation;

  return (ambient + diffuse + specular);
}

void main() {

  // vec3 col = texture(diffuseMap, fs_in.TexCoords).rgb;
  // // Ambient
  // vec3 ambient = light.ambient * col;
  // // Diffuse
  // // vec3 lightDir = normalize(light.position - fs_in.FragPos);
  // vec3 lightDir = normalize(-light.direction);
  // vec3 normal = normalize(fs_in.Normal);
  // float diff = max(dot(lightDir, normal), 0.0);
  // vec3 diffuse = light.diffuse * diff * col;
  // // Specular
  // vec3 viewDir = normalize(viewPos - fs_in.FragPos);
  // vec3 reflectDir = reflect(-lightDir, normal);
  //
  // vec3 halfwayDir = normalize(lightDir + viewDir);
  // float spec = pow(max(dot(normal, halfwayDir), 0.0), 32.0);
  //
  // vec3 specular = light.specular * spec * vec3(texture(specularMap, fs_in.TexCoords)); // assuming bright white light color
  //
  // float distance = length(light.position - fs_in.FragPos);
  // float attenuation = 1.0f / (light.constant + light.linear * distance + light.quadratic * (distance * distance));
  //
  // ambient *= attenuation;
  // diffuse *= attenuation;
  // specular *= attenuation;

  vec3 norm = normalize(fs_in.Normal);
  vec3 viewDir = normalize(viewPos - fs_in.FragPos);

  // directional light
  vec3 result = calcDirLight(dirLight, norm, viewDir);

  // point light
  result += calcPointLight(pointLight, norm, fs_in.FragPos, viewDir);

  color = vec4(result, 1.0f);

}
