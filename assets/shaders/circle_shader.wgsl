// This shader draws a circle with a given input color
#import bevy_ui::ui_vertex_output::UiVertexOutput

struct CustomUiMaterial {
    @location(0) color: vec4<f32>
}

@group(1) @binding(0)
var<uniform> input: CustomUiMaterial;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    // the UVs are now adjusted around the middle of the rect.
    let uv = in.uv * 2.0 - 1.0;
    
    // circle alpha, the higher the power the harsher the falloff.
    let alpha1 = 0.9 - sqrt(dot(uv, uv));
    let alpha2 = 1.0 - sqrt(dot(uv, uv));

    let alpha = ceil(alpha2) - ceil(alpha1);



    // let alpha2 = 1.0 - pow(sqrt(dot(uv, uv)), 8.0);

    return vec4<f32>(input.color.rgb, alpha);
}

