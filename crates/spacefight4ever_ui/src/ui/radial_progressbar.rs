#[derive(Component)]
pub struct RadialProgressBar {
    pub handle: Handle<RadialProgressBarMaterial>,
}

#[derive(AsBindGroup, Asset, TypePath, Clone)]
pub struct RadialProgressBarMaterial {
    #[uniform(0)]
    pub progress: f32,

    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

// hints for shader:
// let uv = in.uv - vec2(0.5);
// let angle = atan2(uv.y, uv.x);
// let norm = (angle + PI) / (2.0 * PI);

// if (norm > data.progress) {
//     discard;
// }