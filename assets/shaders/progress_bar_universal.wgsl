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
    direction: u32,
};

// direction helper
fn get_progress_mask(uv: vec2<f32>, progress: f32, dir: u32) -> f32 {
    if (dir == 0u) {
        return step(uv.x, progress); // L → R
    }
    if (dir == 1u) {
        return step(1.0 - uv.x, progress); // R → L
    }
    if (dir == 2u) {
        return step(uv.y, progress); // B → T
    }
    // Top → Bottom
    return step(1.0 - uv.y, progress);
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {

    let uv = (in.uv - data.uv_offset) * data.uv_scale;

    // hard clip prevents bleeding
    if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) {
        discard;
    }

    let color = textureSample(bar_texture, bar_sampler, uv);

    if (color.a <= 0.0) {
        discard;
    }

    let filled = get_progress_mask(uv, data.progress, data.direction);

    if (filled < 0.5) {
        return vec4<f32>(color.rgb * 0.3, color.a);
    }

    return color;
}