#version 450

layout(location = 0) in vec3 v_normal;
layout(location = 1) in vec2 tex_coordinates;


layout(location = 0) out vec4 f_color;

layout(set = 1, binding = 0) uniform sampler2D albedo;
layout(set = 1, binding = 1) uniform sampler2D normal;
layout(set = 1, binding = 2) uniform sampler2D physical;

const vec3 LIGHT = vec3(0.0, -1.0, 1.0);

void main() {
    float brightness = dot(normalize(v_normal), normalize(LIGHT));
    f_color = vec4(texture(albedo, tex_coordinates).xyz * brightness, 1.0);
}
