extern crate alloc;
#[path = "crypto_shim.rs"]
mod crypto;
#[path = "../../../src/json/mod.rs"]
mod json;
mod gateway_client;

use gateway_client::handshake::{run_handshake, HandshakeError, Identity, Wire};
use ed25519_dalek::SigningKey;
use rand::RngCore;
use tungstenite::{connect, Message, WebSocket, stream::MaybeTlsStream};
use std::net::TcpStream;

/// The committed `Wire` trait, implemented over a real socket.
struct LiveWire(WebSocket<MaybeTlsStream<TcpStream>>);

impl Wire for LiveWire {
    fn send_text(&mut self, text: &str) -> Result<(), HandshakeError> {
        self.0.send(Message::Text(text.to_string())).map_err(|_| HandshakeError::Transport)
    }
    fn recv_text(&mut self) -> Result<Vec<u8>, HandshakeError> {
        loop {
            match self.0.read().map_err(|_| HandshakeError::Transport)? {
                Message::Text(t) => return Ok(t.into_bytes()),
                Message::Binary(b) => return Ok(b),
                _ => continue,
            }
        }
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (host, gw_b58) = (&a[1], &a[2]);
    let gw_identity: [u8; 32] =
        bs58::decode(gw_b58).into_vec().expect("b58").try_into().expect("32 bytes");

    let mut seed = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);
    let own_public = SigningKey::from_bytes(&seed).verifying_key().to_bytes();

    println!("gateway {}  identity {}", host, &gw_b58[..12]);
    let (sock, resp) = connect(format!("ws://{}", host)).expect("ws connect");
    println!("websocket connected, http {}", resp.status());

    let mut wire = LiveWire(sock);
    let id = Identity { own_seed: &seed, own_public: &own_public, gateway_public: &gw_identity };

    // The committed run_handshake drives the whole exchange.
    match run_handshake(&mut wire, &id, 3) {
        Ok(key) => {
            println!("shared key {}", key.iter().map(|b| format!("{:02x}", b)).collect::<String>());
            println!("\n*** run_handshake() COMPLETED — gateway accepted registration ***");
        }
        Err(e) => println!("\nhandshake failed: {:?}", e),
    }
}
