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

pub const UART_CLOCK: u32 = 1_843_200;

pub const COM1_BASE: u16 = 0x3F8;
pub const COM2_BASE: u16 = 0x2F8;
pub const COM3_BASE: u16 = 0x3E8;
pub const COM4_BASE: u16 = 0x2E8;

pub const COM1_IRQ: u8 = 4;
pub const COM2_IRQ: u8 = 3;
pub const COM3_IRQ: u8 = 4;
pub const COM4_IRQ: u8 = 3;

pub const MAX_COM_PORTS: usize = 4;
pub const RX_BUFFER_SIZE: usize = 256;
pub const TX_TIMEOUT: u32 = 100_000;

pub const REG_DATA: u16 = 0;
pub const REG_IER: u16 = 1;
pub const REG_IIR_FCR: u16 = 2;
pub const REG_LCR: u16 = 3;
pub const REG_MCR: u16 = 4;
pub const REG_LSR: u16 = 5;
pub const REG_MSR: u16 = 6;

pub const LSR_DATA_READY: u8 = 1 << 0;
pub const LSR_OVERRUN_ERR: u8 = 1 << 1;
pub const LSR_PARITY_ERR: u8 = 1 << 2;
pub const LSR_FRAMING_ERR: u8 = 1 << 3;
pub const LSR_BREAK_INT: u8 = 1 << 4;
pub const LSR_TX_EMPTY: u8 = 1 << 5;
pub const LSR_FIFO_ERR: u8 = 1 << 7;

pub const MCR_DTR: u8 = 1 << 0;
pub const MCR_RTS: u8 = 1 << 1;
pub const MCR_OUT2: u8 = 1 << 3;
pub const MCR_LOOPBACK: u8 = 1 << 4;

pub const LCR_DLAB: u8 = 1 << 7;
pub const LCR_PARITY_STICKY: u8 = 1 << 5;
pub const LCR_PARITY_EVEN: u8 = 1 << 4;
pub const LCR_PARITY_ENABLE: u8 = 1 << 3;

pub const IER_RX_AVAIL: u8 = 1 << 0;
pub const IER_LINE_STATUS: u8 = 1 << 2;

pub const FCR_ENABLE: u8 = 1 << 0;
pub const FCR_RX_CLEAR: u8 = 1 << 1;
pub const FCR_TX_CLEAR: u8 = 1 << 2;
pub const FCR_TRIGGER_14: u8 = 3 << 6;

pub const IIR_NO_INT: u8 = 1 << 0;
pub const IIR_ID_MASK: u8 = 0x0E;
