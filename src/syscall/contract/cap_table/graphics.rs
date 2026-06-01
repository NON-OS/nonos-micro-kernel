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

// Explicit graphics capability gates. Token must still be valid and
// admission is per-syscall instead of any valid token.

use crate::capabilities::CapabilityToken;
use crate::syscall::caps::Capability;
use crate::syscall::numbers::SyscallNumber;

pub(super) fn check(caps: &CapabilityToken, number: SyscallNumber) -> Option<bool> {
    if !caps.is_valid() {
        return Some(false);
    }
    let required = match number {
        SyscallNumber::GraphicsDisplayDimensions => Capability::GraphicsDisplayQuery,
        _ => return None,
    };
    Some(caps.grants(required))
}
