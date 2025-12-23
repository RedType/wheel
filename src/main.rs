use bevy::prelude::*;
use wheel::vincubus::VincubusPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(VincubusPlugin)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d::default());
}
