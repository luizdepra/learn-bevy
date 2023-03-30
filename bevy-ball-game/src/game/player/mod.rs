mod components;
mod systems;

use bevy::prelude::*;

use crate::game::SimulationState;
use crate::AppState;

use self::systems::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    player_movement.in_set(PlayerSystemSet::Movement),
                    confine_player_movement.in_set(PlayerSystemSet::Confinement),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_systems(
                (enemy_hit_player, player_hit_star)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
