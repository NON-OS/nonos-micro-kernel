// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs that the RTL8821CE key store satisfies the shared `KeyStore` contract:
//! a pairwise key lands in a CAM entry from 4 up (one per peer, reused on
//! rekey), a group key lands in its reserved entry 0..3 by key index, group
//! keys past entry 3 are refused, and removal clears the right entry. Driven
//! through the real `nonos_wifi_core::key::KeyStore` trait against a modeled CAM.

use std::cell::RefCell;
use std::rc::Rc;

use crate::link::RtlKeys;
use crate::regs::Mmio;
use nonos_wifi_core::key::KeyStore;

const SEC_CMD: usize = 0x0670;
const WRITE_ENABLE: u32 = 1 << 16;
const POLLING: u32 = 1 << 31;

// A modeled register window recording the CAM command writes. The command log is
// shared so a test can read it after the store has taken ownership of the window.
#[derive(Clone)]
struct MockMmio {
    cmds: Rc<RefCell<Vec<u32>>>,
}

impl MockMmio {
    fn new() -> Self {
        Self { cmds: Rc::new(RefCell::new(Vec::new())) }
    }
    // The CAM cells (entry base + cell index) addressed so far.
    fn cells(&self) -> Vec<u32> {
        self.cmds.borrow().iter().map(|c| c & !(WRITE_ENABLE | POLLING)).collect()
    }
    fn last_cell(&self) -> u32 {
        *self.cmds.borrow().last().unwrap() & !(WRITE_ENABLE | POLLING)
    }
}

impl Mmio for MockMmio {
    fn read8(&self, _o: usize) -> u8 {
        0
    }
    fn write8(&self, _o: usize, _v: u8) {}
    fn read16(&self, _o: usize) -> u16 {
        0
    }
    fn write16(&self, _o: usize, _v: u16) {}
    fn read32(&self, _o: usize) -> u32 {
        0
    }
    fn write32(&self, off: usize, val: u32) {
        if off == SEC_CMD {
            self.cmds.borrow_mut().push(val);
        }
    }
}

#[test]
fn a_pairwise_key_lands_in_an_entry_from_four_up() {
    let m = MockMmio::new();
    let mut keys = RtlKeys::new(m.clone());
    let peer = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
    assert!(keys.install_ptk(&[0x11; 16], 0, &peer));
    // Eight cells at CAM entry 4: base 4 << 3 = 32, cells 32..=39.
    let c = m.cells();
    assert_eq!(c.len(), 8, "all eight cells written");
    assert!(c.iter().all(|&x| (32..=39).contains(&x)), "cells of entry 4");
}

#[test]
fn re_keying_a_peer_reuses_its_slot() {
    let m = MockMmio::new();
    let mut keys = RtlKeys::new(m.clone());
    let a = [0xA0; 6];
    let b = [0xB0; 6];
    assert!(keys.install_ptk(&[0; 16], 0, &a)); // entry 4
    assert!(keys.install_ptk(&[0; 16], 0, &b)); // entry 5
    assert!(keys.install_ptk(&[1; 16], 0, &a)); // reuses entry 4
                                                // Peer a's second install addresses entry 4 again (base 32), not a new slot.
    let last8: Vec<u32> = m.cells().into_iter().rev().take(8).collect();
    assert!(last8.iter().all(|&x| (32..=39).contains(&x)), "peer a reuses entry 4");
}

#[test]
fn a_group_key_lands_in_its_reserved_entry() {
    let m = MockMmio::new();
    let mut keys = RtlKeys::new(m.clone());
    assert!(keys.install_gtk(&[0x22; 16], 2)); // group key index 2 -> CAM entry 2
                                               // Entry 2: base 2 << 3 = 16, cells 16..=23.
    assert!(m.cells().iter().all(|&x| (16..=23).contains(&x)), "cells of entry 2");
}

#[test]
fn a_group_key_index_past_the_reserved_range_is_refused() {
    let m = MockMmio::new();
    let mut keys = RtlKeys::new(m.clone());
    assert!(!keys.install_gtk(&[0; 16], 4), "group keys only use entries 0..3");
    assert!(m.cmds.borrow().is_empty(), "nothing written on refusal");
}

#[test]
fn removing_a_pairwise_key_clears_its_entry_and_frees_the_slot() {
    let m = MockMmio::new();
    let mut keys = RtlKeys::new(m.clone());
    let peer = [0xC0; 6];
    keys.install_ptk(&[0; 16], 0, &peer); // entry 4
    keys.remove_key(0, Some(&peer));
    assert_eq!(m.last_cell(), 32, "clear addresses cell 0 of entry 4");
    // The slot is free again: the next peer reuses entry 4.
    let other = [0xD0; 6];
    assert!(keys.install_ptk(&[0; 16], 0, &other));
}

#[test]
fn removing_a_group_key_clears_its_entry() {
    let m = MockMmio::new();
    let mut keys = RtlKeys::new(m.clone());
    keys.install_gtk(&[0; 16], 1);
    keys.remove_key(1, None);
    assert_eq!(m.last_cell(), 8, "clear addresses cell 0 of entry 1");
}
