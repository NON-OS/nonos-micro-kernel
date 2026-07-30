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

//! Channel error types.

extern crate alloc;

use alloc::string::String;

/// Channel operation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelError {
    /// Channel not found
    NotFound { from: String, to: String },
    /// Queue is full
    QueueFull { queue_size: usize, max_size: usize },
    /// Message too large
    MessageTooLarge { size: usize, max: usize },
    /// Channel already exists
    AlreadyExists { from: String, to: String },
    /// Invalid channel endpoints
    InvalidEndpoints,
    /// Message integrity check failed
    IntegrityCheckFailed,
}

impl ChannelError {
    /// Get a short description of the error
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::NotFound { .. } => "Channel not found",
            Self::QueueFull { .. } => "Queue full",
            Self::MessageTooLarge { .. } => "Message too large",
            Self::AlreadyExists { .. } => "Channel exists",
            Self::InvalidEndpoints => "Invalid endpoints",
            Self::IntegrityCheckFailed => "Integrity check failed",
        }
    }
}

impl core::fmt::Display for ChannelError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotFound { from, to } => {
                write!(f, "Channel not found: {} -> {}", from, to)
            }
            Self::QueueFull { queue_size, max_size } => {
                write!(f, "Queue full: {}/{} messages", queue_size, max_size)
            }
            Self::MessageTooLarge { size, max } => {
                write!(f, "Message too large: {} bytes (max: {})", size, max)
            }
            Self::AlreadyExists { from, to } => {
                write!(f, "Channel already exists: {} -> {}", from, to)
            }
            Self::InvalidEndpoints => write!(f, "Invalid channel endpoints"),
            Self::IntegrityCheckFailed => write!(f, "Message integrity check failed"),
        }
    }
}
