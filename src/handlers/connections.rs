use crate::{game_logic::game_event::GameEvent, utils::player::Player};
use axum::http::header;
use futures_util::SinkExt;
use log::{debug, error, info, trace, warn};
use quick_protobuf::{BytesReader, MessageRead};
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use tokio_tungstenite::{tungstenite, WebSocketStream};
use tungstenite::Message;

use super::lobby::LobbyClientEvent;

use futures_util::stream::StreamExt;
use futures_util::stream::{SplitSink, SplitStream};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

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

pub struct Connection {
    pub id: Player,
    pub sender: Arc<tokio::sync::Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
}

impl Connection {
    pub fn new(
        id: Player,
        sender: Arc<tokio::sync::Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    ) -> Self {
        Connection { id, sender }
    }
    pub async fn handshake(
        ws_stream: WebSocketStream<TcpStream>,
        lobby_client_mq_tx: UnboundedSender<LobbyClientEvent>,
    ) {
        let (ws_tx, mut ws_rx) = ws_stream.split();
        let ws_tx = Arc::from(tokio::sync::Mutex::from(ws_tx));

        let (tx, rx) = oneshot::channel::<Player>();
        if let Some(msg) = ws_rx.next().await {
            if let Ok(msg) = msg {
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
                                .send(LobbyClientEvent::HandShake(player, conn, tx))
                                .unwrap();
                        }
                        _ => {
                            warn!("invalid hanshake message!");
                            return;
                        }
                    }
                }
            } else {
                return;
            }
        }

        if let Ok(player) = rx.await {
            tokio::spawn(Self::listen_lobby(player, ws_rx, lobby_client_mq_tx));
        } else {
            warn!("Handshake refused!");
        }
    }

    // use ? to simplify code
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
        match type_ {
            "CreateRoom" => Some(WsRxMsgType::CreateRoom),
            "JoinRoom" => Some(WsRxMsgType::JoinRoom),
            "HandShake" => Some(WsRxMsgType::Handshake),
            "LeaveRoom" => Some(WsRxMsgType::LeaveRoom),
            "UseItem" => Some(WsRxMsgType::UseItem),
            "DrawItem" => Some(WsRxMsgType::DrawItem),
            "Shoot" => Some(WsRxMsgType::Shoot),
            _ => None,
        }
    }
    fn check_rx_msg(msg: &serde_json::Value) -> Option<(WsMsgClass, WsRxMsgType)> {
        let class = Self::check_rx_msg_class(msg)?;
        let type_ = Self::check_rx_msg_type(msg)?;
        Some((class, type_))
    }
    pub fn parse_receive_lobby(
        player: Player,
        msg: Vec<u8>,
    ) -> Result<
        (
            Option<oneshot::Receiver<UnboundedSender<GameEvent>>>,
            LobbyClientEvent,
        ),
        &'static str,
    > {
        let j = serde_json::from_slice::<Value>(&msg).or(Err("Invalid json {}"))?;
        let (msg_class, msg_type) = Self::check_rx_msg(&j).ok_or("Invalid json {}")?;

        match msg_type {
            WsRxMsgType::CreateRoom => {
                let (tx, rx) = oneshot::channel();
                Ok((Some(rx), LobbyClientEvent::CreateRoom(player, tx)))
            }
            WsRxMsgType::JoinRoom => {
                let (tx, rx) = oneshot::channel();
                let room_id = j
                    .get("roomid")
                    .ok_or("Invalid json")?
                    .as_str()
                    .ok_or("Invalid json")?
                    .parse()
                    .or(Err("Invalid json"))?;
                Ok((Some(rx), LobbyClientEvent::JoinRoom(player, room_id)))
            }
            WsRxMsgType::LeaveRoom => Ok((None, LobbyClientEvent::LeaveRoom(player))),

            /*
            WsRxMsgType::MatchMake => {
                todo!("parse_receive_lobby: header is LobbyClientEvent::MatchMake");
                Ok((None, LobbyClientEvent::MatchMake(player)))
            }
            */
            // no handshake here !!!
            _ => Err("Undefined msg type"),
        }
    }

    async fn listen_lobby(
        player: Player,
        mut ws_rx: SplitStream<WebSocketStream<TcpStream>>,
        to_lobby: UnboundedSender<LobbyClientEvent>,
    ) {
        info!("listening player id [{}]", player);
        while let Some(msg) = ws_rx.next().await {
            if let Ok(msg) = msg {
                if msg.is_binary() {
                    if let Ok((recv, event)) = Self::parse_receive_lobby(player, msg.into_data()) {
                        to_lobby.send(event).unwrap();
                        if let Some(rx) = recv {
                            if let Ok(to_game) = rx.await {
                                // tokio::spawn(Self::listen_game(player, ws_rx, to_lobby, to_game));
                                todo!("- to-game");
                            }
                        }
                    }
                } else if msg.is_close() {
                    break;
                }
            } else {
                break;
            }
        }
        to_lobby.send(LobbyClientEvent::LeaveRoom(player)).unwrap();
    }

    // todo
    // pub fn parse_receive_game
    // aync fn listen_game

    async fn send_msg(&self, msg: serde_json::Value) {
        self.sender
            .lock()
            .await
            .send(Message::text(msg.to_string()))
            .await
            .unwrap();
    }
    // lobby events
    pub async fn send_opponent_leave(&self) {
        self.send_msg(json!({
            "class": "lobby",
            "type": "OpponentLeave"
        }))
        .await;
    }
    pub async fn send_opponent_join(&self) {
        self.send_msg(json!({
            "class": "lobby",
            "type": "OpponentJoin"
        }))
        .await;
    }
    pub async fn send_room_create_success(&self, room_id: u32) {
        self.send_msg(json!({
            "class": "lobby",
            "type": "CreateRoomSuccess",
            "roomid": room_id.to_string()
        }))
        .await;
    }
    pub async fn send_room_create_fail(&self) {
        self.send_msg(json!({
                "class": "lobby",
                "type": "CreateRoomFail"
        }))
        .await;
    }
    pub async fn send_handshake_success(&self, player: Player) {
        self.send_msg(json!({
            "class": "lobby",
            "type": "HandShakeSuccess",
            "playerid": player.id().to_string()
        }))
        .await;
    }
    pub async fn send_handshake_fail(&self) {
        self.send_msg(json!({
            "class": "lobby",
            "type": "HandShakeFail"
        }))
        .await;
    }
    pub async fn send_join_room_success(&self, room_id: u32) {
        self.send_msg(json!({
            "class": "lobby",
            "type": "JoinRoomSuccess",
            "roomid": room_id.to_string()
        }))
        .await;
    }
    pub async fn send_join_room_fail(&self) {
        self.send_msg(json!({
            "class": "lobby",
            "type": "JoinRoomFail"
        }))
        .await;
    }

    // game events
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        Connection {
            id: self.id,
            sender: self.sender.clone(),
        }
    }
}
