use std::{error, fmt, fs};

use bevy::{
  asset::{AssetLoader, LoadContext, io},
  platform::collections::HashSet,
  prelude::*,
};
use rand::prelude::*;

pub struct PhrasePlugin;

impl Plugin for PhrasePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_asset::<Topic>()
      .init_asset_loader::<TopicLoader>()
      .add_systems(Startup, setup)
      .add_systems(Update, watch_topics_load.run_if(run_if_loading));
  }
}

////////////
// Topics //
////////////

#[derive(Resource)]
pub struct Topics {
  pub finished_loading: bool,
  pub all:              HashSet<Handle<Topic>>,
  pub enabled:          HashSet<Handle<Topic>>,
}

#[derive(Asset, Reflect)]
pub struct Topic {
  pub name:    String,
  pub phrases: Vec<String>,
}

impl Topics {
  pub fn pick(&self) -> Handle<Topic> {
    let mut rng = rand::rng();
    let pick = rng.random_range(0..self.enabled.len());

    self.enabled.iter().skip(pick).next().unwrap().clone()
  }

  pub fn toggle(&mut self, topic: Handle<Topic>) {
    if self.enabled.contains(&topic) {
      self.enabled.remove(&topic);
    } else {
      self.enabled.insert(topic);
    }
  }
}

impl Topic {
  pub fn pick(&self) -> &str {
    let mut rng = rand::rng();
    let pick = rng.random_range(0..self.phrases.len());

    &self.phrases[pick]
  }
}

///////////////////
// Asset Loading //
///////////////////

#[derive(Default)]
struct TopicLoader;

impl AssetLoader for TopicLoader {
  type Asset = Topic;
  type Error = TopicLoadError;
  type Settings = ();

  async fn load(
    &self,
    reader: &mut dyn io::Reader,
    _settings: &Self::Settings,
    _load_context: &mut LoadContext<'_>,
  ) -> Result<Self::Asset, Self::Error> {
    // load all bytes
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes);

    // convert to rust string
    let Ok(text) = String::try_from(bytes) else {
      return Err(TopicLoadError::ConversionError);
    };

    // split into lines
    let mut lines = text.lines().map(|l| l.trim());

    // get topic name
    let Some(topic_name) = lines.next() else {
      return Err(TopicLoadError::LengthError);
    };

    let phrases = lines.map(|l| l.to_uppercase()).collect::<Vec<_>>();

    Ok(Topic {
      name: topic_name.to_owned(),
      phrases,
    })
  }
}

#[derive(Debug)]
pub enum TopicLoadError {
  ConversionError,
  LengthError,
}

impl fmt::Display for TopicLoadError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ConversionError => {
        write!(f, "Topic file must be UTF-8 encoded text")
      }
      Self::LengthError => {
        write!(f, "Topic file must be at least 2 lines")
      }
    }
  }
}

impl error::Error for TopicLoadError {}

/////////////////
// Load Topics //
/////////////////

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let mut topics = Topics {
    finished_loading: false,
    all:              HashSet::new(),
    enabled:          HashSet::new(),
  };

  let topic_file_paths = fs::read_dir("assets/topics")
    .expect("Unable to read topic files")
    .map(|f| f.unwrap())
    .filter(|f| f.file_type().unwrap().is_file())
    .map(|f| f.path());

  for path in topic_file_paths {
    let handle = asset_server.load(path);

    topics.all.insert(handle.clone());
    topics.enabled.insert(handle);
  }

  commands.insert_resource(topics);
}

fn watch_topics_load(mut topics: ResMut<Topics>, asset_server: Res<AssetServer>) {
  if topics.finished_loading {
    return;
  }

  for handle in &topics.all {
    let loaded = asset_server.is_loaded_with_dependencies(handle);

    if !loaded {
      return;
    }
  }

  // if we got this far, then all have loaded
  topics.finished_loading = true;
}

fn run_if_loading(topics: Res<Topics>) -> bool {
  !topics.finished_loading
}
