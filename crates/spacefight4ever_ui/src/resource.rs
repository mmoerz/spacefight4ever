use bevy::prelude::*;



/// resource helps 
#[derive(Resource, Default, Debug)]
pub struct UiWindowAtlas {
    pub window_layout: Handle<TextureAtlasLayout>,
    pub button_layout: Handle<TextureAtlasLayout>,
    pub button_offset: usize,
}

