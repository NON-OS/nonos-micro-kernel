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

//! `TPM2_FlushContext`. The handle is a parameter rather than a handle: the
//! command is about the slot, not an object it authorises against.

use alloc::vec::Vec;

use super::consts::{TPM_CC_FLUSH_CONTEXT, TPM_ST_NO_SESSIONS};
use super::wire::frame;

pub(super) fn build_flush(handle: u32) -> Vec<u8> {
    frame(TPM_ST_NO_SESSIONS, TPM_CC_FLUSH_CONTEXT, &handle.to_be_bytes())
}
