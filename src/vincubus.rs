use bevy::{
  platform::collections::HashSet,
  prelude::*,
};

pub struct VincubusPlugin;

impl Plugin for VincubusPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup)
      .add_systems(Update, (
        update_load_state,
        update_eye,
        update_head,
        update_l_arm,
        update_r_arm,
        update_tail,
      ));
  }
}

#[derive(Component)]
pub struct Vincubus {
  eye_facing:    Entity,
  eye_looking:   Entity,
  head_neutral:  Entity,
  head_sad:      Entity,
  head_surprise: Entity,
  head_talk:     Entity,
  l_arm_hip:     Entity,
  l_arm_shrug:   Entity,
  _body:         Entity,
  r_arm_hangin:  Entity,
  r_arm_shrug:   Entity,
  tail_flip:     Entity,
  tail_flop:     Entity,
}

///////////////////
// Sprite States //
///////////////////

#[derive(Component)]
pub enum EyeState {
  Hidden,
  Facing,
  Looking,
}

#[derive(Component)]
pub enum HeadState {
  Neutral,
  Sad,
  Surprise,
  Talk,
}

#[derive(Component)]
pub enum LArmState {
  Hip,
  Shrug
}

#[derive(Component)]
pub enum RArmState {
  Hangin,
  Shrug,
}

#[derive(Component)]
pub enum TailState {
  Flip,
  Flop,
}

//////////////////////////
// Sprite State Systems //
//////////////////////////

fn update_eye(
  q: Query<(&Vincubus, &EyeState), Changed<EyeState>>,
  mut eyes: Query<&mut Visibility>,
) {
  for (vincubus, eye_state) in &q {
    let mut facing = eyes.get_mut(vincubus.eye_facing)
      .expect("Ol vinc is missing an eye!");
    *facing = if matches!(eye_state, &EyeState::Facing) {
      Visibility::Visible
    } else {
      Visibility::Hidden
    };

    let mut looking = eyes.get_mut(vincubus.eye_looking)
      .expect("Ol vinc is missing an eye!");
    *looking = if matches!(eye_state, &EyeState::Looking) {
      Visibility::Visible
    } else {
      Visibility::Hidden
    };
  }
}

fn update_head(
  q: Query<(&Vincubus, &HeadState), Changed<HeadState>>,
  mut heads: Query<&mut Visibility>,
) {
  for (vincubus, head_state) in &q {
    let mut neutral = heads.get_mut(vincubus.head_neutral)
      .expect("Ol vinc is missing a head!");
    *neutral = if matches!(head_state, &HeadState::Neutral) {
      Visibility::Visible
    } else {
      Visibility::Hidden
    };

    let mut sad = heads.get_mut(vincubus.head_sad)
      .expect("Ol vinc is missing a head!");
    *sad = if matches!(head_state, &HeadState::Sad) {
      Visibility::Visible
    } else {
      Visibility::Hidden
    };

    let mut surprise = heads.get_mut(vincubus.head_surprise)
      .expect("Ol vinc is missing a head!");
    *surprise = if matches!(head_state, &HeadState::Surprise) {
      Visibility::Visible
    } else {
      Visibility::Hidden
    };

    let mut talk = heads.get_mut(vincubus.head_talk)
      .expect("Ol vinc is missing a head!");
    *talk = if matches!(head_state, &HeadState::Talk) {
      Visibility::Visible
    } else {
      Visibility::Hidden
    };
  }
}

fn update_l_arm(
  q: Query<(&Vincubus, &LArmState), Changed<LArmState>>,
  mut l_arms: Query<&mut Visibility>,
) {
  for (vincubus, l_arm_state) in &q {
    let mut hip = l_arms.get_mut(vincubus.l_arm_hip)
      .expect("Ol vinc is missing an l_arm!");
    *hip = if matches!(l_arm_state, &LArmState::Hip) {
      Visibility::Visible
    } else {
      Visibility::Hidden
    };

    let mut shrug = l_arms.get_mut(vincubus.l_arm_shrug)
      .expect("Ol vinc is missing an l_arm!");
    *shrug = if matches!(l_arm_state, &LArmState::Shrug) {
      Visibility::Visible
    } else {
      Visibility::Hidden
    };
  }
}

fn update_r_arm(
  q: Query<(&Vincubus, &RArmState), Changed<RArmState>>,
  mut r_arms: Query<&mut Visibility>,
) {
  for (vincubus, r_arm_state) in &q {
    let mut hangin = r_arms.get_mut(vincubus.r_arm_hangin)
      .expect("Ol vinc is missing an r_arm!");
    *hangin = if matches!(r_arm_state, &RArmState::Hangin) {
      Visibility::Visible
    } else {
      Visibility::Hidden
    };

    let mut shrug = r_arms.get_mut(vincubus.r_arm_shrug)
      .expect("Ol vinc is missing an r_arm!");
    *shrug = if matches!(r_arm_state, &RArmState::Shrug) {
      Visibility::Visible
    } else {
      Visibility::Hidden
    };
  }
}

fn update_tail(
  q: Query<(&Vincubus, &TailState), Changed<TailState>>,
  mut tails: Query<&mut Visibility>,
) {
  for (vincubus, tail_state) in &q {
    let mut flip = tails.get_mut(vincubus.tail_flip)
      .expect("Ol vinc is missing a tail!");
    *flip = if matches!(tail_state, &TailState::Flip) {
      Visibility::Visible
    } else {
      Visibility::Hidden
    };

    let mut flop = tails.get_mut(vincubus.tail_flop)
      .expect("Ol vinc is missing a tail!");
    *flop = if matches!(tail_state, &TailState::Flop) {
      Visibility::Visible
    } else {
      Visibility::Hidden
    };
  }
}

///////////////////
// Spawn Command //
///////////////////

struct SpawnVincubusCommand;

impl Command for SpawnVincubusCommand {
  fn apply(self, world: &mut World) {
    // Eyes
    let eye_facing = world.spawn((
      Sprite {
        image: world.load_asset(sprite_filenames::EYE_FACING),
        ..Default::default()
      },
      Transform::from_xyz(0., 0., 5.),
      Visibility::Visible,
    )).id();
    let eye_looking = world.spawn((
      Sprite {
        image: world.load_asset(sprite_filenames::EYE_LOOKING),
        ..Default::default()
      },
      Transform::from_xyz(0., 0., 5.),
      Visibility::Hidden,
    )).id();

    // Heads
    let head_neutral = world.spawn((
      Sprite {
        image: world.load_asset(sprite_filenames::HEAD_NEUTRAL),
        ..Default::default()
      },
      Transform::from_xyz(0., 0., 4.),
      Visibility::Visible,
    )).id();
    let head_sad = world.spawn((
      Sprite {
        image: world.load_asset(sprite_filenames::HEAD_SAD),
        ..Default::default()
      },
      Transform::from_xyz(0., 0., 4.),
      Visibility::Hidden,
    )).id();
    let head_surprise = world.spawn((
      Sprite {
        image: world.load_asset(sprite_filenames::HEAD_SURPRISE),
        ..Default::default()
      },
      Transform::from_xyz(0., 0., 4.),
      Visibility::Hidden,
    )).id();
    let head_talk = world.spawn((
      Sprite {
        image: world.load_asset(sprite_filenames::HEAD_TALK),
        ..Default::default()
      },
      Transform::from_xyz(0., 0., 4.),
      Visibility::Hidden,
    )).id();

    // L Arms
    let l_arm_hip = world.spawn((
      Sprite {
        image: world.load_asset(sprite_filenames::L_ARM_HIP),
        ..Default::default()
      },
      Transform::from_xyz(0., 0., 3.),
      Visibility::Visible,
    )).id();
    let l_arm_shrug = world.spawn((
      Sprite {
        image: world.load_asset(sprite_filenames::L_ARM_SHRUG),
        ..Default::default()
      },
      Transform::from_xyz(0., 0., 3.),
      Visibility::Hidden,
    )).id();

    // Body
    let body = world.spawn((
      Sprite {
        image: world.load_asset(sprite_filenames::BODY),
        ..Default::default()
      },
      Transform::from_xyz(0., 0., 2.),
      Visibility::Visible,
    )).id();

    // R Arms
    let r_arm_hangin = world.spawn((
      Sprite {
        image: world.load_asset(sprite_filenames::R_ARM_HANGIN),
        ..Default::default()
      },
      Transform::from_xyz(0., 0., 1.),
      Visibility::Visible,
    )).id();
    let r_arm_shrug = world.spawn((
      Sprite {
        image: world.load_asset(sprite_filenames::R_ARM_SHRUG),
        ..Default::default()
      },
      Transform::from_xyz(0., 0., 1.),
      Visibility::Hidden,
    )).id();

    // Tails
    let tail_flip = world.spawn((
      Sprite {
        image: world.load_asset(sprite_filenames::TAIL_1),
        ..Default::default()
      },
      Transform::from_xyz(0., 0., 0.),
      Visibility::Visible,
    )).id();
    let tail_flop = world.spawn((
      Sprite {
        image: world.load_asset(sprite_filenames::TAIL_2),
        ..Default::default()
      },
      Transform::from_xyz(0., 0., 0.),
      Visibility::Hidden,
    )).id();

    // Root Entity
    let mut vincubus = world.spawn((
      Vincubus {
        eye_facing,
        eye_looking,
        head_neutral,
        head_sad,
        head_surprise,
        head_talk,
        l_arm_hip,
        l_arm_shrug,
        _body: body,
        r_arm_hangin,
        r_arm_shrug,
        tail_flip,
        tail_flop,
      },
      EyeState::Facing,
      HeadState::Neutral,
      LArmState::Hip,
      RArmState::Hangin,
      TailState::Flip,
    ));
    vincubus.add_children(&[
      eye_facing,
      eye_looking,
      head_neutral,
      head_sad,
      head_surprise,
      head_sad,
      l_arm_hip,
      l_arm_shrug,
      body,
      r_arm_hangin,
      r_arm_shrug,
      tail_flip,
      tail_flop,
    ]);
  }
}

///////////////////
// Asset Loading //
///////////////////

mod sprite_filenames {
  type S = &'static str;

  pub const TAIL_1:        S = "vincubus/sprites/0_Tail_1.png";
  pub const TAIL_2:        S = "vincubus/sprites/0_Tail_2.png";
  pub const R_ARM_HANGIN:  S = "vincubus/sprites/1_R_Hangin.png";
  pub const R_ARM_SHRUG:   S = "vincubus/sprites/1_R_Shrug.png";
  pub const BODY:          S = "vincubus/sprites/2_Body.png";
  pub const L_ARM_HIP:     S = "vincubus/sprites/3_L_On_Hip.png";
  pub const L_ARM_SHRUG:   S = "vincubus/sprites/3_L_Shrug.png";
  pub const HEAD_NEUTRAL:  S = "vincubus/sprites/4_Head_Neutral.png";
  pub const HEAD_SAD:      S = "vincubus/sprites/4_Head_Sad.png";
  pub const HEAD_SURPRISE: S = "vincubus/sprites/4_Head_Surprise.png";
  pub const HEAD_TALK:     S = "vincubus/sprites/4_Head_Talk.png";
  pub const EYE_FACING:    S = "vincubus/sprites/5_Eye_Facing.png";
  pub const EYE_LOOKING:   S = "vincubus/sprites/5_Eye_Looking.png";
}

#[derive(Resource)]
struct LoadState {
  finished_loading: bool,
  sprites: HashSet<Handle<Image>>,
}

fn update_load_state(
  mut load_state: ResMut<LoadState>,
  asset_server: Res<AssetServer>,
) {
  if load_state.finished_loading {
    return;
  }

  load_state.sprites.retain(|sprite| {
    asset_server
      .get_recursive_dependency_load_state(sprite)
      .is_none_or(|state| !state.is_loaded())
  });

  if load_state.sprites.len() == 0 {
    load_state.finished_loading = true;
  }
}

///////////
// Setup //
///////////

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  ////////////////////////
  // Load Sprite Images //
  ////////////////////////

  let image_tail_1        = asset_server.load(sprite_filenames::TAIL_1);
  let image_tail_2        = asset_server.load(sprite_filenames::TAIL_2);
  let image_r_arm_hangin  = asset_server.load(sprite_filenames::R_ARM_HANGIN);
  let image_r_arm_shrug   = asset_server.load(sprite_filenames::R_ARM_SHRUG);
  let image_body          = asset_server.load(sprite_filenames::BODY);
  let image_l_arm_hip     = asset_server.load(sprite_filenames::L_ARM_HIP);
  let image_l_arm_shrug   = asset_server.load(sprite_filenames::L_ARM_SHRUG);
  let image_head_neutral  = asset_server.load(sprite_filenames::HEAD_NEUTRAL);
  let image_head_sad      = asset_server.load(sprite_filenames::HEAD_SAD);
  let image_head_surprise = asset_server.load(sprite_filenames::HEAD_SURPRISE);
  let image_head_talk     = asset_server.load(sprite_filenames::HEAD_TALK);
  let image_eye_facing    = asset_server.load(sprite_filenames::EYE_FACING);
  let image_eye_looking   = asset_server.load(sprite_filenames::EYE_LOOKING);

  //////////////////////
  // Track Load State //
  //////////////////////

  let load_state = LoadState {
    finished_loading: false,
    sprites: HashSet::from_iter([
      image_tail_1,
      image_tail_2,
      image_r_arm_hangin,
      image_r_arm_shrug,
      image_body,
      image_l_arm_hip,
      image_l_arm_shrug,
      image_head_neutral,
      image_head_sad,
      image_head_surprise,
      image_head_talk,
      image_eye_facing,
      image_eye_looking,
    ].into_iter()),
  };

  commands.insert_resource(load_state);

  //////////////////
  // Finish Setup //
  //////////////////

  commands.queue(SpawnVincubusCommand);
}
