mod game_logic;
mod handlers;
mod utils;

use env_logger::Env;
use futures_util::StreamExt;
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
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
    error!("init");
    let addr = format!("0.0.0.0:{}", PORT);

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Listening TCP failed.");

    println!("Listening on: {}", addr);

    let (lobby_client_mq_tx, lobby_client_mq_rx) = mpsc::unbounded_channel::<LobbyClientEvent>();

    // Listen lobby for room creation and matchmaking
    tokio::spawn(Lobby::listen(
        lobby_client_mq_tx.clone(),
        lobby_client_mq_rx,
    ));
    // Listen room for game logic
    // tokio::spawn(Room::listen(room_mq_tx.clone(), room_receiver));

    // Accept new clients
    while let Ok((stream, peer)) = listener.accept().await {
        match accept_async(stream).await {
            Err(e) => println!("Websocket connection error : {}", e),
            Ok(ws_stream) => {
                println!("New connection: {}", peer);
                // read msg from this connection
                ws_stream
                    .for_each(|msg| async move {
                        println!("Received a message: {:?}", &msg);
                    })
                    .await;
                // tokio::spawn(Connection::handshake(
                //     ws_stream,
                //     lobby_client_mq_tx.clone(),
                //     peer,
                // ))
                // .await
                // .expect("Connection failed.");
            }
        }
    }
}
