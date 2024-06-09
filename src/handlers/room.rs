﻿use std::sync::atomic::{AtomicU32, Ordering};

use crate::game_logic::game_event::GameEvent;
use crate::game_logic::game_state::{GameState, Stage};
use crate::info;
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
    fn new(
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
        if *player == self.host_player.0 {
            self.host_player.0
        } else {
            self.guest_player.0
        }
    }

    async fn after_item_used(&mut self) {
        if let Some(open_state) = self.state.open_state(self.host_player.0) {
            self.host_player.1.send_use_item(&open_state).await;
        }
        if let Some(open_state) = self.state.open_state(self.guest_player.0) {
            self.guest_player.1.send_use_item(&open_state).await;
        }
    }

    async fn after_draw_item(&mut self) {
        if let Some(open_state) = self.state.open_state(self.host_player.0) {
            self.host_player.1.send_drawed_item(&open_state).await;
        }
        if let Some(open_state) = self.state.open_state(self.guest_player.0) {
            self.guest_player.1.send_drawed_item(&open_state).await;
        }
    }

    async fn after_player_leave(&mut self, player: Player) {
        if player == self.host_player.0 {
            self.guest_player.1.send_opponent_leave().await;
        } else {
            self.host_player.1.send_opponent_leave().await;
        }
    }

    async fn send_items(&mut self, player: Player, items: [GameItem; 3]) {
        if player == self.host_player.0 {
            let open_state = self.state.open_state(player).unwrap();
            self.host_player.1.send_new_turn(&open_state).await;
            self.host_player
                .1
                .send_draw_item_pool(&open_state, Vec::from(items))
                .await;
        } else {
            let open_state = self.state.open_state(player).unwrap();
            self.guest_player.1.send_new_turn(&open_state).await;
            self.guest_player
                .1
                .send_draw_item_pool(&open_state, Vec::from(items))
                .await;
        }
    }

    async fn after_round_started(&mut self) {
        // TODO: I gave up and use unwrap() at the end.
        info!("host player is:{}", self.host_player.0);
        info!("guest player is:{}", self.guest_player.0);
        self.host_player
            .1
            .send_new_round(
                self.state.open_state(self.host_player.0).as_ref().unwrap(),
                self.state.hidden_state().as_ref().unwrap(),
            )
            .await;
        self.guest_player
            .1
            .send_new_round(
                self.state.open_state(self.guest_player.0).as_ref().unwrap(),
                self.state.hidden_state().as_ref().unwrap(),
            )
            .await;
    }

    async fn after_game_over(&mut self, winner: Player) {
        self.host_player
            .1
            .send_game_end(self.host_player.0 == winner)
            .await;
        self.guest_player
            .1
            .send_game_end(self.guest_player.0 == winner)
            .await;
    }

    pub async fn listen_game(&mut self) {
        info!("start listening game of room [{}]", self.room_id);
        self.state.start_game();
        self.state.start_round();
        self.after_round_started().await;
        if let Stage::SendItem(pl, items) = self.state.current_stage() {
            self.send_items(pl, items).await;
            self.state.item_sended();
        } else {
            panic!("fuck up!")
        }
        info!("game event loop begin");
        while let Some(event) = self.game_events_rx.recv().await {
            // receive event from receiver (msg from client)
            info!("stage: [{}] <{}>", self.state.current_stage(), event);
            match event {
                GameEvent::UseItem(player, item_idx) => {
                    self.state.use_item(player, item_idx);
                    self.after_item_used().await;
                }
                GameEvent::DrawItem(player, item) => {
                    self.state.draw_item(player, item);
                    self.after_draw_item().await;
                }
                GameEvent::Shoot(player, shoot_self) => {
                    self.state.shoot(player, shoot_self);
                }
                GameEvent::Leave(player) => {
                    self.state.player_leave(player);
                    self.after_player_leave(player).await;
                }
            }

            // send response to connection (msg to client)
            info!("after stage: [{}]", self.state.current_stage());
            match self.state.current_stage() {
                Stage::GameOver(winner) => {
                    self.after_game_over(winner).await;
                    break;
                }
                Stage::Act(player) => {
                    continue;
                }
                Stage::SendItem(player, items) => {
                    self.state.item_sended();
                    self.send_items(player, items).await;
                }
                Stage::RoundStart => {
                    self.state.start_round();
                    self.after_round_started().await;
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