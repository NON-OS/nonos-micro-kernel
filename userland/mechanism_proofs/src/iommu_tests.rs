// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The VT-d bit math, against the encodings in the Intel spec. None of this
//! needs a remapping unit, and getting it wrong is how a device ends up with
//! an address it was never granted.

use crate::iommu::cap::{
    caching_mode, domain_count, fault_recording_count, fault_recording_offset, max_address_width,
    preferred_levels, requires_write_buffer_flush, AgawLevels,
};
use crate::iommu::context::{
    context_domain, context_high, context_index, context_low, entry_address, is_present, root_low,
    ADDR_MASK, CONTEXT_ENTRIES,
};
use crate::iommu::sl_pte::{self, SL_READ, SL_WRITE};

#[test]
fn domain_count_follows_the_nd_encoding() {
    // 2^(4 + 2*ND), and 7 is reserved.
    for (nd, expected) in [(0u64, 16u32), (1, 64), (2, 256), (3, 1024), (4, 4096), (5, 16384)] {
        assert_eq!(domain_count(nd), expected, "ND {nd}");
    }
    assert_eq!(domain_count(7), 0, "reserved ND reports no domains");
}

#[test]
fn max_address_width_is_mgaw_plus_one() {
    // MGAW sits at bits 21:16 and stores width - 1.
    assert_eq!(max_address_width(38 << 16), 39);
    assert_eq!(max_address_width(47 << 16), 48);
    assert_eq!(max_address_width(56 << 16), 57);
}

#[test]
fn sagaw_picks_a_depth_the_unit_supports() {
    // SAGAW at bits 12:8: bit 1 is 3 level, bit 2 is 4 level, bit 3 is 5 level.
    assert_eq!(preferred_levels(0b0010 << 8), Some(AgawLevels::Three));
    assert_eq!(preferred_levels(0b0100 << 8), Some(AgawLevels::Four));
    assert_eq!(preferred_levels(0b1000 << 8), Some(AgawLevels::Five));
    // Four levels is preferred when the unit offers a choice.
    assert_eq!(preferred_levels(0b1110 << 8), Some(AgawLevels::Four));
    // A unit supporting none cannot translate, and must not be guessed at.
    assert_eq!(preferred_levels(0), None);
}

#[test]
fn agaw_levels_match_their_context_encoding() {
    assert_eq!(AgawLevels::Three.page_table_levels(), 3);
    assert_eq!(AgawLevels::Four.page_table_levels(), 4);
    assert_eq!(AgawLevels::Five.page_table_levels(), 5);
    assert_eq!(AgawLevels::Three.context_aw(), 1);
    assert_eq!(AgawLevels::Four.context_aw(), 2);
    assert_eq!(AgawLevels::Five.context_aw(), 3);
}

#[test]
fn the_flush_and_caching_flags_read_their_own_bits() {
    assert!(requires_write_buffer_flush(1 << 4));
    assert!(!requires_write_buffer_flush(!(1u64 << 4)));
    assert!(caching_mode(1 << 7));
    assert!(!caching_mode(!(1u64 << 7)));
}

#[test]
fn fault_recording_fields_decode() {
    // FRO at 33:24 counts 16-byte units; NFR at 47:40 stores count - 1.
    assert_eq!(fault_recording_offset(0x20 << 24), 0x200);
    assert_eq!(fault_recording_count(0 << 40), 1);
    assert_eq!(fault_recording_count(7 << 40), 8);
}

#[test]
fn a_zero_second_level_entry_denies() {
    // There is no present bit: an entry is present when it grants access, so
    // a freshly zeroed table blocks rather than passing traffic through.
    assert!(!sl_pte::is_present(0));
    assert!(sl_pte::is_present(SL_READ));
    assert!(sl_pte::is_present(SL_WRITE));
}

#[test]
fn a_leaf_carries_only_the_permissions_asked_for() {
    let phys = 0x1234_5000u64;
    let ro = sl_pte::leaf(phys, true, false, false);
    assert_eq!(sl_pte::entry_address(ro), phys);
    assert_eq!(ro & SL_WRITE, 0, "read only leaf must not grant write");
    let rw = sl_pte::leaf(phys, true, true, false);
    assert_eq!(rw & SL_WRITE, SL_WRITE);
    let none = sl_pte::leaf(phys, false, false, false);
    assert!(!sl_pte::is_present(none), "no permission means not present");
}

#[test]
fn a_misaligned_frame_cannot_reach_the_permission_bits() {
    // The address is masked, not trusted. Otherwise low bits of a bad frame
    // would be read back as read/write grants.
    let e = sl_pte::leaf(0x1234_5FFF, false, false, false);
    assert_eq!(sl_pte::entry_address(e), 0x1234_5000);
    assert!(!sl_pte::is_present(e));
}

#[test]
fn level_indexing_walks_nine_bits_at_a_time() {
    let addr = 0x0000_1234_5678_9000u64;
    for level in 1..=4u8 {
        let shift = 12 + 9 * (level as u32 - 1);
        assert_eq!(sl_pte::index_for(addr, level), ((addr >> shift) & 0x1FF) as usize);
        assert!(sl_pte::index_for(addr, level) < sl_pte::ENTRIES);
    }
    assert_eq!(sl_pte::level_span(1), 4096);
    assert_eq!(sl_pte::level_span(2), 4096 * 512);
    assert_eq!(sl_pte::level_span(3), 4096 * 512 * 512);
}

#[test]
fn an_address_beyond_the_unit_width_is_refused() {
    assert!(sl_pte::fits_address_width(0xFFFF_FFFF, 39));
    assert!(!sl_pte::fits_address_width(1 << 39, 39));
    assert!(sl_pte::fits_address_width((1 << 48) - 1, 48));
    assert!(!sl_pte::fits_address_width(1 << 48, 48));
}

#[test]
fn a_zeroed_root_or_context_entry_blocks_the_device() {
    assert!(!is_present(0), "a zeroed table must not pass traffic");
    assert!(is_present(root_low(0x9000)));
    assert!(is_present(context_low(0x9000)));
}

#[test]
fn context_entries_carry_the_domain_and_table_they_name() {
    let sl_root = 0x4321_0000u64;
    let low = context_low(sl_root);
    assert_eq!(entry_address(low), sl_root);
    for domain in [0u16, 1, 255, 4095, u16::MAX] {
        let high = context_high(domain, AgawLevels::Four.context_aw());
        assert_eq!(context_domain(high), domain, "domain {domain}");
        assert_eq!(high & 0x7, 2, "address width for four levels");
    }
}

#[test]
fn a_source_id_lands_in_exactly_one_context_slot() {
    let mut seen = [false; CONTEXT_ENTRIES];
    for device in 0..32u8 {
        for function in 0..8u8 {
            let idx = context_index(device, function);
            assert!(idx < CONTEXT_ENTRIES);
            assert!(!seen[idx], "device {device} function {function} collided");
            seen[idx] = true;
        }
    }
    assert!(seen.iter().all(|&s| s), "every slot is reachable");
}

#[test]
fn entry_pointers_stay_page_aligned() {
    // The mask is what keeps a pointer from overlapping its own flag bits.
    assert_eq!(ADDR_MASK & 0xFFF, 0);
    assert_eq!(entry_address(root_low(0x1FFF)), 0x1000);
}
