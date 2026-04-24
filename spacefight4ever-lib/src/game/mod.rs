

pub mod combat {
    pub mod basetypes;
    pub mod health_basetypes;
    pub mod health;
    pub mod ships;
    pub mod attack;
}

pub mod ship {
    pub mod definitions {
        pub mod load_error;
        pub mod definition_repository;
        pub mod module_definition;
        pub mod weapon_definition; // for storing weapon definitions that are reused between identical weapons
        pub mod ammunition_definition; // for storing ammunition definitions that are reused between identical ammunition
        pub mod ship_definition;
        pub mod ship_models; // for storing the asset handles of ship models
    }
    pub mod modules {
        pub mod stats;

        pub mod module;
        pub mod mountpoint;
        pub mod propulsion;
    }
    pub mod cargo;
    pub mod weapon;
    
    //pub mod bundle; // contains all bundles for ships
}

pub mod player {
    pub mod player;
    pub mod playership;
    //pub mod gltf_playership;
}

pub mod physics {
    pub mod raycast_damage;
}

pub mod assets;