#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

struct CubemapMaterial {
    normal: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> material: CubemapMaterial;
@group(1) @binding(1)
var emissive_texture: texture_cube<f32>;
@group(1) @binding(2)
var emissive_sampler: sampler;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var eye_direction = normalize(in.world_position.xyz - view.world_position.xyz);
    var n = in.world_normal * material.normal.xyz; //flip for inside sphere
    n = mix(n, reflect(eye_direction, normalize(n)), material.normal.w);
    return vec4(pow(textureSample(emissive_texture, emissive_sampler, n).rgb, vec3(1.0)), 1.0);
}
