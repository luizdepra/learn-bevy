mod systems;

mod enemy;
mod events;
mod player;
mod score;
mod star;

use bevy::prelude::*;

use crate::enemy::EnemyPlugin;
use crate::events::*;
use crate::player::PlayerPlugin;
use crate::score::ScorePlugin;
use crate::star::StarPlugin;
use crate::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run()
}
