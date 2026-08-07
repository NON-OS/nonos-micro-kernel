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

mod decode;
mod encode;
mod errno;
mod types;

pub use decode::decode_request;
pub use encode::encode_response;
pub use errno::{EACCES, EBUSY, EINVAL, ENOENT, ENOSPC};
pub use types::{
    Request, KERNEL_REPLY_ENDPOINT, OP_COUNT, OP_DELETE, OP_LIST_WALLET_RAILS, OP_LOCK,
    OP_METADATA, OP_RETRIEVE, OP_SIGN_ETH_TRANSFER, OP_SIGN_NOX_APPROVE, OP_SIGN_NOX_RECEIPT,
    OP_SIGN_NOX_STAKE, OP_SIGN_NOX_STAKE_APPROVE, OP_SIGN_NOX_STAKE_LOCKED, OP_SIGN_NOX_TRANSFER,
    OP_SIGN_NOX_UNSTAKE, OP_STORE, OP_UNLOCK, OP_WALLET_ADDRESS, OP_WALLET_EXPORT,
    OP_WALLET_GENERATE, OP_WALLET_GENERATE_HD, OP_WALLET_IMPORT, OP_WALLET_RECOVER,
};
