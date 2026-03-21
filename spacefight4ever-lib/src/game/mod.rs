

pub mod combat {
    pub mod basetypes;
    pub mod health_basetypes;
    pub mod health;
    pub mod ships;
    pub mod attack;
}

pub mod ship {
    pub mod bundle; // contains all bundles for ships
    pub mod module;
    pub mod weapon;
}

pub mod player {
    pub mod player;
    pub mod ship;
}

pub mod physics {
    pub mod raycast_damage;
}