mod game;
mod messages;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use messages::{Parse, Send};
use rand::Rng;
use tokio::{self, time};

fn gen_game() -> Vec<String> {
    let server_seed = game::generate_random_seed();
    let client_seed = game::generate_random_seed();

    println!("Server seed: {server_seed}");
    println!("Client seed: {client_seed}");

    let start = chrono::offset::Local::now();
    let result = game::generate_chain(&server_seed, &client_seed, 10000000);
    println!("{:?}", chrono::offset::Local::now() - start);

    result
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/crash", get(crash_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030")
        .await
        .unwrap();

    println!("Servidor rodando em http://127.0.0.1:3030");

    axum::serve(listener, app).await.unwrap();
}

async fn crash_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_game_req)
}

async fn handle_game_req(socket: WebSocket) {
    let (mut sender, mut _receiver) = socket.split();

    let mut game = gen_game();

    tokio::spawn(async move {
        loop {
            let hash = get_random_value(&mut game);
            let point = game::get_point(&hash);
            let send = Send { hash, point };

            if sender
                .send(Message::Text(send.dump().into()))
                .await
                .is_err()
            {
                break;
            }

            if game.is_empty() {
                game = gen_game();
                continue;
            }

            time::sleep(time::Duration::from_secs(2)).await;
        }
    });
}

fn get_random_value(vec: &mut Vec<String>) -> String {
    let mut rng = rand::rng();
    let index = rng.random_range(0..vec.len());

    vec.remove(index)
}
