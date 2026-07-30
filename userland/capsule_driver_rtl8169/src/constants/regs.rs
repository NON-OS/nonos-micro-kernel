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

pub const REG_MAC0: usize = 0x00;
pub const REG_TXDESC_ADDR_LO: usize = 0x20;
pub const REG_TXDESC_ADDR_HI: usize = 0x24;
pub const REG_CMD: usize = 0x37;
pub const REG_TX_POLL: usize = 0x38;
pub const REG_IMR: usize = 0x3C;
pub const REG_ISR: usize = 0x3E;
pub const REG_TX_CONFIG: usize = 0x40;
pub const REG_RX_CONFIG: usize = 0x44;
/// EEPROM command register, which holds the config-write lock. The IDR bytes at
/// `REG_MAC0` ignore writes until it is unlocked.
pub const REG_CFG9346: usize = 0x50;
pub const CFG9346_UNLOCK: u8 = 0xC0;
pub const CFG9346_LOCK: u8 = 0x00;
pub const REG_PHY_STATUS: usize = 0x6C;
pub const REG_RMS: usize = 0xDA;
pub const REG_RXDESC_ADDR_LO: usize = 0xE4;
pub const REG_RXDESC_ADDR_HI: usize = 0xE8;

pub const CMD_RESET: u8 = 0x10;
pub const CMD_RX_ENABLE: u8 = 0x08;
pub const CMD_TX_ENABLE: u8 = 0x04;
pub const TX_POLL_HPQ: u8 = 0x80;

pub const TX_CONFIG_IFG: u32 = 3 << 24;
pub const TX_CONFIG_DMA: u32 = 7 << 8;
pub const PHY_STATUS_LINK_UP: u8 = 1 << 1;
pub const RX_CONFIG_ACCEPT_PHYS: u32 = 1 << 1;
pub const RX_CONFIG_ACCEPT_MULTI: u32 = 1 << 2;
pub const RX_CONFIG_ACCEPT_BCAST: u32 = 1 << 3;
pub const RX_CONFIG_DMA: u32 = 7 << 8;
pub const RX_CONFIG_MAXDMA: u32 = 7 << 13;

pub const ISR_ROK: u16 = 0x0001;
pub const ISR_RER: u16 = 0x0002;
pub const ISR_TOK: u16 = 0x0004;
pub const ISR_TER: u16 = 0x0008;
pub const ISR_ENABLED: u16 = ISR_ROK | ISR_RER | ISR_TOK | ISR_TER;

pub const DESC_OWN: u32 = 1 << 31;
pub const DESC_EOR: u32 = 1 << 30;
pub const DESC_FS: u32 = 1 << 29;
pub const DESC_LS: u32 = 1 << 28;
pub const DESC_LEN_MASK: u32 = 0x3FFF;
