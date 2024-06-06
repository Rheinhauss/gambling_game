use std::sync::atomic::{AtomicU32, Ordering};

use crate::game_logic::game_logic::GameState;
use crate::utils::player::Player;

pub type RoomId = u32;
pub struct GameRoom {
    players: (Player, Player),
    room_id: u32,
    started: bool,
    state: Option<GameState>,
}
static ROOMGEN: AtomicU32 = AtomicU32::new(0);
impl GameRoom {
    fn get_new_id() -> RoomId {
        ROOMGEN.fetch_add(1, Ordering::SeqCst)
    }
    pub fn new(player1: Player, player2: Player) -> Self {
        GameRoom {
            players: (player1, player2),
            room_id: Self::get_new_id(),
            started: false,
            state: None,
        }
    }
    pub fn from_waiting_room(room :&WaitingRoom, player2: Player) -> Self {
        GameRoom {
            players: (room.player(), player2),
            room_id: room.room_id(),
            started: false,
            state: None,
        }
    }
    pub fn players(&self) -> (Player, Player) {
        self.players
    }
}
pub struct WaitingRoom {
    player: Player,
    room_id: RoomId,
}
impl WaitingRoom {
    pub fn new(player: Player) -> Self {
        WaitingRoom {
            player,
            room_id: GameRoom::get_new_id(),
        }
    }
    pub fn player(&self) -> Player {
        self.player
    }
    pub fn room_id(&self) -> RoomId {
        self.room_id
    }
}
