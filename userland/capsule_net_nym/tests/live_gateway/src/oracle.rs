//! Process a packet this capsule built with the reference implementation.
//!
//! A mixnet drops what it cannot read and says nothing, so a packet that is
//! wrong anywhere looks the same as one that was never sent: no reply. This
//! runs the packet through the real Sphinx code hop by hop with keys held on
//! both sides, so a fault is reported at the layer it happens on instead of
//! being inferred from silence twenty seconds later.

use rand::RngCore;
use sphinx_packet::{ProcessedPacketData, SphinxPacket as RefPacket};
// The reference crate pins its own x25519 version, so its keys are the
// ones used here: passing ours would not type check, and converting at the
// boundary is what keeps the check honest about which code held which key.
use sphinx_packet::crypto::{PrivateKey as StaticSecret, PublicKey};

use crate::sphinx::node::{Destination, Node};
use crate::sphinx::packet::build_packet;

/// Build with our code, unwrap with theirs, and report where it stops.
pub fn run(hops: usize) -> bool {
    let secrets: Vec<StaticSecret> = (0..hops).map(|_| random_secret()).collect();
    let ours: Vec<Node> = secrets
        .iter()
        .enumerate()
        .map(|(i, s)| Node { address: address(i as u8), pub_key: PublicKey::from(s).to_bytes() })
        .collect();

    let mut identifier = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut identifier);
    let dest = Destination { address: [7u8; 32], identifier };
    let delays = vec![[0u8; 8]; hops];
    let mut initial = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut initial);

    let message = b"nonos reference cross check";
    let version = crate::sphinx::constants::PACKET_VERSION;
    let built = match build_packet(&initial, &ours, &dest, &delays, version, message) {
        Ok(p) => p,
        Err(e) => {
            println!("  our build_packet refused the route: {e:?}");
            return false;
        }
    };
    let wire = match built.to_bytes() {
        Some(b) => b,
        None => {
            println!("  our packet would not serialise");
            return false;
        }
    };
    unwrap_all(&wire, &secrets, message)
}

fn unwrap_all(wire: &[u8], secrets: &[StaticSecret], message: &[u8]) -> bool {
    let mut packet = match RefPacket::from_bytes(wire) {
        Ok(p) => p,
        Err(e) => {
            println!("  reference refused our bytes: {e}");
            return false;
        }
    };
    for (i, secret) in secrets.iter().enumerate() {
        let processed = match packet.process(secret) {
            Ok(p) => p,
            Err(e) => {
                println!("  hop {} could not process our packet: {}", i + 1, e);
                return false;
            }
        };
        match processed.data {
            ProcessedPacketData::ForwardHop { next_hop_packet, .. } => {
                if i + 1 == secrets.len() {
                    println!("  hop {} was a forward hop but should have been the last", i + 1);
                    return false;
                }
                packet = next_hop_packet;
            }
            ProcessedPacketData::FinalHop { payload, .. } => {
                return final_hop(i, payload, message, secrets.len());
            }
        }
    }
    println!("  route ran out of hops without reaching the destination");
    false
}

fn final_hop(
    i: usize,
    payload: sphinx_packet::payload::Payload,
    message: &[u8],
    hops: usize,
) -> bool {
    if i + 1 != hops {
        println!("  hop {} ended the route early", i + 1);
        return false;
    }
    match payload.recover_plaintext() {
        Ok(text) if text == message => {
            println!("  {hops} hops: message recovered intact");
            true
        }
        Ok(text) => {
            println!("  {hops} hops: recovered {} bytes, not the message sent", text.len());
            false
        }
        Err(e) => {
            println!("  {hops} hops: the last hop rejected the payload: {e}");
            false
        }
    }
}

/// Build the same loopback packet with the reference implementation.
///
/// Sent down the identical route, it separates two things that look alike
/// from here: a packet the network refuses, and a route the network was never
/// going to answer. Whatever happens to this one is the network's behaviour
/// with our code taken out of it.
pub fn reference_packet(
    route: &[([u8; 32], [u8; 32])],
    destination: [u8; 32],
    message: &[u8],
) -> Option<Vec<u8>> {
    use sphinx_packet::header::delays::Delay;
    use sphinx_packet::route::{
        Destination as RefDest, DestinationAddressBytes, Node as RefNode, NodeAddressBytes,
    };

    let nodes: Vec<RefNode> = route
        .iter()
        .map(|(address, key)| {
            RefNode::new(NodeAddressBytes::from_bytes(*address), PublicKey::from(*key))
        })
        .collect();
    let mut identifier = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut identifier);
    let dest = RefDest::new(DestinationAddressBytes::from_bytes(destination), identifier);
    let delays: Vec<Delay> = (0..nodes.len()).map(|_| Delay::new_from_nanos(0)).collect();
    // Built at the width and version the capsule uses, so the two packets
    // differ in how they were made and in nothing else. The builder defaults
    // to a newer version than the one we send, which would compare two
    // different things.
    let width = crate::sphinx::constants::REGULAR_PAYLOAD_SIZE;
    let version = sphinx_packet::version::Version::new(
        std::env::var("NONOS_SPHINX_VERSION").ok().and_then(|v| v.parse().ok()).unwrap_or(258),
    );
    sphinx_packet::SphinxPacketBuilder::default()
        .with_version(version)
        .with_payload_size(width)
        .build_packet(message.to_vec(), &nodes, &dest, &delays)
        .ok()
        .map(|p| p.to_bytes())
}

fn random_secret() -> StaticSecret {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    StaticSecret::from(bytes)
}

// A routing address as the capsule writes one: kind, port, then the octets.
fn address(seed: u8) -> [u8; 32] {
    let mut addr = [0u8; 32];
    addr[0] = 4;
    addr[1..3].copy_from_slice(&1789u16.to_be_bytes());
    addr[3..7].copy_from_slice(&[10, 0, 0, seed + 1]);
    addr
}


/// Check a reply block the way the far end reads one.
///
/// A recipient parses every attached block before it looks at the request,
/// and slices them at a fixed width, so a block that is the wrong length or
/// whose header will not parse takes the whole message down with it. Nothing
/// says so: the packets are still acknowledged, because the acknowledgement
/// is lifted out of the payload before any of this is read.
pub fn check_surb() -> bool {
    use crate::sphinx::constants::{HEADER_SIZE, NODE_ADDRESS_LENGTH, PAYLOAD_KEY_SIZE};
    use crate::surb::{build_surb, surb_bytes, SURB_KEY_BYTES};

    let hops: Vec<crate::sphinx::node::Node> = (0..4)
        .map(|i| crate::sphinx::node::Node {
            address: address(i as u8),
            pub_key: PublicKey::from(&random_secret()).to_bytes(),
        })
        .collect();
    let delays = vec![[0u8; 8]; hops.len()];
    let surb = match build_surb(&hops, &delays, &[9u8; 32]) {
        Ok(surb) => surb,
        Err(e) => {
            println!("  our build_surb refused the route home: {e:?}");
            return false;
        }
    };
    let bytes = surb_bytes(&surb);

    // The width the far end slices at: key, header, first hop, one payload
    // key per hop of a four hop route home.
    let expected = SURB_KEY_BYTES + HEADER_SIZE + NODE_ADDRESS_LENGTH + 4 * PAYLOAD_KEY_SIZE;
    if bytes.len() != expected {
        println!("  reply block is {} bytes, the far end slices at {}", bytes.len(), expected);
        return false;
    }

    let header = &bytes[SURB_KEY_BYTES..SURB_KEY_BYTES + HEADER_SIZE];
    match sphinx_packet::header::SphinxHeader::from_bytes(header) {
        Ok(_) => {
            println!("  reply block: {expected} bytes, header parses");
            true
        }
        Err(e) => {
            println!("  the reference could not parse our reply block header: {e}");
            false
        }
    }
}

/// Write out the message this capsule would send, fragment by fragment.
///
/// Every layer below this has been checked against the reference in
/// isolation, and each one passes. What has never been read by Nym's own
/// code is the message itself: the thing the exit actually parses before it
/// decides whether to answer. Dumping it lets that parser say what is wrong.
pub fn dump_message(dir: &str) -> bool {
    use crate::message::prepare;
    use crate::surb::{build_surb, surb_bytes};

    let hops: Vec<crate::sphinx::node::Node> = (0..4)
        .map(|i| crate::sphinx::node::Node {
            address: address(i as u8),
            pub_key: PublicKey::from(&random_secret()).to_bytes(),
        })
        .collect();
    let delays = vec![[0u8; 8]; hops.len()];
    let mut surbs = Vec::new();
    for _ in 0..8 {
        match build_surb(&hops, &delays, &[9u8; 32]) {
            Ok(surb) => surbs.push(surb_bytes(&surb)),
            Err(e) => {
                println!("  build_surb failed: {e:?}");
                return false;
            }
        }
    }
    let request = b"\x03\x00\x00\x00\x00\x00\x00\x00\x01\x00\x10icanhazip.com:80";
    let Some(prepared) = prepare(&[7u8; 16], &surbs, request, 0x1234) else {
        println!("  prepare refused the message");
        return false;
    };
    for (i, fragment) in prepared.fragments.iter().enumerate() {
        let path = format!("{dir}/fragment-{i}.bin");
        if std::fs::write(&path, fragment).is_err() {
            println!("  could not write {path}");
            return false;
        }
    }
    println!("  wrote {} fragments to {dir}", prepared.fragments.len());
    true
}

/// Write out one packet payload, with the recipient's secret alongside it.
///
/// This is the last link nothing has checked: the recipient derives the key
/// for our fragment by repeating a Diffie-Hellman with the public half we
/// send, then a KDF over the result. If any of that disagrees, it decrypts
/// our fragment into noise and drops the message, and still returns the
/// acknowledgement, because the acknowledgement is read out of the payload
/// in the clear before any of this happens.
pub fn dump_payload(dir: &str) -> bool {
    use crate::payload::build_payload;

    let recipient_secret = random_secret();
    let recipient_public = PublicKey::from(&recipient_secret);

    // The ack occupies a fixed span the recipient skips over; its contents do
    // not take part in the encryption, so a marker pattern is enough.
    // Any fixed span will do: the ack takes no part in the encryption, and
    // the reference is told where it ends.
    let ack = vec![0xABu8; 418];
    let fragment = b"the fragment the recipient must recover unchanged".to_vec();

    let payload = match build_payload(&ack, &recipient_public.to_bytes(), &fragment) {
        Ok(payload) => payload,
        Err(e) => {
            println!("  our build_payload failed: {e:?}");
            return false;
        }
    };
    let ok = std::fs::write(format!("{dir}/recipient-secret.bin"), recipient_secret.to_bytes())
        .is_ok()
        && std::fs::write(format!("{dir}/payload.bin"), &payload).is_ok()
        && std::fs::write(format!("{dir}/fragment-plain.bin"), &fragment).is_ok()
        && std::fs::write(format!("{dir}/ack-len.txt"), ack.len().to_string()).is_ok();
    if !ok {
        println!("  could not write the payload files");
        return false;
    }
    println!("  wrote a {} byte payload for the reference to open", payload.len());
    true
}
