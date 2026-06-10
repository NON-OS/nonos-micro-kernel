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

mod device;
mod dma;
mod irq;
mod mmio;
mod pci;
mod pio;
mod types;

pub use device::{mk_device_claim, mk_device_list, mk_device_release};
pub use dma::{mk_dma_map, mk_dma_unmap, MK_DMA_MAP_HIGH};
pub use irq::{mk_irq_ack, mk_irq_bind, mk_irq_poll, mk_irq_unbind, mk_irq_wait, MK_IRQ_BIND_MSIX};
pub use mmio::{mk_mmio_map, mk_mmio_unmap};
pub use pci::{
    mk_pci_config_read, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_MSIX_CTRL_ENABLE, MK_PCI_MSIX_CTRL_FUNCTION_MASK,
};
pub use pio::{mk_pio_grant, mk_pio_read, mk_pio_release, mk_pio_write};
pub use types::{
    Bar, DeviceRecord, DmaMapOut, IrqBindOut, IrqPollOut, MmioMapOut, PioGrantOut, BAR_FLAG_MEM64,
    BAR_FLAG_PREFETCH, BAR_KIND_MMIO, BAR_KIND_NONE, BAR_KIND_PIO, BUS_KIND_ACPI, BUS_KIND_PCI,
    BUS_KIND_VIRT, DEVICE_FLAG_CLAIMED, DEVICE_FLAG_DISABLED,
};
