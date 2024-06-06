use super::connections::Connection;
use super::room::GameRoom;
use super::room::RoomId;
use super::room::WaitingRoom;
use crate::game_logic::game_event::GameEvent;
use crate::utils::player::Player;
use crate::utils::player::PlayerId_t;
use log::info;
use log::warn;
use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;
use std::future;
use std::process::Output;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;

pub struct Lobby {
    connections: Arc<Mutex<HashMap<Player, Connection>>>,
    // max_clients: PlayerId_t,
    rooms: Arc<Mutex<std::collections::btree_map::BTreeMap<Player, Arc<GameRoom>>>>,
    rooms_waiting: Arc<Mutex<std::collections::btree_map::BTreeMap<RoomId, Arc<WaitingRoom>>>>,
    /*
    mm_user_pool_tx: mpsc::UnboundedSender<Player>,
    mm_user_pool_rx: mpsc::UnboundedReceiver<Player>,
    */
}

impl Lobby {
    fn get_connection(&self, player: Player) -> Result<Connection, &str> {
        let c = self.connections.lock().unwrap();
        match c.get(&player).cloned() {
            None => Err("Connection not found"),
            Some(conn) => Ok(conn),
        }
    }
    pub fn new() -> Self {
        /*
        let (mm_user_pool_tx, mm_user_pool_rx) = mpsc::unbounded_channel();
        */
        info!("Lobby created.");
        Lobby {
            connections: Arc::new(Mutex::new(HashMap::new())),
            // max_clients: 65535,
            rooms: Arc::new(Mutex::new(BTreeMap::new())),
            rooms_waiting: Arc::new(Mutex::new(BTreeMap::new())),
            /*
            mm_user_pool_tx,
            mm_user_pool_rx,
            */
        }
    }
    /*
    pub async fn matchmaking(&mut self) {
        loop {
            // matchmake: 2 players are matched.
            let player1 = self.mm_user_pool_rx.recv().await.unwrap();
            let player2 = self.mm_user_pool_rx.recv().await.unwrap();
            let room = Arc::new(GameRoom::new(player1, player2));
            let mut r = self.rooms.lock().unwrap();
            r.insert(player1, room.clone());
            r.insert(player2, room.clone());
        }
    }
    */

    fn get_room(&self, player: Player) -> Result<Arc<GameRoom>, &str> {
        let r = self.rooms.lock().unwrap();
        match r.get(&player).cloned() {
            None => Err("Room not found"),
            Some(room) => Ok(room),
        }
    }

    fn remove_room(&self, player: Player) -> Result<(), &str> {
        let mut r = self.rooms.lock().unwrap();
        // get player 1 2 in the room
        match r.remove(&player) {
            None => Err("Room not found"),
            Some(room) => {
                let (player1, player2) = room.players();
                r.remove(&player1);
                r.remove(&player2);
                Ok(())
            }
        }
    }
    fn on_create_room(&self, player: Player) -> Result<RoomId, &str> {
        let mut r = self.rooms_waiting.lock().unwrap();
        let room = Arc::new(WaitingRoom::new(player));
        match r.insert(room.room_id(), room.clone()) {
            None => Ok(room.room_id()),
            Some(_) => Err("Room already exists"),
        }
    }
    fn on_join_room(&self, room_id: RoomId, new_player: Player) -> Result<(), &str> {
        // 有新的玩家输入room_id并连接，则建立新房间。
        let mut r = self.rooms_waiting.lock().unwrap();
        let w_room = r.remove(&room_id);
        drop(r);
        match w_room {
            None => Err("Room not found"),
            Some(w_room) => {
                let player1 = w_room.player();
                let new_room = Arc::new(GameRoom::from_waiting_room(w_room.as_ref(), new_player));
                let mut r = self.rooms.lock().unwrap();
                r.insert(player1, new_room.clone());
                r.insert(new_player, new_room.clone());
                Ok(())
            }
        }
    }
    fn add_connection(&self, player: Player, conn: Connection) -> Result<(), &str> {
        let mut c = self.connections.lock().unwrap();
        match c.insert(player, conn) {
            None => Ok(()),
            Some(_) => Err("Player already connected"),
        }
    }

    pub async fn listen(
        lobby_client_mq_tx: UnboundedSender<LobbyClientEvent>,
        mut lobby_client_mq_rx: UnboundedReceiver<LobbyClientEvent>,
    ) {
        let mut lobby = Self::new();
        while let Some(event) = lobby_client_mq_rx.recv().await {
            match event {
                LobbyClientEvent::HandShake(player, conn, tx) => {
                    let Ok(_) = lobby.add_connection(player, conn) else {
                        continue;
                    };
                    tx.send(player).unwrap();
                    match lobby.get_connection(player) {
                        Ok(conn) => {
                            conn.send_handshake_success(player).await;
                        }
                        Err(e) => {
                            warn!("{}", e);
                        }
                    }
                }
                LobbyClientEvent::CreateRoom(player, sender) => {
                    let Ok(roomid) = lobby.on_create_room(player) else {
                        match lobby.get_connection(player) {
                            Ok(conn) => {
                                conn.send_room_create_fail().await;
                            }
                            Err(e) => {
                                warn!("{}", e);
                            }
                        }
                        continue;
                    };
                    //告知
                    match lobby.get_connection(player) {
                        Ok(conn) => {
                            conn.send_room_create_success(roomid).await;
                        }
                        Err(e) => {
                            warn!("{}", e);
                        }
                    }
                }
                LobbyClientEvent::JoinRoom(player, room_id) => {
                    let Ok(_) = lobby.on_join_room(room_id, player) else {
                        match lobby.get_connection(player) {
                            Ok(conn) => {
                                conn.send_join_room_fail().await;
                            }
                            Err(e) => {
                                warn!("{}", e);
                            }
                        }
                        continue;
                    };
                    // 告知房主
                    let (p1, p2) = lobby.get_room(player).unwrap().players();
                    let player_to_notify = if player == p1 { p2 } else { p1 };
                    match (
                        lobby.get_connection(player_to_notify),
                        lobby.get_connection(player),
                    ) {
                        (Ok(conn_owner), Ok(conn_joiner)) => {
                            tokio::join!(
                                conn_owner.send_opponent_join(),
                                conn_joiner.send_join_room_success(room_id)
                            );
                        }
                        _ => {
                            warn!("can't find connections");
                        }
                    }
                }
                /*
                LobbyClientEvent::MatchMake(player) => {
                    todo!("matchmaking");
                    let Ok(_) = lobby.mm_user_pool_tx.send(player) else {
                        continue;
                    };
                }
                */
                LobbyClientEvent::LeaveRoom(player_leaving) => {
                    if let Ok(room) = lobby.get_room(player_leaving) {
                        let (p1, p2) = room.players();
                        lobby.remove_room(p1).unwrap();
                        lobby.remove_room(p2).unwrap();
                        let player_to_notify = if player_leaving == p1 { p2 } else { p1 };
                        match lobby.get_connection(player_to_notify) {
                            Ok(conn) => {
                                conn.send_opponent_leave().await;
                            }
                            Err(e) => {
                                warn!("{}", e);
                                continue;
                            }
                        }
                    } else {
                        warn!("LobbyClientEvent::LeaveRoom, Room not found");
                        continue;
                    };
                }
            }
        }
    }
}

pub enum LobbyClientEvent {
    HandShake(Player, Connection, oneshot::Sender<Player>),
    CreateRoom(Player, Sender<UnboundedSender<GameEvent>>),
    JoinRoom(Player, RoomId),
    LeaveRoom(Player),
    /*
    MatchMake(Player),
    */
}

pub enum LobbyServerEvent {
    OpponentJoin(RoomId, Player),
    OpponentLeave(RoomId, Player),
    CreateRoomSuccess(RoomId),
    CreateRoomFail,
    JoinRoomSuccess(RoomId),
    JoinRoomFail(RoomId),
}
