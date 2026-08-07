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

use crate::syscall::{numbers::SyscallNumber, SyscallResult};

pub(super) fn matches(nr: SyscallNumber) -> bool {
    use SyscallNumber::*;
    matches!(
        nr,
        MkIpcSend
            | MkIpcRecv
            | MkIpcCall
            | MkIpcRecvFrom
            | MkIpcReply
            | MkIpcSendToPid
            | MkServiceLookup
            | MkServiceRegister
            | MkMmap
            | MkMunmap
            | MkSpawn
            | MkCapsuleLoad
            | MkCapsuleVerify
            | MkExit
            | MkPidAlive
            | MkWait
            | MkKill
            | MkGetPid
            | MkArgs
            | MkThreadSpawn
            | MkSetTls
            | MkYield
            | MkTimeMillis
            | MkTimeMonotonic
            | MkTimeRtc
            | MkTimeAdjust
            | MkBatteryStatus
            | MkProcStat
            | MkProcOutput
            | MkProcInput
            | MkStdinRead
            | MkAttestStatus
            | MkCapGrant
            | MkCapRevoke
            | MkCapCheck
            | MkDeviceList
            | MkDeviceClaim
            | MkDeviceRelease
            | MkMmioMap
            | MkMmioUnmap
            | MkIrqBind
            | MkIrqUnbind
            | MkIrqAck
            | MkIrqPoll
            | MkIrqWait
            | MkDmaMap
            | MkDmaUnmap
            | MkPciConfigRead
            | MkPciConfigWrite
            | MkPioGrant
            | MkPioRead
            | MkPioWrite
            | MkPioRelease
            | MkDebug
            | MkStdoutWrite
            | MkStoreWrite
            | MkSpawnInstance
            | MkToolRun
    )
}

pub(super) fn handle(
    nr: SyscallNumber,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
) -> SyscallResult {
    let value = crate::syscall::microkernel::dispatch_microkernel_syscall(
        nr as u64, a0, a1, a2, a3, a4, a5,
    );
    SyscallResult { value, capability_consumed: false, audit_required: true }
}
