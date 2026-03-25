pub mod bundle;

pub mod messages;
pub mod state;
pub mod layers;

pub mod dialog_stack;
pub mod dialog_manager;

pub mod debug;

pub mod input {
    pub mod keybindings;
}

pub mod hud {
    pub mod hud_root;
    pub mod health_display;
    pub mod ship_modul_bar;
}

pub mod window {
    pub mod structs;
    pub mod consts;
    pub mod component;
    pub mod bundle;
    pub mod window;
    //pub mod window_ninepatch;
    pub mod systems {
        pub mod drag_system;
        pub mod resize;
        pub mod minmax;
        pub mod close;
    }
}


pub mod dialogs {
    pub mod confirm_exit;
    pub mod message;
    pub mod ship_equipment;
}

// pub mod widgets {
//     pub mod button;
// }

pub mod systems {
    pub mod button;
    pub mod dialog;
}