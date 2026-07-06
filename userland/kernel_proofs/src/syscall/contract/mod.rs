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

// The real syscall capability table. `is_allowed` is `pub(super)`, visible to
// this parent module; expose it for the proofs.
#[path = "../../../../../src/syscall/contract/cap_table/mod.rs"]
mod cap_table;

use crate::capabilities::CapabilityToken;
use crate::syscall::numbers::SyscallNumber;

pub fn is_allowed(caps: &CapabilityToken, number: SyscallNumber) -> bool {
    cap_table::is_allowed(caps, number)
}
