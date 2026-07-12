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

mod consume;
mod create;
mod error;
mod limits;
mod material;
mod nonce;
mod nonce_compose;
mod quota;
mod sign;
mod token_type;
mod token_usage;
mod verify;

pub use consume::{
    refund_bytes, refund_ops, reset_token, try_consume, try_consume_bytes, try_consume_ops,
};
pub use create::{create_resource_token, create_resource_token_with_nonce};
pub use error::ResourceError;
pub use material::{compute_signature, token_material};
pub use nonce::{next_nonce, reset_nonce_counter};
pub use quota::ResourceQuota;
pub use sign::sign_resource_token;
pub use token_type::ResourceToken;
pub use verify::{verify_resource_token, verify_resource_token_strict};
