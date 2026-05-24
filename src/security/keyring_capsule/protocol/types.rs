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

pub(super) const OP_STORE: u16 = 1;
pub(super) const OP_RETRIEVE: u16 = 2;
pub(super) const OP_DELETE: u16 = 3;
pub(super) const OP_LOCK: u16 = 4;
pub(super) const OP_UNLOCK: u16 = 5;
pub(super) const OP_METADATA: u16 = 6;
pub(super) const OP_COUNT: u16 = 7;

pub(super) const HDR_LEN: usize = 8;
pub(super) const RESPONSE_HDR_LEN: usize = 8;

pub(in crate::security::keyring_capsule) const ERRNO_NOT_FOUND: i32 = -2;
pub(in crate::security::keyring_capsule) const ERRNO_ACCESS: i32 = -13;
pub(in crate::security::keyring_capsule) const ERRNO_BUSY: i32 = -16;
pub(in crate::security::keyring_capsule) const ERRNO_INVAL: i32 = -22;
pub(in crate::security::keyring_capsule) const ERRNO_NOSPC: i32 = -28;
