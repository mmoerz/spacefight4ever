use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DialogStack {
    pub stack: Vec<DialogType>,
}

#[derive(Clone)]
pub enum DialogType {
    ConfirmExit,
    Message(String),
    ShipEquipment,
}

impl DialogStack {
    pub fn push(&mut self, dialog: DialogType) {
        self.stack.push(dialog);
    }
    pub fn pop(&mut self) {
        self.stack.pop();
    }
    pub fn top(&self) -> Option<&DialogType> {
        self.stack.last()
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn len(&self) -> usize {
        self.stack.len()
    }
}