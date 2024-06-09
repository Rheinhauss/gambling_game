use crate::game_logic::game_event::GameItemIndex;
use crate::game_logic::game_state::{GameStateHidden, GameStateOpen};
use crate::{game_logic::game_event::GameEvent, utils::player::*};
use futures_util::SinkExt;
use log::{debug, error, info, trace, warn};
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use tokio_tungstenite::{tungstenite, WebSocketStream};
use tungstenite::Message;

use super::lobby::{Lobby, LobbyClientEvent};

use futures_util::stream::StreamExt;
use futures_util::stream::{SplitSink, SplitStream};
use serde_json::{json, Value};
use std::io::Seek;
use std::net::SocketAddr;
use std::sync::Arc;

enum WsRxMsgType {
    // lobby
    CreateRoom,
    JoinRoom,
    Handshake,
    LeaveRoom,
    // game
    UseItem,
    DrawItem,
    Shoot,
}
enum WsTxMsgType {
    //lobby
    OpponentJoin,
    OpponentLeave,
    CreateRoomSuccess,
    CreateRoomFail,
    HandshakeSuccess,
    JoinRoomSuccess,
    JoinRoomFail,
    //game
    NewRound,
    NewTurn,
    UseItem,
}
enum WsMsgClass {
    Lobby,
    Game,
}

#[derive(Debug)]
pub struct Connection {
    pub id: Player,
    pub sender: Arc<tokio::sync::Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    // pub sender: SplitSink<WebSocketStream<TcpStream>, Message>,
    pub game_event_tx: Option<tokio::sync::mpsc::UnboundedSender<GameEvent>>,
}

impl Connection {
    pub fn new(
        id: Player,
        sender: Arc<tokio::sync::Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
        // sender: SplitSink<WebSocketStream<TcpStream>, Message>,
    ) -> Self {
        Connection {
            id,
            sender,
            game_event_tx: None,
        }
    }

    pub fn change_game_event_tx(
        &mut self,
        tx: Option<tokio::sync::mpsc::UnboundedSender<GameEvent>>,
    ) {
        self.game_event_tx = tx;
    }

    fn check_rx_msg_class(msg: &serde_json::Value) -> Option<WsMsgClass> {
        let class = msg.get("class")?.as_str()?;
        match class {
            "lobby" => Some(WsMsgClass::Lobby),
            "game" => Some(WsMsgClass::Game),
            _ => None,
        }
    }
    fn check_rx_msg_type(msg: &serde_json::Value) -> Option<WsRxMsgType> {
        let type_ = msg.get("type")?.as_str()?;
        match type_.to_lowercase().as_str() {
            "createroom" => Some(WsRxMsgType::CreateRoom),
            "joinroom" => Some(WsRxMsgType::JoinRoom),
            "handshake" => Some(WsRxMsgType::Handshake),
            "leaveroom" => Some(WsRxMsgType::LeaveRoom),
            "useitem" => Some(WsRxMsgType::UseItem),
            "drawitem" => Some(WsRxMsgType::DrawItem),
            "shoot" => Some(WsRxMsgType::Shoot),
            _ => None,
        }
    }
    fn check_rx_msg(msg: &serde_json::Value) -> Option<(WsMsgClass, WsRxMsgType)> {
        let class = Self::check_rx_msg_class(msg)?;
        let type_ = Self::check_rx_msg_type(msg)?;
        Some((class, type_))
    }

    pub async fn handshake(
        ws_stream: WebSocketStream<TcpStream>,
        lobby_client_mq_tx: UnboundedSender<LobbyClientEvent>,
        peer: SocketAddr,
        lobby: Arc<tokio::sync::Mutex<Lobby>>,
    ) {
        let (ws_tx, mut ws_rx) = ws_stream.split();
        let ws_tx = Arc::from(tokio::sync::Mutex::from(ws_tx));
        info!("handshake: {}", peer);
        let (tx, rx) = oneshot::channel::<Player>();
        // let (tx1, mut rx1) = oneshot::channel::<Arc<Connection>>();
        if let Some(msg) = ws_rx.next().await {
            if let Ok(msg) = msg {
                debug!("{}", &msg);
                if !msg.is_text() || msg.len() > 200 {
                    return;
                }
                let msg = msg.into_data();
                if let Some(j) = serde_json::from_slice::<Value>(&msg).ok() {
                    match Self::check_rx_msg(&j) {
                        Some((WsMsgClass::Lobby, WsRxMsgType::Handshake)) => {
                            let player = Player::new();
                            let conn = Connection::new(player, ws_tx);
                            lobby_client_mq_tx
                                .send(LobbyClientEvent::HandShake(player, conn.clone(), tx))
                                .unwrap();
                            // tx1.send(Arc::new(conn)).unwrap();
                        }
                        _ => {
                            warn!("[handshake] invalid handshake message 1!");
                            return;
                        }
                    }
                }
            } else {
                warn!("[handshake] invalid handshake message 2!");
                return;
            }
        } else {
            warn!("[handshake] invalid handshake message 3!");
        }
        warn!("fuck u");

        if let Ok(player) = rx.await {
            info!("Handshake success! player id [{}]", player.id());
            tokio::spawn(Self::listen_for_player(
                player,
                ws_rx,
                lobby_client_mq_tx,
                lobby,
            ));
        } else {
            warn!("Handshake refused!");
        }
    }

    async fn listen_for_player(
        player: Player,
        mut ws_rx: SplitStream<WebSocketStream<TcpStream>>,
        to_lobby: UnboundedSender<LobbyClientEvent>,
        lobby: Arc<tokio::sync::Mutex<Lobby>>,
    ) {
        info!("[listen_lobby_for_player] listening player id [{}]", player);
        while let Some(msg) = ws_rx.next().await {
            if let Ok(msg) = msg {
                if msg.is_text() {
                    info!("pl [{}] incoming msg : {}", player, msg);
                    let msg = serde_json::from_slice::<Value>(&msg.into_data())
                        .or(Err("Invalid json {}"))
                        .unwrap(); // fixme
                    match Self::check_rx_msg_class(&msg) {
                        Some(WsMsgClass::Lobby) => match Self::parse_receive_lobby(player, msg) {
                            Ok(event) => {
                                info!("player [{}] lobby event: {}", player, event);
                                to_lobby.send(event).unwrap();
                            }
                            Err(e) => {
                                info!("player [{}] invalid lobby event: {}", player, e);
                            }
                        },
                        Some(WsMsgClass::Game) => match Self::parse_receive_game(player, msg) {
                            Ok(event) => {
                                info!("player [{}] game event: {}", player, event);
                                lobby
                                    .lock()
                                    .await
                                    .get_connection(player)
                                    .await
                                    .unwrap()
                                    .game_event_tx
                                    .as_ref()
                                    .ok_or("None tx")
                                    .unwrap()
                                    .send(event)
                                    .unwrap();
                            }
                            Err(e) => {
                                info!("player [{}] invalid game event: {}", player, e);
                            }
                        },
                        None => {
                            warn!("player [{}] invalid msg class", player);
                        }
                    }
                } else if msg.is_close() {
                    info!("player [{}] close!", player);
                    break;
                }
            } else {
                break;
            }
        }
        to_lobby.send(LobbyClientEvent::LeaveRoom(player)).unwrap();
    }

    pub fn parse_receive_lobby(
        player: Player,
        msg: Value,
    ) -> Result<LobbyClientEvent, &'static str> {
        let (msg_class, msg_type) = Self::check_rx_msg(&msg).ok_or("Invalid json {}")?;
        if let WsMsgClass::Game = msg_class {
            return Err("Invalid msg class {}");
        }
        match msg_type {
            WsRxMsgType::CreateRoom => Ok(LobbyClientEvent::CreateRoom(player)),
            WsRxMsgType::JoinRoom => {
                let room_id = msg
                    .get("roomid")
                    .ok_or("Invalid json")?
                    .as_str()
                    .ok_or("Invalid json")?
                    .parse()
                    .or(Err("Invalid json"))?;
                Ok(LobbyClientEvent::JoinRoom(player, room_id))
            }
            WsRxMsgType::LeaveRoom => Ok(LobbyClientEvent::LeaveRoom(player)),

            /*
            WsRxMsgType::MatchMake => {
                todo!("parse_receive_lobby: header is LobbyClientEvent::MatchMake");
                Ok((None, LobbyClientEvent::MatchMake(player)))
            }
            */
            // no handshake here !!!
            _ => Err("Undefined lobby msg type"),
        }
    }

    pub fn parse_receive_game(player: Player, msg: Value) -> Result<GameEvent, &'static str> {
        let (msg_class, msg_type) = Self::check_rx_msg(&msg).ok_or("Invalid json {}")?;
        if let WsMsgClass::Lobby = msg_class {
            return Err("Invalid msg class {}");
        }
        // use serde Deserialize to convert json value to enum GameItem/Bullet
        match msg_type {
            WsRxMsgType::UseItem => {
                let item = msg.get("use").ok_or("Invalid json 1")?.clone();
                let item =
                    serde_json::from_value::<GameItemIndex>(item).or(Err("Invalid json 2"))?;
                Ok(GameEvent::UseItem(player, item))
            }
            WsRxMsgType::DrawItem => {
                let item = msg.get("draw").ok_or("Invalid json 1")?.clone();
                let item = serde_json::from_value::<GameItem>(item).or(Err("Invalid json 2"))?;
                Ok(GameEvent::DrawItem(player, Some(item)))
            }
            WsRxMsgType::Shoot => {
                let shoot = msg.get("shoot").ok_or("Invalid json 1")?.clone();
                let shoot = serde_json::from_value::<bool>(shoot).or(Err("Invalid json 2"))?;
                Ok(GameEvent::Shoot(player, shoot))
            }
            _ => {
                warn!("{}", msg);
                Err("Undefined game msg type")
            }
        }
    }
    // aync fn listen_game

    async fn send_msg(&mut self, msg: serde_json::Value) {
        debug!("send_msg: {}", msg.to_string());
        self.sender
            .lock()
            .await
            .send(Message::text(msg.to_string()))
            .await
            .unwrap();
    }
    // lobby events
    pub async fn send_opponent_leave(&mut self) {
        self.send_msg(json!({
            "class": "lobby",
            "type": "OpponentLeave"
        }))
        .await;
    }
    pub async fn send_opponent_join(&mut self) {
        self.send_msg(json!({
            "class": "lobby",
            "type": "OpponentJoin"
        }))
        .await;
    }
    pub async fn send_room_create_success(&mut self, room_id: u32) {
        self.send_msg(json!({
            "class": "lobby",
            "type": "CreateRoomSuccess",
            "roomid": room_id.to_string()
        }))
        .await;
    }
    pub async fn send_room_create_fail(&mut self) {
        self.send_msg(json!({
                "class": "lobby",
                "type": "CreateRoomFail"
        }))
        .await;
    }
    pub async fn send_handshake_success(&mut self, player: Player) {
        self.send_msg(json!({
            "class": "lobby",
            "type": "HandShakeSuccess",
            "playerid": player.id().to_string()
        }))
        .await;
    }
    pub async fn send_handshake_fail(&mut self) {
        self.send_msg(json!({
            "class": "lobby",
            "type": "HandShakeFail"
        }))
        .await;
    }
    pub async fn send_join_room_success(&mut self, room_id: u32) {
        self.send_msg(json!({
            "class": "lobby",
            "type": "JoinRoomSuccess",
            "roomid": room_id.to_string()
        }))
        .await;
    }
    pub async fn send_join_room_fail(&mut self) {
        self.send_msg(json!({
            "class": "lobby",
            "type": "JoinRoomFail"
        }))
        .await;
    }

    // game events

    pub async fn send_new_round(
        &mut self,
        open_state: &GameStateOpen,
        hidden_state: &GameStateHidden,
    ) {
        self.send_msg(json!({
            "class": "game",
            "type": "NewRound",
            "open_state": open_state,
            "hidden_state": hidden_state
        }))
        .await;
    }
    pub async fn send_new_turn(&mut self, open_state: &GameStateOpen) {
        self.send_msg(json!({
            "class": "game",
            "type": "NewTurn",
            "open_state": open_state,
        }))
        .await;
    }
    pub async fn send_use_item(&mut self, open_state: &GameStateOpen) {
        self.send_msg(json!({
            "class": "game",
            "type": "UseItem",
            "open_state": open_state,
        }))
        .await;
    }
    pub async fn send_shoot(&mut self, open_state: &GameStateOpen) {
        self.send_msg(json!({
            "class": "game",
            "type": "UseItem",
            "open_state": open_state,
        }))
        .await;
    }
    pub async fn send_drawed_item(&mut self, open_state: &GameStateOpen) {
        self.send_msg(json!({
            "class": "game",
            "type": "UpdateCard",
            "open_state": open_state,
        }))
        .await;
    }
    pub async fn send_draw_item_pool(&mut self, open_state: &GameStateOpen, pool: Vec<GameItem>) {
        self.send_msg(json!({
            "class": "game",
            "type": "DrawItemPool",
            "open_state": open_state,
            "item_pool": pool
        }))
        .await;
    }
    pub async fn send_game_end(&mut self, open_state: &GameStateOpen, win: bool) {
        self.send_msg(json!({
            "class": "game",
            "type": "GameEnd",
            "win": win,
            "open_state": open_state,
        }))
        .await;
    }
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        Connection {
            id: self.id,
            sender: self.sender.clone(),
            game_event_tx: self.game_event_tx.clone(),
        }
    }
}
