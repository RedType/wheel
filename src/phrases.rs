use bevy::prelude::*;

#[derive(Reflect)]
pub struct Phrase {
  pub hint: String,
  pub text: String,
}

#[derive(Asset, Reflect)]
pub struct Topic {
  pub phrases: Vec<Phrase>,
}

////////////
// Plugin //
////////////

pub struct PhrasePlugin;

impl Plugin for PhrasePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_asset::<Topic>()
      ;
  }
}
