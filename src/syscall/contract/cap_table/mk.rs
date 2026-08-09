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

use crate::capabilities::{Capability, CapabilityToken};
use crate::syscall::numbers::SyscallNumber;

pub(super) fn check(caps: &CapabilityToken, number: SyscallNumber) -> Option<bool> {
    Some(match number {
        SyscallNumber::MkExit
        | SyscallNumber::MkPidAlive
        | SyscallNumber::MkYield
        | SyscallNumber::MkFutexWait
        | SyscallNumber::MkFutexWake
        | SyscallNumber::MkTimeMillis
        | SyscallNumber::MkTimeMonotonic
        | SyscallNumber::MkTimeRtc
        | SyscallNumber::MkBatteryStatus
        | SyscallNumber::MkProcStat
        | SyscallNumber::MkAttestStatus
        | SyscallNumber::MkCapCheck => caps.is_valid(),

        SyscallNumber::MkTimeAdjust => caps.can_set_time(),

        SyscallNumber::MkMmap => caps.can_allocate_memory(),
        SyscallNumber::MkMunmap => caps.can_deallocate_memory(),

        SyscallNumber::MkCapsuleLoad => {
            caps.is_valid()
                && caps.grants_all(&[Capability::CoreExec, Capability::IPC, Capability::Memory])
        }

        SyscallNumber::MkCapsuleVerify => {
            caps.is_valid()
                && caps.grants_all(&[Capability::CoreExec, Capability::IPC, Capability::Memory])
        }

        SyscallNumber::MkGetPid => caps.can_getpid(),
        SyscallNumber::MkArgs => caps.can_getpid(),
        SyscallNumber::MkThreadSpawn => caps.can_ipc(),
        // A capsule may only move its own fs base; that mutates nothing
        // outside its own PCB, so a valid token is the whole requirement.
        SyscallNumber::MkSetTls => caps.is_valid(),
        SyscallNumber::MkProcOutput => caps.can_ipc(),
        SyscallNumber::MkProcInput => caps.can_ipc(),
        SyscallNumber::MkStdinRead => caps.can_ipc(),
        SyscallNumber::MkWait => caps.can_ipc(),
        SyscallNumber::MkKill => caps.can_ipc(),

        SyscallNumber::MkSpawn
        | SyscallNumber::MkIpcCall
        | SyscallNumber::MkIpcRecv
        | SyscallNumber::MkIpcRecvFrom
        | SyscallNumber::MkIpcReply
        | SyscallNumber::MkIpcSend
        | SyscallNumber::MkIpcSendToPid
        | SyscallNumber::MkServiceLookup
        | SyscallNumber::MkServiceRegister
        | SyscallNumber::MkCapGrant
        | SyscallNumber::MkCapRevoke => caps.can_ipc(),

        SyscallNumber::MkDeviceList => caps.can_device_enum(),
        SyscallNumber::MkDeviceClaim | SyscallNumber::MkDeviceRelease => caps.can_driver(),
        SyscallNumber::MkMmioMap | SyscallNumber::MkMmioUnmap => caps.can_mmio(),
        SyscallNumber::MkIrqBind
        | SyscallNumber::MkIrqUnbind
        | SyscallNumber::MkIrqAck
        | SyscallNumber::MkIrqPoll => caps.can_irq(),
        SyscallNumber::MkIrqWait => caps.can_irq(),
        SyscallNumber::MkDmaMap | SyscallNumber::MkDmaUnmap => caps.can_dma(),
        SyscallNumber::MkPciConfigRead | SyscallNumber::MkPciConfigWrite => caps.can_driver(),
        SyscallNumber::MkPioGrant
        | SyscallNumber::MkPioRead
        | SyscallNumber::MkPioWrite
        | SyscallNumber::MkPioRelease => caps.can_pio(),

        SyscallNumber::MkDebug => caps.can_debug(),
        SyscallNumber::MkStdoutWrite => caps.can_ipc(),
        SyscallNumber::MkStoreWrite => caps.can_store_write(),

        SyscallNumber::MkSurfaceRegister
        | SyscallNumber::MkSurfaceShare
        | SyscallNumber::MkSurfaceRelease => caps.can_surface_create(),
        SyscallNumber::MkSurfaceAttach => caps.can_surface_map(),
        SyscallNumber::MkSurfacePresent => caps.can_present(),
        SyscallNumber::MkDisplayVsyncWait => caps.can_display_query(),
        SyscallNumber::MkInputEventPost => caps.can_input_source(),
        // Draining/waiting on the global raw-input ring is a privileged consumer
        // operation: the ring carries every keystroke. `can_input_source()`
        // accepts `Irq`, which every device driver holds, so it let any driver
        // capsule steal the keystroke stream (cross-capsule keylogging). The
        // consumer gate requires `InputSource` (the input_router's cap) and
        // excludes `Irq`, keeping drivers able to POST but not DRAIN.
        SyscallNumber::MkInputEventDrain => caps.can_input_consumer(),
        SyscallNumber::MkInputEventWait => caps.can_input_consumer(),

        // Only a SpawnWindow-trusted capsule (the desktop shell) may ask the
        // kernel to open another window instance of an embedded app capsule.
        SyscallNumber::MkSpawnInstance => caps.can_spawn_window(),

        // Running a baked command-line tool needs only IPC: the tool is spawned
        // parented to the caller so the caller can drive its stdio, and only the
        // baked, attested set can be named.
        SyscallNumber::MkToolRun => caps.can_ipc(),

        _ => return None,
    })
}
