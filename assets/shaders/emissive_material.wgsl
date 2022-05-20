
#import bevy_pbr::mesh_view_bind_group

struct CubemapMaterial {
    normal: vec4<f32>;
};
[[group(1), binding(0)]]
var<uniform> material: CubemapMaterial;
[[group(1), binding(1)]]
var emissive_texture: texture_cube<f32>;
[[group(1), binding(2)]]
var emissive_sampler: sampler;

struct FragmentInput {
    [[builtin(front_facing)]] is_front: bool;
    [[builtin(position)]] frag_coord: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    var eye_direction = normalize(in.world_position.xyz - view.world_position.xyz);
    var n = in.world_normal * material.normal.xyz; //flip for inside sphere
    n = mix(n, reflect(eye_direction, normalize(n)), material.normal.w);
    return vec4<f32>(pow(textureSample(emissive_texture, emissive_sampler, n).rgb, vec3<f32>(1.0)), 1.0);
}
