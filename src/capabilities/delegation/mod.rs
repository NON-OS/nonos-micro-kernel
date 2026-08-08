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

mod create_checked;
mod create_unchecked;
mod error;
mod lifetime;
mod material;
mod sign;
mod types;
mod verify;

pub use create_checked::create_delegation;
pub use create_unchecked::create_delegation_unchecked;
pub use error::DelegationError;
pub use material::{compute_delegation_signature, delegation_material};
pub use sign::sign_delegation;
pub use types::Delegation;
pub use verify::{verify_delegation, verify_delegation_standalone, verify_delegation_strict};
