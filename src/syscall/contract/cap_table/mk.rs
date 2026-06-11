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

use crate::capabilities::CapabilityToken;
use crate::syscall::numbers::SyscallNumber;

pub(super) fn check(caps: &CapabilityToken, number: SyscallNumber) -> Option<bool> {
    Some(match number {
        SyscallNumber::MkExit
        | SyscallNumber::MkPidAlive
        | SyscallNumber::MkYield
        | SyscallNumber::MkTimeMillis
        | SyscallNumber::MkTimeRtc
        | SyscallNumber::MkBatteryStatus
        | SyscallNumber::MkProcStat
        | SyscallNumber::MkAttestStatus
        | SyscallNumber::MkCapCheck => caps.is_valid(),

        SyscallNumber::MkMmap => caps.can_allocate_memory(),
        SyscallNumber::MkMunmap => caps.can_deallocate_memory(),

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
        | SyscallNumber::MkIrqPoll
        | SyscallNumber::MkIrqWait => caps.can_irq(),
        SyscallNumber::MkDmaMap | SyscallNumber::MkDmaUnmap => caps.can_dma(),
        SyscallNumber::MkPciConfigRead | SyscallNumber::MkPciConfigWrite => caps.can_driver(),
        SyscallNumber::MkPioGrant
        | SyscallNumber::MkPioRead
        | SyscallNumber::MkPioWrite
        | SyscallNumber::MkPioRelease => caps.can_pio(),

        SyscallNumber::MkDebug => caps.can_debug(),

        SyscallNumber::MkSurfaceRegister
        | SyscallNumber::MkSurfaceShare
        | SyscallNumber::MkSurfaceRelease => caps.can_surface_create(),
        SyscallNumber::MkSurfaceAttach => caps.can_surface_map(),
        SyscallNumber::MkSurfacePresent => caps.can_present(),
        SyscallNumber::MkDisplayVsyncWait => caps.can_display_query(),
        SyscallNumber::MkInputEventPost => caps.can_input_source(),
        SyscallNumber::MkInputEventDrain => caps.can_ipc(),
        SyscallNumber::MkInputEventWait => caps.can_ipc(),

        _ => return None,
    })
}
