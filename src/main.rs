mod game_logic;
mod handlers;
mod utils;

use handlers::connections::Connection;
use handlers::lobby::{Lobby, LobbyClientEvent};
use log::{debug, error, info, trace, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::{net::tcp, sync, task};
use tokio_tungstenite::{self, accept_async};

const PORT: &str = "6444";

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let addr = format!("0.0.0.0:{}", PORT);

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Listening TCP failed.");

    println!("Listening on: {}", addr);

    let (lobby_client_mq_tx, lobby_client_mq_rx) = mpsc::unbounded_channel::<LobbyClientEvent>();
    

    // Listen lobby for room creation and matchmaking
    tokio::spawn(Lobby::listen(lobby_client_mq_tx.clone(), lobby_client_mq_rx));
    // Listen room for game logic
    // tokio::spawn(Room::listen(room_mq_tx.clone(), room_receiver));

    // Accept new clients
    while let Ok((stream, peer)) = listener.accept().await {
        match accept_async(stream).await {
            Err(e) => println!("Websocket connection error : {}", e),
            Ok(ws_stream) => {
                println!("New connection : {}", peer);
                tokio::spawn(Connection::handshake(ws_stream, lobby_client_mq_tx.clone()));
            }
        }
    }
}
