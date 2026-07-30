// NONOS Operating System (AGPL-3.0-or-later)
//! Named cases beside the Kani harnesses, for the specific addresses that
//! matter and for the shape of the result.

use crate::local::{from_random, is_factory_assigned, is_local_unicast, MAC_LEN};

/// A real Intel OUI, the kind an e1000 EEPROM holds.
const FACTORY: [u8; MAC_LEN] = [0x00, 0x1B, 0x21, 0x3C, 0x4D, 0x5E];

#[test]
fn factory_address_is_recognised() {
    assert!(is_factory_assigned(&FACTORY));
    assert!(!is_local_unicast(&FACTORY));
}

#[test]
fn factory_address_becomes_local() {
    let mac = from_random(FACTORY);
    assert!(is_local_unicast(&mac));
    assert!(!is_factory_assigned(&mac));
    assert_ne!(mac, FACTORY);
}

#[test]
fn only_the_first_octet_moves() {
    let bytes = [0x00, 0x1B, 0x21, 0x3C, 0x4D, 0x5E];
    let mac = from_random(bytes);
    assert_eq!(&mac[1..], &bytes[1..]);
}

#[test]
fn group_bit_is_cleared() {
    // A multicast address as the input: the group bit has to come back off,
    // because a station cannot transmit from a group address.
    let mac = from_random([0x01, 0x00, 0x5E, 0x00, 0x00, 0x01]);
    assert_eq!(mac[0] & 0x01, 0);
    assert!(is_local_unicast(&mac));
}

#[test]
fn broadcast_input_is_defanged() {
    let mac = from_random([0xFF; MAC_LEN]);
    assert_ne!(mac, [0xFF; MAC_LEN]);
    assert!(is_local_unicast(&mac));
}

#[test]
fn zero_input_is_not_zero_output() {
    let mac = from_random([0x00; MAC_LEN]);
    assert_ne!(mac, [0x00; MAC_LEN]);
    assert!(is_local_unicast(&mac));
}

#[test]
fn applying_twice_changes_nothing() {
    let once = from_random([0x9A, 0x11, 0x22, 0x33, 0x44, 0x55]);
    assert_eq!(from_random(once), once);
}
