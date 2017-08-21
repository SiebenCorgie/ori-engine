#version 450

//General definition
#define MAX_DIR_LIGHTS 1
#define MAX_POINT_LIGHTS 6
#define MAX_SPOT_LIGHTS 1


///INS FROM VERTEX
//Vertex Shader Input
layout(location = 0) in vec3 v_normal;
layout(location = 1) in vec2 tex_coordinates;

//TEXTURES
layout(set = 1, binding = 0) uniform sampler2D albedo;
layout(set = 1, binding = 1) uniform sampler2D normal;
layout(set = 1, binding = 2) uniform sampler2D physical;
layout(set = 1, binding = 3) uniform sampler2D emissive;
//TEXTURE_USAGE
//Texture usage infos (!= 1 is "not used" for now)
layout(set = 2, binding = 0) uniform TextureUsageInfo {
  int b_albedo;
  int b_normal;
  int b_metal;
  int b_roughness;
  int b_occlusion;
  int b_emissive;
} u_tex_usage_info;

//TEXTURE_FACTORS
//Linear Texture factors from the material
layout(set = 2, binding = 1) uniform TextureFactors {
  vec4 albedo_factor;
  vec4 normal_factor;
  vec4 emissive_factor;
  int metal_factor;
  int roughness_factor;
  int occlusion_factor;
} u_tex_fac;

//LIGHTS
//definitions of the lights for the unsized arrays
struct PointLight
{
  float intensity;
  vec3 color;
};

struct DirectionalLight
{
  float intensity;
  vec3 color;
  vec3 direction;
};

struct SpotLight
{
  float intensity;
  vec3 color;
  vec3 direction;
  float outer_radius;
  float inner_radius;
};

//And the send bindings from rust/vulkano
layout(set = 3, binding = 0) uniform point_lights{
  PointLight p_light[MAX_POINT_LIGHTS];
}u_point_light;

layout(set = 3, binding = 1) uniform directional_lights{
  DirectionalLight d_light[MAX_DIR_LIGHTS];
}u_dir_light;

layout(set = 3, binding = 2) uniform spot_lights{
  SpotLight s_light[MAX_SPOT_LIGHTS];
}u_spot_light;

//defines the number of lights used
layout(set = 3, binding = 3) uniform number_of_lights{
  int num_point_lights;
  int num_dir_lights;
  int num_spot_lights;
}u_light_count;

///outgoing final color
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
