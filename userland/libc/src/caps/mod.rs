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

//! The capability syscalls, which every capsule is governed by and none could
//! call.
//!
//! Every syscall this kernel dispatches is checked against the caller's mask,
//! and the mask itself was invisible to the process holding it: the three
//! calls that ask about, extend and withdraw authority were implemented in the
//! kernel and reachable from no capsule. A program learned it lacked a
//! capability by being refused, which is the wrong time to find out.

mod check;
mod grant;
mod revoke;

pub use check::mk_cap_check;
pub use grant::mk_cap_grant;
pub use revoke::mk_cap_revoke;
