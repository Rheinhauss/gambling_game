pub mod game_state;
pub mod game_event {
    use crate::utils::player::*;

    #[derive(Debug, Clone, Copy)]
    pub enum GameEvent {
        UseItem(Player, GameItem),
        DrawItem(Player, Option<GameItem>),
        Shoot(Player, bool),
        Leave(Player),
    }
    impl std::fmt::Display for GameEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                GameEvent::UseItem(player, item) => write!(f, "UseItem({}, {})", player, item),
                GameEvent::DrawItem(player, item) => write!(f, "DrawItem({}, {:?})", player, item),
                GameEvent::Shoot(player, is_hit) => write!(f, "Shoot({}, {})", player, is_hit),
                GameEvent::Leave(player) => write!(f, "Leave({})", player),
            }
        }
    }
}
