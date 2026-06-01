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

mod admin;
mod crypto;
mod graphics;
mod mk;
#[cfg(test)]
mod tests;

use crate::capabilities::CapabilityToken;
use crate::syscall::numbers::SyscallNumber;

// Total cap-table over `SyscallNumber`. A number unclaimed by any
// family is refused by the trailing `unwrap_or(false)`. The legacy
// `hardware` family (IoPortRead/IoPortWrite/MmioMap) has been folded
// into the `mk` family — `MkPioRead/MkPioWrite/MkMmioMap` are the
// single source of truth.
pub(super) fn is_allowed(caps: &CapabilityToken, number: SyscallNumber) -> bool {
    crypto::check(caps, number)
        .or_else(|| admin::check(caps, number))
        .or_else(|| mk::check(caps, number))
        .or_else(|| graphics::check(caps, number))
        .unwrap_or(false)
}
