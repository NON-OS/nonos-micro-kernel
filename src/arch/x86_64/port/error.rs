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

use super::constants::port_name;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortError {
    AccessDenied { port: u16 },
    PortReserved { port: u16 },
    InvalidRange { start: u16, end: u16 },
    ReadTimeout { port: u16 },
    WriteTimeout { port: u16 },
    BufferTooSmall { required: usize, provided: usize },
    NotInitialized,
}

impl PortError {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::AccessDenied { .. } => "Port access denied",
            Self::PortReserved { .. } => "Port is reserved",
            Self::InvalidRange { .. } => "Invalid port range",
            Self::ReadTimeout { .. } => "Port read timeout",
            Self::WriteTimeout { .. } => "Port write timeout",
            Self::BufferTooSmall { .. } => "Buffer too small for string I/O",
            Self::NotInitialized => "Port subsystem not initialized",
        }
    }
}

impl core::fmt::Display for PortError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AccessDenied { port } => {
                write!(f, "Port access denied: 0x{:04X} ({})", port, port_name(*port))
            }
            Self::PortReserved { port } => {
                write!(f, "Port 0x{:04X} ({}) is reserved", port, port_name(*port))
            }
            Self::InvalidRange { start, end } => {
                write!(f, "Invalid port range: 0x{:04X}-0x{:04X}", start, end)
            }
            Self::ReadTimeout { port } => {
                write!(f, "Port read timeout: 0x{:04X} ({})", port, port_name(*port))
            }
            Self::WriteTimeout { port } => {
                write!(f, "Port write timeout: 0x{:04X} ({})", port, port_name(*port))
            }
            Self::BufferTooSmall { required, provided } => {
                write!(f, "Buffer too small: need {} bytes, provided {}", required, provided)
            }
            Self::NotInitialized => {
                write!(f, "Port I/O subsystem not initialized")
            }
        }
    }
}
