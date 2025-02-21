use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    println!("Voulez-vous envoyer ou écouter des messages ? (envoyer/écouter)");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim().to_lowercase();

    match choice.as_str() {
        "envoyer" => send_message().await,
        "écouter" => listen_for_messages().await,
        _ => println!("Choix invalide. Veuillez entrer 'envoyer' ou 'écouter'."),
    }
}

async fn listen_for_messages() {
    let addr = "0.0.0.0:9001";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Écoute sur: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake");
        println!("Nouvelle connexion WebSocket: {}", addr);

        let (mut write, mut read) = ws_stream.split();

        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                let msg = msg.expect("Failed to read message");
                if let Message::Text(text) = msg {
                    println!("Message reçu: {}", text);
                    write.send(Message::Text(text)).await.expect("Failed to send message");
                }
            }
        });
    }
}

async fn send_message() {
    println!("Veuillez entrer l'adresse IP du pair:");
    let mut ip_address = String::new();
    io::stdin().read_line(&mut ip_address).expect("Failed to read line");
    let ip_address = ip_address.trim();

    let url = format!("ws://{}:9001", ip_address);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connecté au serveur");

    let (mut write, mut read) = ws_stream.split();

    println!("Veuillez entrer le message à envoyer:");
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read line");
    let message = message.trim().to_string();

    write.send(Message::Text(message)).await.expect("Failed to send message");

    while let Some(msg) = read.next().await {
        let msg = msg.expect("Failed to read message");
        if let Message::Text(text) = msg {
            println!("Message reçu: {}", text);
        }
    }
}
