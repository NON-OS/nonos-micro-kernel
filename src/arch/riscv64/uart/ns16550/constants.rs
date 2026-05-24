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

pub const RBR: u64 = 0;
pub const THR: u64 = 0;
pub const IER: u64 = 1;
pub const FCR: u64 = 2;
pub const LCR: u64 = 3;
pub const MCR: u64 = 4;
pub const LSR: u64 = 5;
pub const DLL: u64 = 0;
pub const DLM: u64 = 1;

pub const LSR_DR: u8 = 1 << 0;
pub const LSR_THRE: u8 = 1 << 5;
pub const LSR_TEMT: u8 = 1 << 6;

pub const LCR_DLAB: u8 = 1 << 7;
pub const LCR_8N1: u8 = 0x03;

pub const FCR_ENABLE: u8 = 1 << 0;
pub const FCR_CLEAR_RX: u8 = 1 << 1;
pub const FCR_CLEAR_TX: u8 = 1 << 2;
pub const FCR_TRIGGER_14: u8 = 0xC0;

pub const IER_RDA: u8 = 1 << 0;

pub const MCR_DTR: u8 = 1 << 0;
pub const MCR_RTS: u8 = 1 << 1;
pub const MCR_OUT2: u8 = 1 << 3;
