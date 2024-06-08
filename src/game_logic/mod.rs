pub mod game_state;
pub mod game_event {
use crate::utils::player::*;


    pub enum GameEvent {
        UseItem(Player, GameItem),
        DrawItem(Player, Option<GameItem>),
        Shoot(Player, bool),
        Leave(Player),
    }
}
