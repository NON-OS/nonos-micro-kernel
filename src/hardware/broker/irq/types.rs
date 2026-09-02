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

//! Wire and kernel-side types for `MkIrqBind`. The `flags` field on
//! the request selects between two delivery paths: legacy INTx
//! (default, flags == 0) and MSI-X (`BIND_MSIX` set). The grant
//! carries the kind and, for MSI-X, the device-relative vector
//! index so teardown can unwind the MSI-X table entry it programmed.

// Public flag bits for `IrqBindRequest::flags`. The kernel rejects
// any unset bit so capsules cannot quietly opt into a future flag
// they were not designed against.
pub const BIND_MSIX: u32 = 1 << 0;
pub const FLAGS_KNOWN: u32 = BIND_MSIX;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrqGrantKind {
    Intx = 0,
    Msix = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct IrqGrant {
    pub grant_id: u64,
    pub pid: u32,
    pub device_id: u64,
    pub claim_epoch: u64,
    pub irq_source: u32,
    pub vector: u8,
    pub flags: u32,
    pub kind: IrqGrantKind,
    // For `IrqGrantKind::Msix` this is the index of the MSI-X table
    // entry the kernel programmed for this grant (0..table_size).
    // Always 0 for INTx grants.
    pub device_vector: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct IrqBindRequest {
    pub device_id: u64,
    pub claim_epoch: u64,
    // INTx mode: GSI from `mk_device_list`.
    // MSI-X mode: must be 0; the kernel always programs the MSI-X
    // table starting at entry 0 for the device.
    pub irq_source: u32,
    pub flags: u32,
    // INTx mode: must be 0.
    // MSI-X mode: 1..=BROKER_VEC_COUNT, capped further by the
    // device's MSI-X table size.
    pub vector_count: u32,
}

// `IrqBindResult` is the base of an N-vector range. For INTx N is
// always 1. For MSI-X N == request.vector_count and the capsule
// derives the per-vector grant IDs as `grant_id + i` and vectors
// as `vector + i` for i in 0..vector_count.
#[derive(Debug, Clone, Copy)]
pub struct IrqBindResult {
    pub grant_id: u64,
    pub vector: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrqBindError {
    NotClaimed,
    StaleEpoch,
    UnknownDevice,
    NotDeviceIrq,
    AlreadyBound,
    /// The line is one the kernel routes to itself.
    ReservedGsi,
    NoVector,
    UnsupportedFlags,
    NotIntx,
    NoMsixCap,
    BadMsixBar,
    BadVectorCount,
    MsixProgramFailed,
    NoDeviceHandle,
    PlatformError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrqError {
    UnknownGrant,
    NotHolder,
    PlatformError,
}

#[derive(Debug, Clone, Copy)]
pub struct IrqPollResult {
    pub seq: u64,
    pub overflow: u64,
}
