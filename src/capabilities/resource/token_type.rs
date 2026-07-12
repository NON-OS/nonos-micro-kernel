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

use super::error::ResourceError;
use super::quota::ResourceQuota;

#[derive(Debug, Clone)]
pub struct ResourceToken {
    pub owner_module: u64,
    pub(super) original_quota: ResourceQuota,
    pub(super) remaining_bytes: u64,
    pub(super) remaining_ops: u64,
    pub nonce: u64,
    pub signature: [u8; 64],
}

impl ResourceToken {
    #[inline]
    pub fn original_quota(&self) -> &ResourceQuota {
        &self.original_quota
    }
    #[inline]
    pub fn remaining_bytes(&self) -> u64 {
        self.remaining_bytes
    }
    #[inline]
    pub fn remaining_ops(&self) -> u64 {
        self.remaining_ops
    }
    #[inline]
    pub fn bytes_used(&self) -> u64 {
        self.original_quota.bytes.saturating_sub(self.remaining_bytes)
    }
    #[inline]
    pub fn ops_used(&self) -> u64 {
        self.original_quota.ops.saturating_sub(self.remaining_ops)
    }
    #[inline]
    pub fn is_expired(&self) -> bool {
        self.original_quota.is_expired()
    }
    #[inline]
    pub fn is_exhausted(&self) -> bool {
        self.remaining_bytes == 0 && self.remaining_ops == 0
    }
    #[inline]
    pub fn has_bytes(&self, amount: u64) -> bool {
        super::limits::has_at_least(self.remaining_bytes, amount)
    }
    #[inline]
    pub fn has_ops(&self, count: u64) -> bool {
        super::limits::has_at_least(self.remaining_ops, count)
    }
    pub fn remaining_ms(&self) -> Option<u64> {
        self.original_quota.remaining_ms()
    }
    pub(super) fn consume_bytes(&mut self, amount: u64) -> Result<(), ResourceError> {
        if self.remaining_bytes < amount {
            return Err(ResourceError::InsufficientBytes {
                requested: amount,
                available: self.remaining_bytes,
            });
        }
        self.remaining_bytes -= amount;
        Ok(())
    }
    pub(super) fn consume_ops(&mut self, count: u64) -> Result<(), ResourceError> {
        if self.remaining_ops < count {
            return Err(ResourceError::InsufficientOps {
                requested: count,
                available: self.remaining_ops,
            });
        }
        self.remaining_ops -= count;
        Ok(())
    }
}

impl core::fmt::Display for ResourceToken {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ResourceToken[owner:{} bytes:{}/{} ops:{}/{}]",
            self.owner_module,
            self.remaining_bytes,
            self.original_quota.bytes,
            self.remaining_ops,
            self.original_quota.ops
        )
    }
}
