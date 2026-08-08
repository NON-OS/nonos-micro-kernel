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

use crate::arch::interrupt_controller::{broadcast_ipi, Ipi};
use crate::arch::power::{enter, PowerOff};
use crate::security::hardening::zerostate_shutdown_wipe;

/// Wipe, then leave. The only way out of a running system.
///
/// The order is the whole point and it is not negotiable:
///
/// 1. Stop the other CPUs. A core still scheduling writes memory behind the
///    wipe, and whatever it writes survives into the next boot. Quiescing
///    first is what makes the wipe a snapshot rather than a race.
/// 2. Wipe. Process memory, the kernel heap, the key vault.
/// 3. Hand the machine to the firmware, which never returns.
///
/// The stop is best effort by design. On a uniprocessor boot there is nobody
/// to stop, and on a controller that refuses the broadcast the honest choice
/// is to wipe what this core can reach rather than to stay up holding
/// everything. A refusal costs coverage; refusing to shut down costs the lot.
pub(crate) fn terminate(off: PowerOff) -> ! {
    let _ = broadcast_ipi(Ipi::Stop);
    zerostate_shutdown_wipe();
    enter(off)
}
