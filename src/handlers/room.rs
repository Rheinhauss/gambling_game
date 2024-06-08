use std::sync::atomic::{AtomicU32, Ordering};

use crate::game_logic::game_event::GameEvent;
use crate::game_logic::game_state::{GameState, Stage};
use crate::utils::player::{GameItem, Player};

use super::connections::Connection;

pub type RoomId = u32;
pub struct GameRoom {
    host_player: (Player, Connection),
    guest_player: (Player, Connection),
    room_id: u32,
    started: bool,
    state: GameState,
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
        let p1 = host_player.0.clone();
        let p2 = guest_player.0.clone();
        GameRoom {
            host_player: host_player,
            guest_player: guest_player,
            room_id: Self::get_new_id(),
            started: false,
            state: GameState::new(p1, p2),
            game_events_rx: rx,
        }
    }
    pub fn from_waiting_room(
        room: &WaitingRoom,
        player2: Player,
        room_conn: Connection,
        p2_conn: Connection,
        rx: tokio::sync::mpsc::UnboundedReceiver<GameEvent>,
    ) -> Self {
        GameRoom {
            host_player: (room.player(), room_conn),
            guest_player: (player2, p2_conn),
            room_id: room.room_id(),
            started: false,
            state: GameState::new(room.player(), player2),
            game_events_rx: rx,
        }
    }

    pub fn players(&self) -> (Player, Player) {
        (self.host_player.0, self.guest_player.0)
    }

    pub fn opponent(&self, player: &Player) -> Player {
        if *player == self.host_player.0 { self.host_player.0 } else { self.guest_player.0 }
    }

    fn after_item_used(&mut self){
        if let Some(open_state) = self.state.open_state(self.host_player.0){
            self.host_player.1.send_use_item(open_state);
        }
        if let Some(open_state) = self.state.open_state(self.guest_player.0){
            self.guest_player.1.send_use_item(open_state);
        }
    }

    fn after_player_leave(&mut self, player: Player){
        if player == self.host_player.0 {
            self.guest_player.1.send_opponent_leave();
        } else {
            self.host_player.1.send_opponent_leave();
        }
    }

    fn send_items(&mut self, player: Player, items: [GameItem; 3]) {
        if player == self.host_player.0 {
            self.host_player.1.send_draw_item_pool(self.state.open_state(player).unwrap(), Vec::from(items));
        } else {
            self.guest_player.1.send_draw_item_pool(self.state.open_state(player).unwrap(), Vec::from(items));
        }
    }

    fn after_round_started(&mut self){
        // TODO: I gave up and use unwrap() at the end.
        self.host_player.1.send_new_round(self.state.open_state(self.host_player.0).unwrap(), self.state.hidden_state().unwrap());
        self.guest_player.1.send_new_round(self.state.open_state(self.guest_player.0).unwrap(), self.state.hidden_state().unwrap());
    }

    fn after_game_over(&mut self, winner: Player){
        self.host_player.1.send_game_end(self.host_player.0 == winner);
        self.guest_player.1.send_game_end(self.guest_player.0 == winner);
    }

    pub async fn listen_game(&mut self) {
        while let Some(event) = self.game_events_rx.recv().await {
            // receive event from receiver (msg from client)
            match event {
                GameEvent::UseItem(player, item) => {
                    self.state.use_item(player, item);
                    self.after_item_used();
                }
                GameEvent::DrawItem(player, item) => {
                    self.state.draw_item(player, item);
                }
                GameEvent::Shoot(player, is_hit) => {
                    self.state.shoot(player, is_hit);
                }
                GameEvent::Leave(player) => {
                    self.state.player_leave(player);
                    self.after_player_leave(player);
                }
            }
            
            // send response to connection (msg to client)
            match self.state.current_stage(){
                Stage::GameOver(winner) => {
                    self.after_game_over(winner);
                    break;
                }
                Stage::Act(player) => {
                    continue;
                }
                Stage::SendItem(player, items) => {
                    self.state.item_sended();
                    self.send_items(player, items);
                }
                Stage::RoundStart => {
                    self.state.start_round();
                    self.after_round_started();
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
