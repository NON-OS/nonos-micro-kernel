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
use crate::constants::{GPU_CFG_EVENTS_READ, GPU_CFG_NUM_CAPSETS, GPU_CFG_NUM_SCANOUTS};
use crate::device::ControlQueue;
use crate::regs::Regs;
use crate::setup::Primary;
use crate::state::{FenceCounter, ResourceTable, ScanoutTable};
pub struct Driver {
    pub device_id: u64,
    pub pci_device: u16,
    pub claim_epoch: u64,
    pub mmio_grant: u64,
    pub irq_grant: u64,
    pub queue_grant: u64,
    pub queue_user_va: u64,
    pub queue_device_addr: u64,
    pub queue_size: u16,
    pub host_features: u32,
    // True when VirGL was negotiated and render context 1 exists on the host,
    // i.e. RESOURCE_CREATE_3D / SUBMIT_3D are usable.
    pub virgl_ready: bool,
    pub regs: Regs,
    pub control_queue: ControlQueue,
    pub resources: ResourceTable,
    pub scanouts: ScanoutTable,
    pub fences: FenceCounter,
    pub primary: Option<Primary>,
}
impl Driver {
    pub fn config(&self) -> (u32, u32, u32) {
        unsafe {
            (
                self.regs.config_r32(GPU_CFG_EVENTS_READ),
                self.regs.config_r32(GPU_CFG_NUM_SCANOUTS),
                self.regs.config_r32(GPU_CFG_NUM_CAPSETS),
            )
        }
    }
    pub fn display_info(&self) -> (u32, u32, u32) {
        self.config()
    }
}
