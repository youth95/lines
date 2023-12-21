#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(1) @binding(0) var<uniform> material_color: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(1., 1., 1., 1.);
}
