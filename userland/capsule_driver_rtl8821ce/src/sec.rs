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

//! Install and clear keys in the hardware security CAM, and turn the engine on
//! for receive decryption. On the RTL8821CE the hardware decrypts received frames
//! from its key cache correctly, but its transmit encryption does not work without
//! a firmware MACID binding the driver does not set up, so transmit is encrypted
//! in software (see `link`/`station`) and only the receive path uses the engine.
//! A key occupies eight CAM cells written newest-cell-first through a write/command
//! register pair, each cell carrying the key index and type, the peer address or
//! four key bytes. This follows rtw88 `rtw_sec_write_cam` / `rtw_sec_clear_cam`
//! in `sec.c`; the exact cell contents and command program are checked in
//! `rtl8821ce_proofs`.

use crate::constants::regs::REG_CR;
use crate::regs::Mmio;

/// `RTW_SEC_CONFIG`: the security-engine configuration register.
const RTW_SEC_CONFIG: usize = 0x0680;
/// `RTW_SEC_ENGINE_EN`: the security-engine enable bit in `REG_CR`.
const RTW_SEC_ENGINE_EN: u32 = 1 << 9;
/// `RTW_SEC_CONFIG` bits: enable RX decryption (bit 3) with default-key search for
/// unicast and broadcast (bits 1, 7). TX encryption (bit 2) and its default-key
/// bits stay clear on purpose: transmit is encrypted in software, and leaving the
/// engine's TX crypto on corrupts an already-encrypted frame so the access point
/// drops it. The engine decrypts received frames by the sender's MACID-indexed
/// default key, which this chip does correctly.
const RTW_SEC_RX_UNI_USE_DK: u16 = 1 << 1;
const RTW_SEC_RX_DEC_EN: u16 = 1 << 3;
const RTW_SEC_RX_BC_USE_DK: u16 = 1 << 7;

/// `RTW_SEC_CMD_REG`: the CAM command register (address, enable, poll).
const RTW_SEC_CMD_REG: usize = 0x0670;
/// `RTW_SEC_WRITE_REG`: the CAM write-data register.
const RTW_SEC_WRITE_REG: usize = 0x0674;
/// `RTW_SEC_CAM_ENTRY_SHIFT`: each key occupies eight cells (`idx << 3`).
const RTW_SEC_CAM_ENTRY_SHIFT: u32 = 3;
/// `RTW_SEC_CMD_WRITE_ENABLE`.
const RTW_SEC_CMD_WRITE_ENABLE: u32 = 1 << 16;
/// `RTW_SEC_CMD_POLLING`.
const RTW_SEC_CMD_POLLING: u32 = 1 << 31;

/// `RTW_CAM_AES`: the CCMP (AES) hardware key type.
pub const CAM_AES: u8 = 4;

/// A key to install: the 16-byte cipher key, the peer MAC (broadcast for a group
/// key), the 802.11 key index and whether it is pairwise.
pub struct Key {
    pub key: [u8; 16],
    pub mac: [u8; 6],
    pub key_idx: u8,
    pub pairwise: bool,
}

/// Write `k` into CAM entry `hw_key_idx` with cipher `hw_key_type` (e.g.
/// [`CAM_AES`] for CCMP). Cells are written from 7 down to 0 so the valid bit in
/// cell 0 lands last, arming the entry only once it is fully written.
pub fn write_cam<M: Mmio>(mmio: &M, hw_key_idx: u8, hw_key_type: u8, k: &Key) {
    let base = (hw_key_idx as u32) << RTW_SEC_CAM_ENTRY_SHIFT;
    let cmd = RTW_SEC_CMD_WRITE_ENABLE | RTW_SEC_CMD_POLLING;
    let group = u32::from(!k.pairwise);

    for cell in (0..8u32).rev() {
        let content = match cell {
            0 => {
                (u32::from(k.key_idx) & 0x3)
                    | ((u32::from(hw_key_type) & 0x7) << 2)
                    | (group << 6)
                    | (1 << 15) // valid
                    | (u32::from(k.mac[0]) << 16)
                    | (u32::from(k.mac[1]) << 24)
            }
            1 => {
                u32::from(k.mac[2])
                    | (u32::from(k.mac[3]) << 8)
                    | (u32::from(k.mac[4]) << 16)
                    | (u32::from(k.mac[5]) << 24)
            }
            2..=5 => {
                let j = ((cell - 2) * 4) as usize;
                u32::from_le_bytes([k.key[j], k.key[j + 1], k.key[j + 2], k.key[j + 3]])
            }
            _ => 0, // cells 6 and 7 are unused for CCMP
        };
        mmio.write32(RTW_SEC_WRITE_REG, content);
        mmio.write32(RTW_SEC_CMD_REG, cmd | (base + cell));
    }
}

/// Turn on the hardware security engine for RECEIVE decryption only: set the
/// engine enable bit, then RX crypto-enable with default-key search. Received
/// frames decrypt against the CAM key resolved by the sender's MACID, which this
/// chip does correctly. TX crypto stays off because transmit is encrypted in
/// software; enabling it corrupts the already-encrypted frame. Called once the
/// four-way handshake has installed the keys, since before that the CAM is empty.
pub fn enable_sec_engine<M: Mmio>(mmio: &M) {
    mmio.write32(REG_CR, mmio.read32(REG_CR) | RTW_SEC_ENGINE_EN);
    let cfg = mmio.read16(RTW_SEC_CONFIG)
        | RTW_SEC_RX_DEC_EN
        | RTW_SEC_RX_UNI_USE_DK
        | RTW_SEC_RX_BC_USE_DK;
    mmio.write16(RTW_SEC_CONFIG, cfg);
}

/// Clear CAM entry `hw_key_idx` by zeroing its cell 0, which drops the valid bit
/// and disables the whole key.
pub fn clear_cam<M: Mmio>(mmio: &M, hw_key_idx: u8) {
    let base = (hw_key_idx as u32) << RTW_SEC_CAM_ENTRY_SHIFT;
    mmio.write32(RTW_SEC_WRITE_REG, 0);
    mmio.write32(RTW_SEC_CMD_REG, RTW_SEC_CMD_WRITE_ENABLE | RTW_SEC_CMD_POLLING | base);
}
