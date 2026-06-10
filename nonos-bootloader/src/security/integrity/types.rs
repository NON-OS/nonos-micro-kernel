// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

#[derive(Clone, Copy)]
pub struct ChainLink {
    pub stage: BootStage,
    pub measurement: [u8; 32],
    pub cumulative: [u8; 32],
    pub timestamp: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BootStage {
    Init = 0,
    UefiServices = 1,
    SecurityPolicy = 2,
    HardwareDiscovery = 3,
    KernelLoad = 4,
    CryptoVerify = 5,
    ZkAttestation = 6,
    ElfParse = 7,
    HandoffPrepare = 8,
    ExitBootServices = 9,
    KernelEntry = 10,
}

impl ChainLink {
    pub const fn empty() -> Self {
        Self { stage: BootStage::Init, measurement: [0u8; 32], cumulative: [0u8; 32], timestamp: 0 }
    }
}
