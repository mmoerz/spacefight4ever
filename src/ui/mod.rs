pub mod bundle;

pub mod messages;
pub mod state;
pub mod layers;

pub mod dialog_stack;
pub mod dialog_manager;

pub mod debug;

pub mod dialogs {
    pub mod confirm_exit;
    pub mod message;
}

// pub mod widgets {
//     pub mod button;
// }

pub mod systems {
    pub mod button;
    pub mod dialog;
}