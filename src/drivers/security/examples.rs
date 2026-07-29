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

use crate::memory::addr::{PhysAddr, VirtAddr};

use super::{
    is_config_write_allowed, safe_mmio_read32, validate_dma_buffer, validate_lba_range,
    validate_mmio_region, validate_pci_access, validate_prp_list, DriverError, DriverOpType,
    RateLimiter,
};

pub fn secure_nvme_init(bar_addr: usize, bar_size: usize) -> Result<(), DriverError> {
    validate_mmio_region(bar_addr, bar_size)?;

    let cap = safe_mmio_read32(VirtAddr::new(bar_addr as u64))?;

    crate::log_info!("NVMe Controller Capabilities: 0x{:08x}", cap);
    Ok(())
}

pub fn secure_dma_transfer(
    buffer_phys: PhysAddr,
    size: usize,
    prp_list: &[u64],
) -> Result<(), DriverError> {
    validate_dma_buffer(buffer_phys, size)?;

    validate_prp_list(prp_list, size)?;

    Ok(())
}

pub fn secure_pci_write(
    bus: u8,
    device: u8,
    function: u8,
    offset: u8,
    value: u32,
) -> Result<(), DriverError> {
    validate_pci_access(bus, device, function, offset)?;

    if !is_config_write_allowed(offset) {
        return Err(DriverError::ConfigWriteDenied);
    }

    crate::log_info!(
        "PCI write: {:02x}:{:02x}.{} offset 0x{:02x} = 0x{:08x}",
        bus,
        device,
        function,
        offset,
        value
    );
    Ok(())
}

pub fn secure_storage_read(
    lba: u64,
    block_count: u64,
    device_capacity_lba: u64,
) -> Result<(), DriverError> {
    validate_lba_range(lba, block_count, device_capacity_lba)?;

    crate::log_info!("Reading LBA {} count {}", lba, block_count);
    Ok(())
}

pub struct SecureNvmeQueue {
    rate_limiter: RateLimiter,
}

impl SecureNvmeQueue {
    pub const fn new() -> Self {
        Self { rate_limiter: RateLimiter::new(100_000) }
    }

    pub fn submit_io(&self, op_type: DriverOpType) -> Result<(), DriverError> {
        self.rate_limiter.check_rate(op_type)?;

        Ok(())
    }

    pub fn submit_admin(&self) -> Result<(), DriverError> {
        self.rate_limiter.check_rate(DriverOpType::AdminCommand)?;

        Ok(())
    }

    pub fn stats(&self) -> (u64, u32) {
        self.rate_limiter.stats()
    }
}

impl Default for SecureNvmeQueue {
    fn default() -> Self {
        Self::new()
    }
}

pub fn example_secure_driver_flow() -> Result<(), DriverError> {
    let bar_addr = 0xFED0_0000;
    let bar_size = 8192;
    validate_mmio_region(bar_addr, bar_size)?;

    let version = safe_mmio_read32(VirtAddr::new(bar_addr as u64 + 8))?;
    crate::log_info!("Device version: 0x{:08x}", version);

    let dma_phys = PhysAddr::new(0x5000_0000);
    let dma_size = 4096;
    validate_dma_buffer(dma_phys, dma_size)?;

    let prp_list = [0x5000_0000u64, 0x5000_1000u64];
    validate_prp_list(&prp_list, dma_size)?;

    validate_pci_access(0, 1, 0, 0x10)?;

    validate_lba_range(0, 16, 1000)?;

    let queue = SecureNvmeQueue::new();
    queue.submit_io(DriverOpType::IoCommand)?;

    Ok(())
}
