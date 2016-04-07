#version 410 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 uvs;
layout (location = 2) in vec3 color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 col;
out vec2 texCoords;

void main() {
  gl_Position = projection * model * vec4(position, 1.0);
  col = color;
  texCoords = uvs;
}
