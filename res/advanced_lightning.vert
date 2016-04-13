#version 330 core

layout(location = 0) in vec4 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 uv;
layout(location = 3) in vec3 binormal;
layout(location = 4) in vec3 tangent;

uniform mat4 sys_ProjectionMatrix;
uniform mat4 sys_ViewMatrix;
uniform mat4 sys_ModelMatrix;
uniform vec3 sys_CameraPosition;

uniform mat4 u_DepthBiasMVP;

out DATA
{
	vec4 position;
	vec3 normal;
	vec2 uv;
	vec3 binormal;
	vec3 tangent;
	vec3 color;
	vec4 shadowCoord;
	vec3 cameraPos;
} vs_out;

void main()
{
	vec4 pos = sys_ModelMatrix * position;
	vs_out.position = pos;
	gl_Position = sys_ProjectionMatrix * sys_ViewMatrix * pos;

	mat3 model = mat3(sys_ModelMatrix);
	vs_out.normal = model * normal;
	vs_out.binormal = model * binormal;
	vs_out.tangent = model * tangent;
	vs_out.uv = uv;
	vs_out.color = vec3(1.0);
	vs_out.cameraPos = sys_CameraPosition;

	vs_out.shadowCoord = u_DepthBiasMVP * pos;
}
