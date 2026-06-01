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

use crate::syscall::abi::{tag4, AbiDomain, AbiEntry, AbiStatus};
use crate::syscall::numbers::SyscallNumber;

// All Mk* native syscalls. Every entry is Routed — the dispatcher
// match in `dispatch/router/dispatch_fn.rs` forwards each to
// `microkernel::dispatch_microkernel_syscall`. Capability gates live
// at `contract/cap_table/mk.rs`.
pub(super) const ENTRIES: &[AbiEntry] = &[
    e(b"MISD", SyscallNumber::MkIpcSend, "MkIpcSend"),
    e(b"MIRC", SyscallNumber::MkIpcRecv, "MkIpcRecv"),
    e(b"MICL", SyscallNumber::MkIpcCall, "MkIpcCall"),
    e(b"MIRF", SyscallNumber::MkIpcRecvFrom, "MkIpcRecvFrom"),
    e(b"MIRY", SyscallNumber::MkIpcReply, "MkIpcReply"),
    e(b"MISP", SyscallNumber::MkIpcSendToPid, "MkIpcSendToPid"),
    e(b"MSVL", SyscallNumber::MkServiceLookup, "MkServiceLookup"),
    e(b"MSVR", SyscallNumber::MkServiceRegister, "MkServiceRegister"),
    e(b"MMAP", SyscallNumber::MkMmap, "MkMmap"),
    e(b"MUMP", SyscallNumber::MkMunmap, "MkMunmap"),
    e(b"MSPN", SyscallNumber::MkSpawn, "MkSpawn"),
    e(b"MEXT", SyscallNumber::MkExit, "MkExit"),
    e(b"MYLD", SyscallNumber::MkYield, "MkYield"),
    e(b"MTMS", SyscallNumber::MkTimeMillis, "MkTimeMillis"),
    e(b"MCGT", SyscallNumber::MkCapGrant, "MkCapGrant"),
    e(b"MCRV", SyscallNumber::MkCapRevoke, "MkCapRevoke"),
    e(b"MCCK", SyscallNumber::MkCapCheck, "MkCapCheck"),
    e(b"MDLS", SyscallNumber::MkDeviceList, "MkDeviceList"),
    e(b"MDCL", SyscallNumber::MkDeviceClaim, "MkDeviceClaim"),
    e(b"MDRL", SyscallNumber::MkDeviceRelease, "MkDeviceRelease"),
    e(b"MMMP", SyscallNumber::MkMmioMap, "MkMmioMap"),
    e(b"MMUM", SyscallNumber::MkMmioUnmap, "MkMmioUnmap"),
    e(b"MIRB", SyscallNumber::MkIrqBind, "MkIrqBind"),
    e(b"MIRU", SyscallNumber::MkIrqUnbind, "MkIrqUnbind"),
    e(b"MIRA", SyscallNumber::MkIrqAck, "MkIrqAck"),
    e(b"MIRP", SyscallNumber::MkIrqPoll, "MkIrqPoll"),
    e(b"MDMM", SyscallNumber::MkDmaMap, "MkDmaMap"),
    e(b"MDMU", SyscallNumber::MkDmaUnmap, "MkDmaUnmap"),
    e(b"MPCR", SyscallNumber::MkPciConfigRead, "MkPciConfigRead"),
    e(b"MPCW", SyscallNumber::MkPciConfigWrite, "MkPciConfigWrite"),
    e(b"MPGT", SyscallNumber::MkPioGrant, "MkPioGrant"),
    e(b"MPRD", SyscallNumber::MkPioRead, "MkPioRead"),
    e(b"MPWR", SyscallNumber::MkPioWrite, "MkPioWrite"),
    e(b"MPRL", SyscallNumber::MkPioRelease, "MkPioRelease"),
    e(b"MDBG", SyscallNumber::MkDebug, "MkDebug"),
    e(b"MSRG", SyscallNumber::MkSurfaceRegister, "MkSurfaceRegister"),
    e(b"MSSH", SyscallNumber::MkSurfaceShare, "MkSurfaceShare"),
    e(b"MSAT", SyscallNumber::MkSurfaceAttach, "MkSurfaceAttach"),
    e(b"MSRL", SyscallNumber::MkSurfaceRelease, "MkSurfaceRelease"),
    e(b"MSPR", SyscallNumber::MkSurfacePresent, "MkSurfacePresent"),
    e(b"MDVW", SyscallNumber::MkDisplayVsyncWait, "MkDisplayVsyncWait"),
    e(b"MIEP", SyscallNumber::MkInputEventPost, "MkInputEventPost"),
    e(b"MIED", SyscallNumber::MkInputEventDrain, "MkInputEventDrain"),
    e(b"MIEW", SyscallNumber::MkInputEventWait, "MkInputEventWait"),
];

const fn e(tag: &[u8; 4], variant: SyscallNumber, name: &'static str) -> AbiEntry {
    AbiEntry { id: tag4(tag), variant, name, domain: AbiDomain::Mk, status: AbiStatus::Routed }
}
