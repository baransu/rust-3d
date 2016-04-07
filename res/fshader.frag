#version 410 core

uniform sampler2D texture1;
uniform sampler2D texture2;

out vec4 color;

in vec3 col;
in vec2 texCoords;

void main() {
  vec2 uv = vec2(texCoords.x, 1.0 - texCoords.y);
  color = mix(texture(texture1, uv), texture(texture2, uv), 0.25);
}
