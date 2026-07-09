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

use crate::capabilities::Capability;
use crate::process::Pid;

// A parent pid attributed to a capsule spawn on behalf of another process.
// The field is private to this module, so the only way to obtain one is
// `attest`, which is the single chokepoint that checks the caller holds
// `Capability::SpawnBroker` and that the attributed pid still names a live
// process. No downstream spawn code can hand a raw, unchecked pid to
// `create_process_with_parent` without having passed both checks here.
#[derive(Clone, Copy)]
pub(crate) struct AttestedParent(Pid);

impl AttestedParent {
    // Fails closed to `None` (caller-as-parent, the default attribution) on
    // any of: a zero/absent raw pid, a caller without `SpawnBroker`, or a
    // `raw` that does not name a currently live process. Pids never recycle
    // in this kernel, so a dead `raw` can only mean the requester already
    // exited; attributing to it would create a permanently unkillable,
    // unwaitable orphan instead of a merely misattributed one.
    pub(crate) fn attest(raw: u32) -> Option<Self> {
        if raw == 0 {
            return None;
        }
        let caller = crate::process::current_pid()?;
        if !crate::process::caps::has(caller, Capability::SpawnBroker.bit()) {
            return None;
        }
        crate::process::get_process(raw)?;
        Some(Self(raw))
    }

    pub(crate) fn pid(self) -> Pid {
        self.0
    }
}
