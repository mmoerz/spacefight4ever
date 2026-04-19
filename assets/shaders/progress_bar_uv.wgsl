#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0)
var<uniform> data: UiProgressBarUniform;

@group(1) @binding(1)
var bar_texture: texture_2d<f32>;

@group(1) @binding(2)
var bar_sampler: sampler;

struct UiProgressBarUniform {
    progress: f32,
    uv_offset: vec2<f32>,
    uv_scale: vec2<f32>,
};

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {

    let uv = (in.uv - data.uv_offset) * data.uv_scale;

    let color = textureSample(bar_texture, bar_sampler, uv);

    if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) {
        discard;
    }

    if (color.a <= 0.0) {
        discard;
    }

    if (uv.x > data.progress) {
        return vec4<f32>(color.rgb * 0.3, color.a);
    }

    return color;
}