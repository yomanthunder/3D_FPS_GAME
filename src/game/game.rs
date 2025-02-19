use bevy::prelude::*;
use crate::game::window; 

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            window::WindowPlugin,
        ));
    }
}
