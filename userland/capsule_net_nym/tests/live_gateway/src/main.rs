extern crate alloc;
#[path = "crypto_shim.rs"]
mod crypto;
#[path = "../../../src/json/mod.rs"]
mod json;
#[path = "topology_shim.rs"]
mod topology;
#[path = "../../../src/directory_sync/api/mod.rs"]
mod api;
mod gateway_client;
#[path = "sphinx_mod.rs"]
mod sphinx_root;
use sphinx_root::sphinx;

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
            println!("registered, shared key derived");
            if a.len() > 3 {
                loopback(&mut wire, &key, &own_public, &a[3]);
            } else {
                println!("\n*** run_handshake() COMPLETED — gateway accepted registration ***");
            }
        }
        Err(e) => println!("\nhandshake failed: {:?}", e),
    }
}


/// Send one real Sphinx packet through three real mixes and back to ourselves.
///
/// The route ends at our own gateway with our identity as the destination, so
/// if the packet is well formed the network hands it back and the gateway
/// pushes it to us. Nothing about that can succeed unless the header is
/// correct at every hop.
fn loopback(wire: &mut LiveWire, shared: &[u8; 32], own_public: &[u8; 32], spec: &str) {
    use sphinx::node::{Destination, Node};
    use sphinx::packet::build_packet;

    // spec is "ip:port:b58key,ip:port:b58key,..." for mix1,mix2,mix3,gateway
    let hops: Vec<Node> = spec
        .split(',')
        .map(|h| {
            let p: Vec<&str> = h.split(':').collect();
            let ip: Vec<u8> = p[0].split('.').map(|o| o.parse::<u8>().unwrap()).collect();
            let port: u16 = p[1].parse().unwrap();
            let key: [u8; 32] =
                bs58::decode(p[2]).into_vec().unwrap().try_into().unwrap();
            let mut addr = [0u8; 32];
            addr[0] = 4;
            addr[1..3].copy_from_slice(&port.to_be_bytes());
            addr[3..7].copy_from_slice(&ip);
            Node { address: addr, pub_key: key }
        })
        .collect();
    println!("route: {} hops", hops.len());

    let mut ident = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut ident);
    let dest = Destination { address: *own_public, identifier: ident };
    let delays = vec![[0u8; 8]; hops.len()];
    let mut secret = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut secret);

    let msg = b"nonos sphinx loopback";
    let packet = match build_packet(&secret, &hops, &dest, &delays, [1, 0, 0], msg) {
        Ok(p) => p,
        Err(e) => { println!("build_packet failed: {:?}", e); return; }
    };
    let bytes = packet.to_bytes().expect("wire");
    println!("built sphinx packet: {} bytes", bytes.len());

    // The gateway is handed a MixPacket, not a bare Sphinx packet: it needs to
    // know which mix to forward to, and the first hop is deliberately absent
    // from the header. v1 layout is packet_type, next hop, then the packet.
    let mut mix_packet = vec![0u8];             // PacketType::Mix
    mix_packet.extend_from_slice(&hops[0].address[..7]);  // next hop, unpadded
    mix_packet.extend_from_slice(&bytes);
    println!("mix packet: {} bytes (1 type + 7 next hop + {} sphinx)", mix_packet.len(), bytes.len());
    let bytes = mix_packet;

    // ForwardSphinx frame: kind, encrypted flag, nonce, sealed body
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    let sealed = crypto::gcm_siv::seal(shared, &nonce, &[], &bytes);
    let mut frame = vec![1u8, 1u8];
    frame.extend_from_slice(&nonce);
    frame.extend_from_slice(&sealed);
    println!("-> ForwardSphinx frame, {} bytes", frame.len());
    wire.0.send(Message::Binary(frame)).expect("send packet");

    println!("waiting for the mixnet to hand it back (20s)...");
    // Without a deadline a silent drop is indistinguishable from a slow route.
    if let MaybeTlsStream::Plain(t) = wire.0.get_mut() {
        let _ = t.set_read_timeout(Some(std::time::Duration::from_secs(20)));
    }
    for _ in 0..8 {
        match wire.0.read() {
            Ok(Message::Binary(b)) => {
                println!("<- binary frame, {} bytes, kind {}", b.len(), b[0]);
                if b[0] == 1 {
                    println!("\n*** SPHINX PACKET COMPLETED THE MIXNET ROUND TRIP ***");
                    return;
                }
            }
            Ok(Message::Text(t)) => println!("<- text {}", &t[..120.min(t.len())]),
            Ok(_) => continue,
            Err(e) => { println!("read ended: {}", e); break; }
        }
    }
    println!("\nno packet came back");
}
