pub mod game_state;
pub mod game_event {
    use crate::utils::player::Player;

    use super::game_state::GameItem;

    pub enum GameEvent {
        UseItem(Player, GameItem),
        DrawItem(Player, GameItem),
        Shoot(Player, bool),
        Leave(Player),
    }
}
