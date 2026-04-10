

pub mod combat {
    pub mod basetypes;
    pub mod health_basetypes;
    pub mod health;
    pub mod ships;
    pub mod attack;
}

pub mod ship {
    pub mod cargo;
    pub mod module;
    pub mod weapon;
    pub mod definition_repository;
    pub mod weapon_definition; // for storing weapon definitions that are reused between identical weapons
    pub mod ammunition_definitions; // for storing ammunition definitions that are reused between identical ammunition
    pub mod bundle; // contains all bundles for ships
}

pub mod player {
    pub mod player;
    pub mod ship;
    pub mod gameassets;
}

pub mod physics {
    pub mod raycast_damage;
}