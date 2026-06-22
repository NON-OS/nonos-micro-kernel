// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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
pub enum BindingError {
    KernelHashMismatch,
    NonceMismatch,
    MachineIdMismatch,
    TimestampExpired,
    TimestampInFuture,
    CommitmentMismatch,
    NonceNotInitialized,
    MachineIdNotInitialized,
    PublicInputsMalformed,
}

impl BindingError {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::KernelHashMismatch => "ZK binding: kernel hash does not match proof",
            Self::NonceMismatch => "ZK binding: boot nonce does not match proof",
            Self::MachineIdMismatch => "ZK binding: machine ID does not match proof",
            Self::TimestampExpired => "ZK binding: proof timestamp expired",
            Self::TimestampInFuture => "ZK binding: proof timestamp is in the future",
            Self::CommitmentMismatch => "ZK binding: capsule commitment mismatch",
            Self::NonceNotInitialized => "ZK binding: boot nonce not initialized",
            Self::MachineIdNotInitialized => "ZK binding: machine ID not initialized",
            Self::PublicInputsMalformed => "ZK binding: public inputs malformed",
        }
    }
}
