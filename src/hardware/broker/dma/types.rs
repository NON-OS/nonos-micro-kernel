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

pub(super) const DMA_MAP_HIGH: u32 = 1 << 0;

#[derive(Debug, Clone, Copy)]
pub struct DmaGrant {
    pub grant_id: u64,
    pub pid: u32,
    pub device_id: u64,
    pub claim_epoch: u64,
    pub physical_start: u64,
    pub user_va: u64,
    pub length: u64,
    pub flags: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct DmaMapRequest {
    pub device_id: u64,
    pub claim_epoch: u64,
    pub length: u64,
    pub flags: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct DmaMapResult {
    pub user_va: u64,
    pub device_addr: u64,
    pub length: u64,
    pub grant_id: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaMapError {
    NotClaimed,
    StaleEpoch,
    UnknownDevice,
    BadAlignment,
    BadLength,
    BadLengthForClass,
    UnsupportedFlags,
    NoMemory,
    NoVaSpace,
    MapFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaError {
    UnknownGrant,
    NotHolder,
}
