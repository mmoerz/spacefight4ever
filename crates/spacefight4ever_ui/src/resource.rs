use bevy::prelude::*;



/// resource helps 
#[derive(Resource, Default, Debug)]
pub struct UiWindowAtlas {
    pub window_layout: Handle<TextureAtlasLayout>,
    pub button_layout: Handle<TextureAtlasLayout>,
    pub button_offset: usize,
}

/// resource to track the focused window
#[derive(Resource, Debug)]
pub struct UiWindowFocused(Entity);

impl Default for UiWindowFocused {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

impl UiWindowFocused {
    pub fn set(&mut self, entity: Entity) {
        self.0 = entity;
    }
    pub fn get(&self) -> Entity {
        self.0
    }
}