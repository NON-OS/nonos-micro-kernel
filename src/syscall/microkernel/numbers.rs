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

//! Microkernel syscall ABI tags. Mirrors `SyscallNumber::Mk*`
//! discriminants so the numeric router has a fixed local set.

use crate::syscall::abi::tag4;

pub const SYS_IPC_SEND: u64 = tag4(b"MISD");
pub const SYS_IPC_RECV: u64 = tag4(b"MIRC");
pub const SYS_IPC_CALL: u64 = tag4(b"MICL");
pub const SYS_IPC_RECV_FROM: u64 = tag4(b"MIRF");
pub const SYS_IPC_REPLY: u64 = tag4(b"MIRY");
pub const SYS_IPC_SEND_TO_PID: u64 = tag4(b"MISP");
pub const SYS_SERVICE_LOOKUP: u64 = tag4(b"MSVL");
pub const SYS_SERVICE_REGISTER: u64 = tag4(b"MSVR");
pub const SYS_MMAP: u64 = tag4(b"MMAP");
pub const SYS_MUNMAP: u64 = tag4(b"MUMP");
pub const SYS_SPAWN: u64 = tag4(b"MSPN");
pub const SYS_CAPSULE_LOAD: u64 = tag4(b"MCLD");
pub const SYS_CAPSULE_VERIFY: u64 = tag4(b"MCVF");
pub const SYS_EXIT: u64 = tag4(b"MEXT");
pub const SYS_PID_ALIVE: u64 = tag4(b"MPAL");
pub const SYS_WAIT: u64 = tag4(b"MWAT");
pub const SYS_KILL: u64 = tag4(b"MKIL");
pub const SYS_GETPID: u64 = tag4(b"MGPD");
pub const SYS_ARGS: u64 = tag4(b"MKAR");
pub const SYS_THREAD_SPAWN: u64 = tag4(b"MTSP");
pub const SYS_SET_TLS: u64 = tag4(b"MSTB");
pub const SYS_YIELD: u64 = tag4(b"MYLD");
pub const SYS_FUTEX_WAIT: u64 = tag4(b"MFTW");
pub const SYS_FUTEX_WAKE: u64 = tag4(b"MFTK");
pub const SYS_SLEEP_MS: u64 = tag4(b"MSLP");
pub const SYS_TIME_MILLIS: u64 = tag4(b"MTMS");
pub const SYS_TIME_MONOTONIC: u64 = tag4(b"MMON");
pub const SYS_TIME_RTC: u64 = tag4(b"MTRT");
pub const SYS_TIME_ADJUST: u64 = tag4(b"MTAD");
pub const SYS_BATTERY_STATUS: u64 = tag4(b"MBAT");
pub const SYS_PROC_STAT: u64 = tag4(b"MPST");
pub const SYS_PROC_OUTPUT: u64 = tag4(b"MOUT");
pub const SYS_PROC_INPUT: u64 = tag4(b"MPIN");
pub const SYS_STDIN_READ: u64 = tag4(b"MSRD");
// Program stdout: mirrors bytes into the caller's own `proc.<pid>` inbox and
// writes nothing to serial. Gated on the IPC capability so a capsule without
// `Capability::Debug` still has a stdout.
pub const SYS_STDOUT_WRITE: u64 = tag4(b"MSOW");
pub const SYS_STORE_WRITE: u64 = tag4(b"MSWR");
pub const SYS_ATTEST_STATUS: u64 = tag4(b"MAST");
/// A signed attestation document, as opposed to the unsigned status above.
pub const SYS_ATTEST_DOC: u64 = tag4(b"MADC");
/// Ask to enrol a signing root so software built here runs here. Prints a
/// confirmation code; enrols nothing on its own.
pub const SYS_DEV_ROOT_REQUEST: u64 = tag4(b"MDRQ");
/// Complete a pending enrolment with the code the kernel displayed.
pub const SYS_DEV_ROOT_CONFIRM: u64 = tag4(b"MDRC");
pub const SYS_CAP_GRANT: u64 = tag4(b"MCGT");
pub const SYS_CAP_REVOKE: u64 = tag4(b"MCRV");
pub const SYS_CAP_CHECK: u64 = tag4(b"MCCK");
pub const SYS_DEVICE_LIST: u64 = tag4(b"MDLS");
pub const SYS_DEVICE_CLAIM: u64 = tag4(b"MDCL");
pub const SYS_DEVICE_RELEASE: u64 = tag4(b"MDRL");
pub const SYS_MMIO_MAP: u64 = tag4(b"MMMP");
pub const SYS_MMIO_UNMAP: u64 = tag4(b"MMUM");
pub const SYS_IRQ_BIND: u64 = tag4(b"MIRB");
pub const SYS_IRQ_UNBIND: u64 = tag4(b"MIRU");
pub const SYS_IRQ_ACK: u64 = tag4(b"MIRA");
pub const SYS_IRQ_POLL: u64 = tag4(b"MIRP");
pub const SYS_IRQ_WAIT: u64 = tag4(b"MIRW");
pub const SYS_DMA_MAP: u64 = tag4(b"MDMM");
pub const SYS_DMA_UNMAP: u64 = tag4(b"MDMU");
pub const SYS_PIO_GRANT: u64 = tag4(b"MPGT");
pub const SYS_PIO_READ: u64 = tag4(b"MPRD");
pub const SYS_PIO_WRITE: u64 = tag4(b"MPWR");
pub const SYS_PIO_RELEASE: u64 = tag4(b"MPRL");
pub const SYS_MK_DEBUG: u64 = tag4(b"MDBG");
pub const SYS_PCI_CONFIG_READ: u64 = tag4(b"MPCR");
pub const SYS_PCI_CONFIG_WRITE: u64 = tag4(b"MPCW");
// Spawn another window instance of an embedded, attested app capsule
// (terminal or browser). Gated on the SpawnWindow capability.
pub const SYS_SPAWN_INSTANCE: u64 = tag4(b"MSPI");

// Run a baked, attested command-line tool by name, parented to the caller so
// it can drive the tool's stdin and stdout. Gated on the IPC capability.
pub const SYS_TOOL_RUN: u64 = tag4(b"MTRN");
