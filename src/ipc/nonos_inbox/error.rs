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

//! Inbox error types.

extern crate alloc;

use alloc::string::String;

/// Inbox operation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InboxError {
    /// Inbox not found for the specified module
    NotFound { module: String },
    /// Inbox is full, cannot enqueue
    Full { module: String, capacity: usize },
    /// Enqueue operation timed out
    Timeout { module: String, waited_ms: u64 },
    /// Invalid capacity value
    InvalidCapacity { value: usize, min: usize, max: usize },
    /// Module name is empty
    EmptyModuleName,
    /// An inbox with this name is already registered
    AlreadyRegistered { module: String },
}

/// Result of `try_enqueue_strict`. Distinguishes the four routes a
/// strict enqueue can fail through; a caller can map each one to a
/// distinct errno (ENOENT for missing, ESRCH for dead owner, EAGAIN
/// for full).
#[derive(Debug)]
pub enum StrictEnqueueError {
    /// No inbox is registered under this name.
    MissingInbox,
    /// The inbox exists but its owner pid is no longer in the
    /// process table. Teardown for the owner has already started or
    /// completed; the dead pid will not drain this queue.
    DeadOwner,
    /// The inbox is full. The original message is returned so the
    /// caller can retry, drop, or surface it.
    QueueFull(crate::ipc::nonos_channel::IpcMessage),
}

impl InboxError {
    /// Get a short description of the error
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::NotFound { .. } => "Inbox not found",
            Self::Full { .. } => "Inbox full",
            Self::Timeout { .. } => "Enqueue timeout",
            Self::InvalidCapacity { .. } => "Invalid capacity",
            Self::EmptyModuleName => "Empty module name",
            Self::AlreadyRegistered { .. } => "Inbox already registered",
        }
    }
}

impl core::fmt::Display for InboxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotFound { module } => {
                write!(f, "Inbox not found for module '{}'", module)
            }
            Self::Full { module, capacity } => {
                write!(f, "Inbox full for module '{}' (capacity: {})", module, capacity)
            }
            Self::Timeout { module, waited_ms } => {
                write!(f, "Enqueue timeout for module '{}' after {}ms", module, waited_ms)
            }
            Self::InvalidCapacity { value, min, max } => {
                write!(f, "Invalid capacity {}: must be between {} and {}", value, min, max)
            }
            Self::EmptyModuleName => write!(f, "Module name cannot be empty"),
            Self::AlreadyRegistered { module } => {
                write!(f, "Inbox already registered for module '{}'", module)
            }
        }
    }
}
