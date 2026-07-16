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

//! Walk a rtw88 PHY-condition table and hand each applicable register write to a
//! callback. The tables are address/value pairs where a real (4-aligned) address
//! is a register write, and a value with bit 31 set is a branch marker
//! (if/elif/else/endif) or with bit 30 set is a condition to evaluate against the
//! running chip (its cut version, package, host interface and RF front-end
//! option). Only writes inside a matching branch are applied. This reimplements
//! rtw88 `rtw_parse_tbl_phy_cond` and `check_positive` in `phy.c`; the branch
//! logic is checked against known-answer tables in `rtl8821ce_proofs`.

/// The running chip's condition, matched against the table's branch guards. Set
/// from the chip at bring-up (cut version and RF-front-end option come from the
/// efuse); the interface is PCIe for this adapter.
#[derive(Clone, Copy)]
pub struct PhyCond {
    pub cut: u8,
    pub pkg: u8,
    pub intf: u8,
    pub rfe: u8,
}

/// `INTF_PCIE`: the host-interface selector for a PCIe adapter.
pub const INTF_PCIE: u8 = 1;

// Marker bits within an address word.
const POS: u32 = 1 << 31; // a branch marker
const NEG: u32 = 1 << 30; // a condition to evaluate
const BRANCH_ELSE: u32 = 2;
const BRANCH_ENDIF: u32 = 3;

fn branch(addr: u32) -> u32 {
    (addr >> 28) & 0x3
}
fn cond_cut(addr: u32) -> u8 {
    ((addr >> 24) & 0xF) as u8
}
fn cond_pkg(addr: u32) -> u8 {
    ((addr >> 12) & 0xF) as u8
}
fn cond_intf(addr: u32) -> u8 {
    ((addr >> 8) & 0xF) as u8
}
fn cond_rfe(addr: u32) -> u8 {
    (addr & 0xFF) as u8
}

// A branch guard matches when each present field equals the chip's, and the RF
// front-end option matches exactly (rtw88 `check_positive`).
fn check_positive(guard: u32, drv: &PhyCond) -> bool {
    if cond_cut(guard) != 0 && cond_cut(guard) != drv.cut {
        return false;
    }
    if cond_pkg(guard) != 0 && cond_pkg(guard) != drv.pkg {
        return false;
    }
    if cond_intf(guard) != 0 && cond_intf(guard) != drv.intf {
        return false;
    }
    cond_rfe(guard) == drv.rfe
}

/// Apply `table` for the chip described by `drv`, calling `cfg(addr, data)` for
/// each register write inside a matching branch.
pub fn apply<F: FnMut(u32, u32)>(table: &[u32], drv: &PhyCond, mut cfg: F) {
    let mut is_matched = true;
    let mut is_skipped = false;
    let mut pos_cond = 0u32;

    for pair in table.chunks_exact(2) {
        let (addr, data) = (pair[0], pair[1]);
        if addr & POS != 0 {
            match branch(addr) {
                BRANCH_ENDIF => {
                    is_matched = true;
                    is_skipped = false;
                }
                BRANCH_ELSE => {
                    is_matched = !is_skipped;
                }
                // if / elif: remember the guard for the following condition.
                _ => pos_cond = addr,
            }
        } else if addr & NEG != 0 {
            if is_skipped {
                is_matched = false;
            } else if check_positive(pos_cond, drv) {
                is_matched = true;
                is_skipped = true;
            } else {
                is_matched = false;
                is_skipped = false;
            }
        } else if is_matched {
            cfg(addr, data);
        }
    }
}
