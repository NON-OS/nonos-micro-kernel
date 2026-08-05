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
mod oracle;
#[path = "surb_mod.rs"]
mod surb;
#[path = "../../../src/ack/mod.rs"]
mod ack;
#[path = "../../../src/payload/mod.rs"]
mod payload;
#[path = "reply_mod.rs"]
mod reply;
#[cfg(test)]
mod real_api;
#[cfg(test)]
mod message_vectors;
#[cfg(test)]
mod reply_vectors;
#[cfg(test)]
mod payload_vectors;
#[cfg(test)]
mod surb_vectors;
#[path = "../../../src/message/mod.rs"]
mod message;
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
    if a.get(1).map(|s| s.as_str()) == Some("oracle") {
        // Every route length the capsule can build, checked against the real
        // implementation before any of it is trusted on a live network.
        let mut all = true;
        for hops in 1..=5 {
            all &= oracle::run(hops);
        }
        all &= oracle::check_surb();
        if let Some(dir) = a.get(2) {
            all &= oracle::dump_message(dir);
            all &= oracle::dump_payload(dir);
        }
        println!("\n{}", if all { "*** every route verified against the reference ***" } else { "*** the reference rejected at least one route ***" });
        return;
    }
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
            claim_bandwidth(&mut wire);
            if a.len() > 3 {
                loopback(&mut wire, &key, &own_public, &a[3]);
            } else {
                println!("\n*** run_handshake() COMPLETED — gateway accepted registration ***");
            }
        }
        Err(e) => println!("\nhandshake failed: {:?}", e),
    }
}

/// Ask the gateway for allowance and print what it says.
///
/// A gateway prices every packet before it forwards it and drops one it has
/// no credit for, so whatever comes back here decides whether anything sent
/// afterwards can leave at all.
fn claim_bandwidth(wire: &mut LiveWire) {
    let claim = "{\"type\":\"claimFreeTestnetBandwidth\"}";
    println!("-> {}", claim);
    if wire.0.send(Message::Text(claim.to_string())).is_err() {
        println!("claim could not be sent");
        return;
    }
    if let MaybeTlsStream::Plain(t) = wire.0.get_mut() {
        let _ = t.set_read_timeout(Some(std::time::Duration::from_secs(5)));
    }
    for _ in 0..3 {
        match wire.0.read() {
            Ok(Message::Text(t)) => println!("<- {}", &t[..240.min(t.len())]),
            Ok(Message::Binary(b)) => println!("<- binary {} bytes", b.len()),
            Ok(_) => continue,
            Err(_) => break,
        }
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
    // With NONOS_REF set the packet comes from the reference implementation
    // instead of ours, down the same route to the same destination.
    if std::env::var("NONOS_REF").is_ok() {
        let spec: Vec<([u8; 32], [u8; 32])> =
            hops.iter().map(|h| (h.address, h.pub_key)).collect();
        match crate::oracle::reference_packet(&spec, *own_public, msg) {
            Some(bytes) => {
                println!("built REFERENCE sphinx packet: {} bytes", bytes.len());
                return send_and_wait(wire, shared, &hops[0].address, &bytes);
            }
            None => {
                println!("reference refused to build the packet");
                return;
            }
        }
    }
    // The version the shipping constant names, not a hand-written one: a hop
    // reads this to decide how to derive its payload key, so a route built
    // with any other value is not the route the capsule builds.
    let version = sphinx::constants::PACKET_VERSION;
    let packet = match build_packet(&secret, &hops, &dest, &delays, version, msg) {
        Ok(p) => p,
        Err(e) => { println!("build_packet failed: {:?}", e); return; }
    };
    let bytes = packet.to_bytes().expect("wire");
    println!("built sphinx packet: {} bytes", bytes.len());
    send_and_wait(wire, shared, &hops[0].address, &bytes);
}

/// Wrap a Sphinx packet for the gateway, send it, and wait for it to return.
///
/// Whoever built the packet, everything from here on is identical, so the two
/// runs differ in exactly one thing: which implementation wrote the bytes.
fn send_and_wait(wire: &mut LiveWire, shared: &[u8; 32], first_hop: &[u8; 32], bytes: &[u8]) {
    // The gateway is handed a MixPacket, not a bare Sphinx packet: it needs to
    // know which mix to forward to, and the first hop is deliberately absent
    // from the header. v1 layout is packet_type, next hop, then the packet;
    // v2 names the sphinx key rotation between the two, so a node holding two
    // active keys knows which one the packet was built against instead of
    // having to assume the current one.
    let v2 = std::env::var("NONOS_V2").ok();
    let mut mix_packet = vec![0u8];             // PacketType::Mix
    if let Some(rotation) = v2.as_deref() {
        mix_packet.push(rotation.parse::<u8>().unwrap_or(0));
    }
    mix_packet.extend_from_slice(&first_hop[..7]);  // next hop, unpadded
    mix_packet.extend_from_slice(bytes);
    println!("mix packet: {} bytes (1 type + 7 next hop + {} sphinx)", mix_packet.len(), bytes.len());
    let bytes = mix_packet;

    // ForwardSphinx frame: kind, encrypted flag, nonce, sealed body
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    let sealed = crypto::gcm_siv::seal(shared, &nonce, &[], &bytes);
    let kind = if v2.is_some() { 2u8 } else { 1u8 };
    let mut frame = vec![kind, 1u8];
    frame.extend_from_slice(&nonce);
    frame.extend_from_slice(&sealed);
    println!("-> ForwardSphinx frame, {} bytes", frame.len());
    wire.0.send(Message::Binary(frame)).expect("send packet");

    println!("waiting for the mixnet to hand it back (20s)...");
    // Without a deadline a silent drop is indistinguishable from a slow route.
    if let MaybeTlsStream::Plain(t) = wire.0.get_mut() {
        let _ = t.set_read_timeout(Some(std::time::Duration::from_secs(20)));
    }
    // A read that times out is the normal state of a link with nothing on it
    // yet, so the wait is bounded by the clock rather than by a read count.
    let until = std::time::Instant::now() + std::time::Duration::from_secs(30);
    if let MaybeTlsStream::Plain(t) = wire.0.get_mut() {
        let _ = t.set_read_timeout(Some(std::time::Duration::from_secs(2)));
    }
    while std::time::Instant::now() < until {
        match wire.0.read() {
            Ok(Message::Binary(b)) => {
                println!("<- binary frame, {} bytes, kind {}", b.len(), b[0]);
                if b[0] == 1 {
                    println!("\n*** SPHINX PACKET COMPLETED THE MIXNET ROUND TRIP ***");
                    return;
                }
            }
            Ok(Message::Text(t)) => println!("<- text {}", &t[..160.min(t.len())]),
            Ok(_) => continue,
            Err(tungstenite::Error::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => {
                println!("read ended: {}", e);
                break;
            }
        }
    }
    println!("\nno packet came back");
}
