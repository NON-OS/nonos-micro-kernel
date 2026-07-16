use crate::regs::Regs;

/// A mapped GPIO-community window serving as the touchpad's "fresh report"
/// doorbell: the pad's PADCFG0.GPIORXSTATE bit is the live level of its
/// interrupt line, which an i2c-HID device holds active-low while a report
/// waits and releases once it is read. Purely read-only sensing — no
/// interrupt routing, no latch configuration, no GPIO writes.
#[derive(Clone, Copy)]
pub struct Doorbell {
    pub regs: Regs,
    /// Offset of the pad's PADCFG0 register inside the community window.
    pub cfg_offset: u64,
}

pub struct Driver {
    /// Present when the platform declared the pad's GPIO interrupt and its
    /// community window could be claimed and mapped.
    pub doorbell: Option<Doorbell>,
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
