use bevy::prelude::*;

pub mod loading;
pub mod phrases;
pub mod vincubus;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
  #[default]
  Splash,
  MainMenu,
  SettingsMenu,
  InGame,
}
