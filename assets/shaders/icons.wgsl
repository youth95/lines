// This shader draws a circle with a given input color
#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0) var<uniform> pos: vec2<f32>;
@group(1) @binding(1) var<uniform> size: f32;
@group(1) @binding(2) var icons_texture: texture_2d<f32>;
@group(1) @binding(3) var icons_sampler: sampler;

@fragment
fn fragment(mesh: UiVertexOutput) -> @location(0) vec4<f32> {
    var scale = 128.0 / size;
    var uv = (mesh.uv + pos) / scale;
    return textureSample(icons_texture, icons_sampler, uv);
}

