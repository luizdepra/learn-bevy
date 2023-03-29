pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
pub mod systems;

use bevy::prelude::*;

use crate::events::GameOver;
use crate::AppState;

use self::enemy::EnemyPlugin;
use self::player::PlayerPlugin;
use self::score::ScorePlugin;
use self::star::StarPlugin;
use self::systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
