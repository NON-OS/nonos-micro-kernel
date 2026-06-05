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

//! IPC message with integrity checksum.

extern crate alloc;

use alloc::{string::String, vec::Vec};

use super::hash::{compute_checksum, verify_checksum};

/// Maximum message payload size (1 MB)
pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024;

/// IPC message with integrity checksum
///
/// Messages are validated using a BLAKE3-based checksum that covers
/// the sender, receiver, payload, and timestamp.
#[derive(Debug, Clone)]
pub struct IpcMessage {
    /// Source module identifier
    pub from: String,
    /// Destination module identifier
    pub to: String,
    /// Message payload
    pub data: Vec<u8>,
    /// Creation timestamp (milliseconds since boot)
    pub timestamp_ms: u64,
    /// Kernel request/reply correlation token (0 = uncorrelated send)
    pub correlation: u64,
    /// Integrity checksum
    checksum64: u64,
}

impl IpcMessage {
    /// Create a new message with computed checksum
    pub fn new(from: &str, to: &str, data: &[u8]) -> Result<Self, &'static str> {
        if data.len() > MAX_MESSAGE_SIZE {
            return Err("Message payload too large");
        }

        let ts = crate::time::timestamp_millis();
        let csum = compute_checksum(from, to, data, ts);

        Ok(Self {
            from: String::from(from),
            to: String::from(to),
            data: data.to_vec(),
            timestamp_ms: ts,
            correlation: 0,
            checksum64: csum,
        })
    }

    /// Create a message with custom timestamp (for testing)
    pub fn with_timestamp(from: &str, to: &str, data: &[u8], ts: u64) -> Self {
        let csum = compute_checksum(from, to, data, ts);
        Self {
            from: String::from(from),
            to: String::from(to),
            data: data.to_vec(),
            timestamp_ms: ts,
            correlation: 0,
            checksum64: csum,
        }
    }

    /// Validate message integrity using constant-time comparison
    #[inline]
    pub fn validate_integrity(&self) -> bool {
        verify_checksum(&self.from, &self.to, &self.data, self.timestamp_ms, self.checksum64)
    }

    /// Get message age in milliseconds
    #[inline]
    pub fn age_ms(&self) -> u64 {
        crate::time::timestamp_millis().saturating_sub(self.timestamp_ms)
    }

    /// Get payload size
    #[inline]
    pub fn payload_size(&self) -> usize {
        self.data.len()
    }

    /// Check if message is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl core::fmt::Display for IpcMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "IpcMessage[{} -> {}, {} bytes, {}ms old]",
            self.from,
            self.to,
            self.data.len(),
            self.age_ms()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_message_display() {
        let msg = IpcMessage::with_timestamp("sender", "receiver", b"hello", 1000);
        let s = format!("{}", msg);
        assert!(s.contains("sender"));
        assert!(s.contains("receiver"));
        assert!(s.contains("5 bytes"));
    }

    #[test]
    fn test_message_checksum() {
        let msg = IpcMessage::with_timestamp("a", "b", b"test", 12345);
        assert!(msg.validate_integrity());
    }

    #[test]
    fn test_message_size_limit() {
        let large_data = alloc::vec![0u8; MAX_MESSAGE_SIZE + 1];
        let result = IpcMessage::new("a", "b", &large_data);
        assert!(result.is_err());
    }
}
