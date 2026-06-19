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

//! Capability inheritance policy for new PCBs.
//!
//! Syscall authority is now decided through `pcb.capability_token`;
//! `pcb.caps_bits` is the derived bitmap cache. This module produces
//! the inherited bitmap that seeds both — the cache directly, and
//! the initial token via `process::caps::new_token`. The bits stay
//! in the `crate::capabilities::Capability` namespace.
//!
//! Two policy knobs live here:
//!   - `AMBIENT_CAPS`: init's production-ambient set, also the
//!     upper bound on inheritance. Hardware authority (Driver,
//!     DeviceEnum, Mmio, Irq, Dma, Pio), Admin, Debug, and the
//!     graphics caps are never ambient. They must be granted by
//!     each capsule's spawn spec.
//! A compile-time assertion below proves `AMBIENT_CAPS` carries
//! none of the forbidden bits.

use core::sync::atomic::Ordering;

use super::super::types::Pid;
use super::types::PROCESS_TABLE;
use crate::capabilities::Capability;

// Every process needs these to live: invoke core lifecycle syscalls
// (`MkExit`, `MkYield`), talk to capsules over IPC, and allocate
// user memory through `MkMmap`. Anything beyond this set — hardware
// authority, debug, admin, graphics — is granted per-capsule by the
// spawner. `RegisterService` and `Network`/`FileSystem`/`Crypto`/
// `Hardware` are not part of the active syscall surface today and
// are deliberately excluded from the ambient.
const AMBIENT_CAPS: u64 =
    Capability::CoreExec.bit() | Capability::IPC.bit() | Capability::Memory.bit();

// Bits that must never appear in `AMBIENT_CAPS` in any production
// build. Hardware authority and graphics flow through explicit
// grants only; Admin and Debug are out of the ambient set entirely.
const FORBIDDEN_AMBIENT: u64 = Capability::Admin.bit()
    | Capability::Driver.bit()
    | Capability::DeviceEnum.bit()
    | Capability::Mmio.bit()
    | Capability::Irq.bit()
    | Capability::Dma.bit()
    | Capability::Pio.bit()
    | Capability::Debug.bit()
    | Capability::GraphicsDisplayQuery.bit()
    | Capability::GraphicsSurfaceCreate.bit()
    | Capability::GraphicsSurfaceMap.bit()
    | Capability::GraphicsPresent.bit();

const _: () = assert!(
    AMBIENT_CAPS & FORBIDDEN_AMBIENT == 0,
    "AMBIENT_CAPS must not include Admin/Driver/DeviceEnum/Mmio/Irq/Dma/Pio/Debug/Graphics*"
);

pub(super) fn compute_inherited_caps(pid: Pid, parent_pid: Pid) -> u64 {
    if pid == 1 {
        return apply_inherit_bound(AMBIENT_CAPS);
    }
    match PROCESS_TABLE.find_by_pid(parent_pid) {
        Some(parent) => apply_inherit_bound(parent.caps_bits.load(Ordering::Acquire)),
        None => 0,
    }
}

// Sole producer of inherited caps. Both `compute_inherited_caps`
// and the fork/clone path go through here.
pub(crate) fn apply_inherit_bound(parent_caps: u64) -> u64 {
    parent_caps & inheritable_bound()
}

// Children inherit only the small ambient set. Hardware and admin
// authority do not ride inheritance; spawners that need them carry
// explicit `caps_bits` in their `CapsuleSpec`.
fn inheritable_bound() -> u64 {
    AMBIENT_CAPS
}
