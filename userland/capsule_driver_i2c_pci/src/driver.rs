use crate::regs::Regs;

pub struct Driver {
    pub device_id: u64,
    pub pci_device: u16,
    pub claim_epoch: u64,
    pub mmio_grant: u64,
    pub irq_grant: u64,
    pub irq_vector: u64,
    pub clock_hz: u32,
    pub family: &'static str,
    pub comp_type: u32,
    pub comp_param: u32,
    pub enabled: u32,
    pub status: u32,
    /// True when this controller was bound because the touchpad's address
    /// ACKed the setup probe; false for a fallback bind (named controller or
    /// first-fit) where the device never answered during setup.
    pub bound_by_probe: bool,
    /// The candidate address the bind settled on (ACKed the probe, or the
    /// firmware-named fallback's address). Zero when setup had no ACPI
    /// candidates; the HID driver then scans the bus itself.
    pub bound_addr: u8,
    pub regs: Regs,
}
