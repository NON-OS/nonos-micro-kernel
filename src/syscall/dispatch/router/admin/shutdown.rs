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

use crate::arch::power::PowerOff;
use crate::security::zerostate::terminate;

/// Power the machine off, wiping on the way.
///
/// Divergent, which is what the caller actually gets: the router needs an
/// arm for this syscall number but never reaches the point of returning from
/// it.
pub(super) fn shutdown() -> ! {
    terminate(PowerOff::Shutdown)
}
