#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0) var<uniform> progress: f32;
@group(1) @binding(1) var bar_texture: texture_2d<f32>;
@group(1) @binding(2) var bar_sampler: sampler;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(bar_texture, bar_sampler, in.uv);
    
    // Use the bar's alpha as a mask for everything
    if (color.a <= 0.0) {
        discard;
    }

    // If UV.x is beyond the progress, darken the pixel
    if (in.uv.x > progress) {
        // Blend with 50% black
        return vec4<f32>(color.rgb * 0.3, color.a);
    }

    return color;
}