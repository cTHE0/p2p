use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Demander à l'utilisateur s'il veut être l'expéditeur ou le receveur
    println!("Voulez-vous être l'expéditeur (s) ou le receveur (r) ?");
    let mut role = String::new();
    std::io::stdin().read_line(&mut role)?;

    if role.trim() == "s" {
        // Expéditeur
        println!("Entrez l'adresse IP et le port du receveur (ex: 192.168.1.2:8080):");
        let mut address = String::new();
        std::io::stdin().read_line(&mut address)?;

        let mut stream = TcpStream::connect(address.trim()).await?;
        println!("Connecté au receveur. Entrez le texte à envoyer:");

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            stream.write_all(input.as_bytes()).await?;
        }
    } else if role.trim() == "r" {
        // Receveur
        println!("Entrez le port sur lequel écouter (ex: 8080):");
        let mut port = String::new();
        std::io::stdin().read_line(&mut port)?;

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port.trim())).await?;
        println!("En attente de connexion...");

        let (mut socket, _) = listener.accept().await?;
        println!("Connexion établie!");

        let mut buf = [0; 1024];
        loop {
            let n = socket.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            let received_text = std::str::from_utf8(&buf[..n])?;
            println!("Message reçu: {}", received_text);
        }
    } else {
        println!("Rôle non reconnu. Veuillez choisir 's' pour expéditeur ou 'r' pour receveur.");
    }

    Ok(())
}