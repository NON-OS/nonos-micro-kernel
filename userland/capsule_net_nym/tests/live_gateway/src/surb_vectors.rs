//! Reply blocks, against the layout nym's reader expects.

use crate::sphinx_root::sphinx::constants::{
    HEADER_SIZE, NODE_ADDRESS_LENGTH, PAYLOAD_KEY_SIZE,
};
use crate::surb::{surb_bytes, ReplySurb, SURB_KEY_BYTES};

fn fixture(hops: usize) -> ReplySurb {
    ReplySurb {
        key: [0xaa; SURB_KEY_BYTES],
        header: [0xbb; HEADER_SIZE],
        first_hop_address: [0xcc; NODE_ADDRESS_LENGTH],
        payload_keys: (0..hops).map(|i| [i as u8; PAYLOAD_KEY_SIZE]).collect(),
    }
}

/// The order nym reads a reply block in: our key, then the block itself,
/// which is the header, the first hop, and the per hop keys.
#[test]
fn a_reply_block_matches_the_reference_layout() {
    let surb = fixture(4);
    let bytes = surb_bytes(&surb);

    let mut at = 0;
    assert_eq!(&bytes[at..at + SURB_KEY_BYTES], &[0xaa; SURB_KEY_BYTES]);
    at += SURB_KEY_BYTES;
    assert_eq!(&bytes[at..at + HEADER_SIZE], &[0xbb; HEADER_SIZE][..]);
    at += HEADER_SIZE;
    assert_eq!(&bytes[at..at + NODE_ADDRESS_LENGTH], &[0xcc; NODE_ADDRESS_LENGTH]);
    at += NODE_ADDRESS_LENGTH;
    for hop in 0..4u8 {
        assert_eq!(&bytes[at..at + PAYLOAD_KEY_SIZE], &[hop; PAYLOAD_KEY_SIZE][..]);
        at += PAYLOAD_KEY_SIZE;
    }
    assert_eq!(at, bytes.len(), "nothing may trail the last hop key");
}

/// The reader tells the two key forms apart by length, so a block carrying
/// full keys has to stay at or above one whole key.
#[test]
fn a_reply_block_carries_full_keys_not_seeds() {
    let bytes = surb_bytes(&fixture(4));
    let material = bytes.len() - SURB_KEY_BYTES - HEADER_SIZE - NODE_ADDRESS_LENGTH;
    assert_eq!(material, 4 * PAYLOAD_KEY_SIZE);
    assert!(material >= PAYLOAD_KEY_SIZE, "shorter would be read as key seeds");
}

/// A block is sized entirely by its hop count, so its length says how long
/// the route home is and nothing else.
#[test]
fn a_reply_block_is_sized_by_its_route() {
    let three = surb_bytes(&fixture(3)).len();
    let four = surb_bytes(&fixture(4)).len();
    assert_eq!(four - three, PAYLOAD_KEY_SIZE);
}
