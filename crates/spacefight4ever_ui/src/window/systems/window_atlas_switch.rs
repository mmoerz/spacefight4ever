use bevy::prelude::*;

use crate::component::*;

#[derive(Resource)]
pub struct UiWindowAtlasStatus {
    pub current: usize,
    pub total: usize,
}

impl Default for UiWindowAtlasStatus {
    fn default() -> Self {
        Self {
            current: 0,
            total: 15,
        }
    }
}


#[derive(Message)]
pub struct UiWindowsSwitchAtlasRequest(pub usize);

pub fn window_atlas_switch_system(
    mut switch_message: MessageReader<UiWindowsSwitchAtlasRequest>,
    mut query_windows: Query<&mut ImageNode, With<UiWindow>>,
    mut atlas_status: ResMut<UiWindowAtlasStatus>,
) {
    for message in switch_message.read() {
        if message.0 < atlas_status.total {
            println!("{:?}", message.0);
            atlas_status.current = message.0 ;
        } else {
            atlas_status.current = 0;
        }
        for mut window in &mut query_windows {
            if let Some(atlas) = &mut window.texture_atlas {
                atlas.index = atlas_status.current;
            }
        }
    }
}