use bevy::prelude::*;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(hello_world());
    }
}

fn hello_world(){
    println!("Hello World")
}