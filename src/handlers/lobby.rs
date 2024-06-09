use super::connections::Connection;
use super::room;
use super::room::GameRoom;
use super::room::RoomId;
use super::room::WaitingRoom;
use crate::game_logic::game_event::GameEvent;
use crate::utils::player::Player;
use futures_util::TryFutureExt;
use log::info;
use log::warn;
use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;
// use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{oneshot, Mutex, RwLock};

pub struct Lobby {
    connections: Arc<Mutex<HashMap<Player, Connection>>>,
    // max_clients: PlayerId_t,
    rooms: Arc<Mutex<std::collections::btree_map::BTreeMap<Player, Arc<RwLock<GameRoom>>>>>,
    rooms_waiting: Arc<Mutex<std::collections::btree_map::BTreeMap<RoomId, Arc<WaitingRoom>>>>,
    /*
    mm_user_pool_tx: mpsc::UnboundedSender<Player>,
    mm_user_pool_rx: mpsc::UnboundedReceiver<Player>,
    */
}

impl Lobby {
    async fn get_connection(&self, player: Player) -> Result<Connection, &str> {
        let c = self.connections.lock().await;
        match c.get(&player).cloned() {
            None => Err("Connection not found"),
            Some(conn) => Ok(conn),
        }
    }
    async fn change_game_event_tx(
        &mut self,
        p: Player,
        tx: Option<tokio::sync::mpsc::UnboundedSender<GameEvent>>,
    ) {
        let mut c = self.connections.lock().await;
        match c.get_mut(&p) {
            None => {
                warn!("Player not found");
            }
            Some(conn) => {
                conn.change_game_event_tx(tx);
            }
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

    async fn get_room(&self, player: Player) -> Result<Arc<RwLock<GameRoom>>, &str> {
        let r = self.rooms.lock().await;
        match r.get(&player).cloned() {
            None => Err("Room not found"),
            Some(room) => Ok(room),
        }
    }

    async fn remove_room(&self, player: Player) -> Result<(), &str> {
        let mut r = self.rooms.lock().await;
        // get player 1 2 in the room
        match r.remove(&player) {
            None => Err("Room not found"),
            Some(room) => {
                let (player1, player2) = room.read().await.players();
                r.remove(&player1);
                r.remove(&player2);
                Ok(())
            }
        }
    }
    async fn on_create_room(&self, player: Player) -> Result<RoomId, &str> {
        let mut r = self.rooms_waiting.lock().await;
        let room = Arc::new(WaitingRoom::new(player));
        match r.insert(room.room_id(), room.clone()) {
            None => Ok(room.room_id()),
            Some(_) => Err("Room already exists"),
        }
    }
    async fn on_join_room(
        &mut self,
        room_id: RoomId,
        new_player: Player,
    ) -> Result<Arc<RwLock<GameRoom>>, &str> {
        // 有新的玩家输入room_id并连接，则建立新房间。
        let mut r = self.rooms_waiting.lock().await;
        let w_room = r.remove(&room_id);
        drop(r);
        let (game_event_tx, game_event_rx) = mpsc::unbounded_channel();
        match w_room {
            None => Err("Room not found"),
            Some(w_room) => {
                let room_player = w_room.player();
                if let Ok(room_conn) = self.get_connection(room_player).await {
                    if let Ok(p2_conn) = self.get_connection(new_player).await {
                        let new_room = Arc::new(RwLock::new(GameRoom::from_waiting_room(
                            w_room.as_ref(),
                            new_player,
                            room_conn,
                            p2_conn,
                            game_event_rx,
                        )));
                        {
                            let mut r = self.rooms.lock().await;
                            r.insert(room_player, new_room.clone());
                            r.insert(new_player, new_room.clone());
                        }
                        // add tx to connections
                        self.change_game_event_tx(room_player, Some(game_event_tx.clone()))
                            .await;
                        self.change_game_event_tx(new_player, Some(game_event_tx))
                            .await;
                        info!("game_event_mq set!");
                        return Ok(new_room);
                    }
                }
                Err("Error when creating room: cannot find connection")
            }
        }
    }
    async fn add_connection(&self, player: Player, conn: Connection) -> Result<(), &str> {
        let mut c = self.connections.lock().await;
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
                    let Ok(_) = lobby.add_connection(player, conn).await else {
                        continue;
                    };
                    tx.send(player).unwrap();
                    match lobby.get_connection(player).await {
                        Ok(mut conn) => {
                            conn.send_handshake_success(player).await;
                        }
                        Err(e) => {
                            warn!("{}", e);
                        }
                    }
                }
                LobbyClientEvent::CreateRoom(player) => {
                    let Ok(roomid) = lobby.on_create_room(player).await else {
                        match lobby.get_connection(player).await {
                            Ok(mut conn) => {
                                conn.send_room_create_fail().await;
                            }
                            Err(e) => {
                                warn!("{}", e);
                            }
                        }
                        continue;
                    };
                    //告知
                    match lobby.get_connection(player).await {
                        Ok(mut conn) => {
                            conn.send_room_create_success(roomid).await;
                        }
                        Err(e) => {
                            warn!("{}", e);
                        }
                    }
                }
                LobbyClientEvent::JoinRoom(player, room_id) => {
                    match lobby.on_join_room(room_id, player).await {
                        Ok(new_room) => {
                            // 告知房主
                            let (p1, p2) =
                                lobby.get_room(player).await.unwrap().read().await.players();
                            let player_to_notify = if player == p1 { p2 } else { p1 };
                            match (
                                lobby.get_connection(player_to_notify).await,
                                lobby.get_connection(player).await,
                            ) {
                                (Ok(mut conn_owner), Ok(mut conn_joiner)) => {
                                    tokio::join!(
                                        conn_owner.send_opponent_join(),
                                        conn_joiner.send_join_room_success(room_id)
                                    );
                                }
                                _ => {
                                    warn!("can't find connections");
                                }
                            }
                            tokio::spawn(async move {
                                new_room.write().await.listen_game().await;
                            });
                        }
                        Err(e) => match lobby.get_connection(player).await {
                            Ok(mut conn) => {
                                conn.send_join_room_fail().await;
                            }
                            Err(e) => {
                                warn!("{}", e);
                            }
                        },
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
                    if let Ok(room) = lobby.get_room(player_leaving).await {
                        let (p1, p2) = room.read().await.players();
                        lobby.remove_room(p1).await;
                        lobby.remove_room(p2).await;
                        let player_to_notify = if player_leaving == p1 {
                            p2.clone()
                        } else {
                            p1.clone()
                        };
                        match lobby.get_connection(player_to_notify).await {
                            Ok(mut conn) => {
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
    CreateRoom(Player),
    JoinRoom(Player, RoomId),
    LeaveRoom(Player),
    /*
    MatchMake(Player),
    */
}
impl std::fmt::Display for LobbyClientEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LobbyClientEvent::HandShake(player, _, _) => write!(f, "HandShake({})", player),
            LobbyClientEvent::CreateRoom(player) => write!(f, "CreateRoom({})", player),
            LobbyClientEvent::JoinRoom(player, room_id) => {
                write!(f, "JoinRoom({}, {})", player, room_id)
            }
            LobbyClientEvent::LeaveRoom(player) => write!(f, "LeaveRoom({})", player),
            /*
            LobbyClientEvent::MatchMake(player) => write!(f, "MatchMake({})", player),
            */
        }
    }
}

pub enum LobbyServerEvent {
    OpponentJoin(RoomId, Player),
    OpponentLeave(RoomId, Player),
    CreateRoomSuccess(RoomId),
    CreateRoomFail,
    JoinRoomSuccess(RoomId),
    JoinRoomFail(RoomId),
}
