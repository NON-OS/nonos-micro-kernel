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

use super::constants::UART_CLOCK;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BaudRate {
    Baud300 = 300,
    Baud1200 = 1200,
    Baud2400 = 2400,
    Baud4800 = 4800,
    Baud9600 = 9600,
    Baud19200 = 19200,
    Baud38400 = 38400,
    Baud57600 = 57600,
    Baud115200 = 115200,
}

impl BaudRate {
    pub const fn divisor(self) -> u16 {
        (UART_CLOCK / (16 * self as u32)) as u16
    }

    pub const fn from_divisor(divisor: u16) -> u32 {
        if divisor == 0 {
            return 0;
        }
        UART_CLOCK / (16 * divisor as u32)
    }

    pub const fn as_u32(self) -> u32 {
        self as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataBits {
    Five = 0,
    Six = 1,
    Seven = 2,
    Eight = 3,
}

impl DataBits {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub const fn bits(self) -> u8 {
        match self {
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Parity {
    None = 0,
    Odd = 1,
    Even = 3,
    Mark = 5,
    Space = 7,
}

impl Parity {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StopBits {
    One = 0,
    Two = 1,
}

impl StopBits {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SerialConfig {
    pub baud_rate: BaudRate,
    pub data_bits: DataBits,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub enable_fifo: bool,
    pub enable_interrupts: bool,
}

impl SerialConfig {
    pub const fn new(
        baud_rate: BaudRate,
        data_bits: DataBits,
        parity: Parity,
        stop_bits: StopBits,
    ) -> Self {
        Self { baud_rate, data_bits, parity, stop_bits, enable_fifo: true, enable_interrupts: true }
    }

    pub const fn with_fifo(mut self, enable: bool) -> Self {
        self.enable_fifo = enable;
        self
    }

    pub const fn with_interrupts(mut self, enable: bool) -> Self {
        self.enable_interrupts = enable;
        self
    }
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            baud_rate: BaudRate::Baud115200,
            data_bits: DataBits::Eight,
            parity: Parity::None,
            stop_bits: StopBits::One,
            enable_fifo: true,
            enable_interrupts: true,
        }
    }
}
