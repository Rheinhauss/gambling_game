use std::sync::atomic::{AtomicU32, Ordering};

use crate::game_logic::game_event::GameEvent;
use crate::game_logic::game_state::{GameState, Stage};
use crate::utils::player::Player;

use super::connections::Connection;

pub type RoomId = u32;
pub struct GameRoom {
    host_player: (Player, Connection),
    guest_player: (Player, Connection),
    room_id: u32,
    started: bool,
    state: Option<GameState>,
    game_events_rx: tokio::sync::mpsc::UnboundedReceiver<GameEvent>,
}
static ROOMGEN: AtomicU32 = AtomicU32::new(0);
impl GameRoom {
    fn get_new_id() -> RoomId {
        ROOMGEN.fetch_add(1, Ordering::SeqCst)
    }
    pub fn new(
        host_player: (Player, Connection),
        guest_player: (Player, Connection),
        rx: tokio::sync::mpsc::UnboundedReceiver<GameEvent>,
    ) -> Self {
        GameRoom {
            host_player: host_player,
            guest_player: guest_player,
            room_id: Self::get_new_id(),
            started: false,
            state: None, // todo
            game_events_rx: rx,
        }
    }
    pub fn from_waiting_room(
        room: &WaitingRoom,
        player2: Player,
        rx: tokio::sync::mpsc::UnboundedReceiver<GameEvent>,
    ) -> Self {
        GameRoom {
            host_player: ,
            guest_player: ,
            players: (room.player(), player2),
            room_id: room.room_id(),
            started: false,
            state: None,
            game_events_rx: rx,
        }
    }

    pub fn players(&self) -> (Player, Player) {
        self.players
    }

    pub fn opponent(&self, player: &Player) -> Player {
        let (p1, p2) = self.players;
        if p1 == *player { p2 } else { p1 }
    }

    pub async fn listen_game(&mut self) {
        while let Some(event) = self.game_events_rx.recv().await {
            match event {
                GameEvent::UseItem(player, item) => {
                    // todo
                }
                GameEvent::DrawItem(player, item) => {
                    // todo
                }
                GameEvent::Shoot(player, is_hit) => {
                    // todo
                }
                GameEvent::Leave(player) => {
                    // todo
                }
            }
        }
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
