// NONOS Operating System (AGPL-3.0-or-later)
//! Kani harnesses: the address rules hold for every input, not just the ones
//! a test happened to pick.
//!
//! Randomness is the point of this crate, so named cases prove very little on
//! their own. These quantify over all six octets.

use crate::local::{from_random, is_factory_assigned, is_local_unicast, MAC_LEN};

fn any_mac() -> [u8; MAC_LEN] {
    [kani::any(), kani::any(), kani::any(), kani::any(), kani::any(), kani::any()]
}

/// Whatever bytes arrive, the result is transmittable: one station, not a
/// vendor range. This is the property the privacy claim rests on, so it has to
/// hold for every draw rather than for most of them.
#[kani::proof]
fn always_local_unicast() {
    let mac = from_random(any_mac());
    assert!(is_local_unicast(&mac));
    assert!(!is_factory_assigned(&mac));
}

/// The forty six bits that are not the group and local flags are passed
/// through untouched. An implementation that quietly fixed more of the address
/// would shrink the space these are drawn from, and a smaller space is easier
/// to correlate across networks.
#[kani::proof]
fn keeps_every_other_bit() {
    let bytes = any_mac();
    let mac = from_random(bytes);

    assert_eq!(mac[0] & 0xFC, bytes[0] & 0xFC);
    assert_eq!(mac[1], bytes[1]);
    assert_eq!(mac[2], bytes[2]);
    assert_eq!(mac[3], bytes[3]);
    assert_eq!(mac[4], bytes[4]);
    assert_eq!(mac[5], bytes[5]);
}

/// A factory address can never come back out. Whatever the EEPROM held, the
/// result is in a locally administered range, so the two can never collide.
#[kani::proof]
fn never_returns_a_factory_address() {
    let factory = any_mac();
    kani::assume(is_factory_assigned(&factory));

    let mac = from_random(any_mac());
    assert!(mac[0] != factory[0]);
}

/// Broadcast has every bit of the first octet set, which includes the group
/// bit, so the result is never the broadcast address.
#[kani::proof]
fn never_broadcast() {
    let mac = from_random(any_mac());
    assert!(mac != [0xFF; MAC_LEN]);
}

/// Nor the all zero address, which some drivers treat as "no address set".
#[kani::proof]
fn never_all_zero() {
    let mac = from_random(any_mac());
    assert!(mac != [0x00; MAC_LEN]);
}

/// Applying the rule to an address that already follows it changes nothing, so
/// a re-randomised address cannot drift out of the valid range.
#[kani::proof]
fn idempotent() {
    let once = from_random(any_mac());
    assert_eq!(from_random(once), once);
}
