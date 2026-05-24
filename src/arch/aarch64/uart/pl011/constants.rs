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

pub const UARTDR: u64 = 0x000;
pub const UARTFR: u64 = 0x018;
pub const UARTIBRD: u64 = 0x024;
pub const UARTFBRD: u64 = 0x028;
pub const UARTLCR_H: u64 = 0x02C;
pub const UARTCR: u64 = 0x030;
pub const UARTIMSC: u64 = 0x038;
pub const UARTMIS: u64 = 0x040;
pub const UARTICR: u64 = 0x044;

pub const FR_BUSY: u32 = 1 << 3;
pub const FR_RXFE: u32 = 1 << 4;
pub const FR_TXFF: u32 = 1 << 5;

pub const CR_UARTEN: u32 = 1 << 0;
pub const CR_TXE: u32 = 1 << 8;
pub const CR_RXE: u32 = 1 << 9;

pub const LCR_FEN: u32 = 1 << 4;
pub const LCR_WLEN_8: u32 = 0b11 << 5;

pub const IMSC_RXIM: u32 = 1 << 4;
pub const INTERRUPT_CLEAR_ALL: u32 = 0x7FF;
