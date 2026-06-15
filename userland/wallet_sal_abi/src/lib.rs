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

#![no_std]
#![forbid(unsafe_code)]

pub mod limits;
pub mod protocol;
pub mod types;

pub use protocol::{
    OP_ADDRESS, OP_BALANCE, OP_BROADCAST, OP_BUILD_SEND, OP_CLOSE, OP_CREATE, OP_HISTORY, OP_LOCK,
    OP_OPEN_SEALED, OP_PROOF, OP_RESTORE_KEYS, OP_RESTORE_SEED, OP_SIGN_SEND, OP_STATUS,
    OP_SYNC_STEP, OP_UNLOCK,
};
pub use types::{
    SalAddress, SalBalance, SalNetwork, SalProof, SalSession, SalStatus, SalSync, SalTxDraft,
    SalTxHash, SalWalletId,
};
