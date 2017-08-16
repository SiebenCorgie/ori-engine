#version 450


//Vertex Shader Input
layout(location = 0) in vec3 v_normal;
layout(location = 1) in vec2 tex_coordinates;

//Textures
layout(set = 1, binding = 0) uniform sampler2D albedo;
layout(set = 1, binding = 1) uniform sampler2D normal;
layout(set = 1, binding = 2) uniform sampler2D physical;
layout(set = 1, binding = 3) uniform sampler2D emessive;
//Texture usage infos (!= 1 is "not used" for now)
layout(set = 2, binding = 0) uniform TextureUsageInfo {
  int b_albedo;
  int b_normal;
  int b_metal;
  int b_roughness;
  int b_occlusion;
  int b_emissive;
} u_tex_usage_info;
//Linear Texture factors from the material
layout(set = 2, binding = 1) uniform TextureFactors{
  vec4 albedo_factor;
  vec4 normal_factor;
  int metal_factor;
  int roughness_factor;
  int occlusion_factor;
  vec4 emissive_factor;
} u_tex_fac;

layout(location = 0) out vec4 f_color;


const vec3 LIGHT = vec3(0.0, -1.0, 1.0);

void main() {
    float brightness = dot(normalize(v_normal), normalize(LIGHT));
    //f_color = vec4(texture(albedo, tex_coordinates).xyz * brightness, 1.0);
    vec3 color = vec3(0.0);

    if (u_tex_usage_info.b_albedo == 1){
      f_color = vec4(texture(albedo, tex_coordinates).xyz * brightness, 1.0) * u_tex_fac.albedo_factor;
    }else{
      f_color = vec4(texture(albedo, tex_coordinates).xyz * vec3(1.0, 0.0, 0.0) * brightness, 1.0) * u_tex_fac.albedo_factor;
    }
}
