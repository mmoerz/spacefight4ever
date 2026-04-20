use bevy::prelude::*;

use super::progressbar_material::UiLinearProgressBarMaterial;
use super::progressbar::*;


#[derive(Message)]
pub enum UiProgressBarCommand {
    SetProgress {
        entity: Entity,
        value: f32,
    },
}

pub fn ui_bar_executor_system(
    mut events: MessageReader<UiProgressBarCommand>,
    bars: Query<&UiLinearProgressBar>,
    mut materials: ResMut<Assets<UiLinearProgressBarMaterial>>,
    //time: Res<Time>,
    //mut anim_state: ResMut<UiBarAnimationState>,
) {
    for event in events.read() {
        match event {
            UiProgressBarCommand::SetProgress { entity, value } => {
                if let Ok(bar) = bars.get(*entity) {
                    if let Some(mat) = materials.get_mut(&bar.0) {
                        mat.data.progress = value.clamp(0.0, 1.0);
                    }
                }
            }

            // UiBarCommand::AnimateTo { entity, target, duration } => {
            //     anim_state.add_animation(*entity, *target, *duration);
            // }

            // UiBarCommand::Flash { entity } => {
            //     anim_state.trigger_flash(*entity);
            // }
        }
    }
}

pub struct UiProgressBarApi<'w> {
    pub writer: MessageWriter<'w, UiProgressBarCommand>,
}

impl<'w> UiProgressBarApi<'w> {
    pub fn set_progress(&mut self, entity: Entity, value: f32) {
        self.writer.write(UiProgressBarCommand::SetProgress {
            entity,
            value,
        });
    }

    // pub fn animate_to(&mut self, entity: Entity, target: f32, duration: f32) {
    //     self.writer.send(UiBarCommand::AnimateTo {
    //         entity,
    //         target,
    //         duration,
    //     });
    // }

    // pub fn flash(&mut self, entity: Entity) {
    //     self.writer.send(UiBarCommand::Flash { entity });
    // }
}