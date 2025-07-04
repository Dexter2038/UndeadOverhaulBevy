use bevy::prelude::*;

use crate::player::systems::{handle_ground_contacts, pickup_drill, player_input_system, setup};

mod bundle;
mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, player_input_system);
        app.add_systems(Update, handle_ground_contacts);
        app.add_systems(Update, pickup_drill);
    }
}
