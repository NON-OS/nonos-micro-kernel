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

//! The host-to-card (H2C) mailbox: how the driver hands short commands to the
//! firmware running on the on-chip 8051 (media status, rate-adaptation info, PS
//! mode and so on). There are four mailboxes used round-robin; each command is
//! an eight-byte message written as an extended word then a main word, but only
//! once the firmware has drained that mailbox (its bit in the transfer register
//! reads clear). This follows rtw88 `rtw_fw_send_h2c_command` in `fw.c`; the box
//! selection, the free-wait and the register writes are checked against a modeled
//! device in `rtl8821ce_proofs`.

use crate::regs::Mmio;

/// `REG_HMETFR`: one bit per mailbox, set while the firmware still owns it.
const REG_HMETFR: usize = 0x01CC;
/// `REG_HMEBOX0`: the first mailbox's main word; the others follow every 4 bytes.
const REG_HMEBOX0: usize = 0x01D0;
/// `REG_HMEBOX0_EX`: the first mailbox's extended word; others every 4 bytes.
const REG_HMEBOX0_EX: usize = 0x01F0;
/// The number of mailboxes.
const BOX_COUNT: u8 = 4;
/// Reads allowed before a mailbox is declared stuck (firmware not draining).
const FREE_POLL_LIMIT: u32 = 100_000;

/// The round-robin mailbox cursor.
#[derive(Clone, Copy, Default)]
pub struct H2c {
    box_num: u8,
}

impl H2c {
    pub const fn new() -> Self {
        Self { box_num: 0 }
    }

    /// Send one eight-byte command (`msg` low word, `msg_ext` high word) to the
    /// firmware. Returns false if the next mailbox never drains.
    pub fn send<M: Mmio>(&mut self, mmio: &M, msg: u32, msg_ext: u32) -> bool {
        let b = self.box_num as usize;
        if !self.box_free(mmio) {
            return false;
        }
        mmio.write32(REG_HMEBOX0_EX + b * 4, msg_ext);
        mmio.write32(REG_HMEBOX0 + b * 4, msg);
        self.box_num = (self.box_num + 1) % BOX_COUNT;
        true
    }

    // A mailbox is free once its bit in the transfer register reads clear.
    fn box_free<M: Mmio>(&self, mmio: &M) -> bool {
        for _ in 0..FREE_POLL_LIMIT {
            if (mmio.read8(REG_HMETFR) >> self.box_num) & 0x1 == 0 {
                return true;
            }
            core::hint::spin_loop();
        }
        false
    }
}
