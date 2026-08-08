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

extern crate alloc;

use alloc::{string::String, vec::Vec};

use super::hash::{compute_checksum, verify_checksum};
use super::limits::MAX_MESSAGE_SIZE;

#[derive(Debug, Clone)]
pub struct IpcMessage {
    pub from: String,
    pub to: String,
    pub data: Vec<u8>,
    pub timestamp_ms: u64,
    pub correlation: u64,
    checksum64: u64,
}

impl IpcMessage {
    pub fn new(from: &str, to: &str, data: &[u8]) -> Result<Self, &'static str> {
        if data.len() > MAX_MESSAGE_SIZE {
            return Err("Message payload too large");
        }

        let ts = crate::time::timestamp_millis();
        let csum = compute_checksum(from, to, data, ts)?;

        Ok(Self {
            from: String::from(from),
            to: String::from(to),
            data: data.to_vec(),
            timestamp_ms: ts,
            correlation: 0,
            checksum64: csum,
        })
    }

    pub fn with_timestamp(
        from: &str,
        to: &str,
        data: &[u8],
        ts: u64,
    ) -> Result<Self, &'static str> {
        let csum = compute_checksum(from, to, data, ts)?;
        Ok(Self {
            from: String::from(from),
            to: String::from(to),
            data: data.to_vec(),
            timestamp_ms: ts,
            correlation: 0,
            checksum64: csum,
        })
    }

    #[inline]
    pub fn validate_integrity(&self) -> bool {
        verify_checksum(&self.from, &self.to, &self.data, self.timestamp_ms, self.checksum64)
    }

    #[inline]
    pub fn age_ms(&self) -> u64 {
        crate::time::timestamp_millis().saturating_sub(self.timestamp_ms)
    }

    #[inline]
    pub fn payload_size(&self) -> usize {
        self.data.len()
    }

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
