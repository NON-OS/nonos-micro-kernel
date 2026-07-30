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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SerialError {
    None = 0,
    NotInitialized = 1,
    AlreadyInitialized = 2,
    PortNotPresent = 3,
    InvalidPort = 4,
    InvalidBaudRate = 5,
    TransmitTimeout = 6,
    ReceiveTimeout = 7,
    BufferOverflow = 8,
    ParityError = 9,
    FramingError = 10,
    OverrunError = 11,
    BreakDetected = 12,
    FifoError = 13,
}

impl SerialError {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "no error",
            Self::NotInitialized => "serial port not initialized",
            Self::AlreadyInitialized => "serial port already initialized",
            Self::PortNotPresent => "serial port not present or faulty",
            Self::InvalidPort => "invalid COM port number",
            Self::InvalidBaudRate => "invalid baud rate",
            Self::TransmitTimeout => "transmit timeout",
            Self::ReceiveTimeout => "receive timeout",
            Self::BufferOverflow => "receive buffer overflow",
            Self::ParityError => "parity error detected",
            Self::FramingError => "framing error detected",
            Self::OverrunError => "receiver overrun error",
            Self::BreakDetected => "break condition detected",
            Self::FifoError => "FIFO error detected",
        }
    }

    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::NotInitialized,
            2 => Self::AlreadyInitialized,
            3 => Self::PortNotPresent,
            4 => Self::InvalidPort,
            5 => Self::InvalidBaudRate,
            6 => Self::TransmitTimeout,
            7 => Self::ReceiveTimeout,
            8 => Self::BufferOverflow,
            9 => Self::ParityError,
            10 => Self::FramingError,
            11 => Self::OverrunError,
            12 => Self::BreakDetected,
            13 => Self::FifoError,
            _ => Self::None,
        }
    }
}
