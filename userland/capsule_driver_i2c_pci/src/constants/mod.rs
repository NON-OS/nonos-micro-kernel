pub const INTEL_VENDOR_ID: u16 = 0x8086;
pub const BAR_INDEX: u32 = 0;
pub const BAR_OFFSET: u64 = 0;
pub const PAGE_MASK: u64 = 0xFFF;

pub const IC_CON: u64 = 0x00;
pub const IC_TAR: u64 = 0x04;
pub const IC_DATA_CMD: u64 = 0x10;
pub const IC_SS_SCL_HCNT: u64 = 0x14;
pub const IC_SS_SCL_LCNT: u64 = 0x18;
pub const IC_FS_SCL_HCNT: u64 = 0x1C;
pub const IC_FS_SCL_LCNT: u64 = 0x20;
pub const IC_FS_SPKLEN: u64 = 0xA0;
pub const IC_INTR_MASK: u64 = 0x30;
pub const IC_RAW_INTR_STAT: u64 = 0x34;
pub const IC_RX_TL: u64 = 0x38;
pub const IC_TX_TL: u64 = 0x3C;
pub const IC_CLR_INTR: u64 = 0x40;
pub const IC_ENABLE: u64 = 0x6C;
pub const IC_STATUS: u64 = 0x70;
pub const IC_TXFLR: u64 = 0x74;
pub const IC_RXFLR: u64 = 0x78;
pub const IC_SDA_HOLD: u64 = 0x7C;
pub const IC_TX_ABRT_SOURCE: u64 = 0x80;
pub const IC_ENABLE_STATUS: u64 = 0x9C;
pub const IC_COMP_PARAM_1: u64 = 0xF4;
pub const IC_COMP_TYPE: u64 = 0xFC;

// Intel LPSS wrapper (Skylake and later, drivers/mfd/intel-lpss.c). The
// DesignWare core sits at 0x000..0x100; the LPSS private register block is at
// offset 0x200 inside the same MMIO BAR. Out of reset the LPSS holds the
// DesignWare core asserted, so the RESETS register must be deasserted before
// any core register is touched or every access returns garbage.
pub const LPSS_PRIV: u64 = 0x200;
pub const LPSS_PRIV_RESETS: u64 = LPSS_PRIV + 0x04;
// Bit 2 (FUNC) releases the function reset; bits 1:0 (IDMA) release the
// integrated DMA reset. Writing 0x7 deasserts all three; writing 0 asserts
// them. This matches intel_lpss_deassert_reset().
pub const LPSS_PRIV_RESETS_FUNC: u32 = 1 << 2;
pub const LPSS_PRIV_RESETS_IDMA: u32 = 0x3;
pub const LPSS_PRIV_RESETS_DEASSERT: u32 = LPSS_PRIV_RESETS_FUNC | LPSS_PRIV_RESETS_IDMA;

pub fn device_info(device: u16) -> Option<(&'static str, u32)> {
    match device {
        0x9D60..=0x9D65 => Some(("Sunrise Point-LP", 120_000_000)),
        0xA160..=0xA163 => Some(("Sunrise Point-H", 120_000_000)),
        0x9DE8..=0x9DEB => Some(("Cannon Point-LP", 120_000_000)),
        0xA368..=0xA36B => Some(("Cannon Lake-H", 120_000_000)),
        0x02E8..=0x02EB => Some(("Comet Lake", 120_000_000)),
        0x06E8..=0x06EB => Some(("Comet Lake-H", 120_000_000)),
        0xA0E8..=0xA0EB | 0xA0C5 | 0xA0C6 => Some(("Tiger Lake-LP", 100_000_000)),
        0x43E8..=0x43EB => Some(("Tiger Lake-H", 100_000_000)),
        0x51E8..=0x51EB | 0x51C5 | 0x51C6 => Some(("Alder Lake-P", 100_000_000)),
        0x7AE8..=0x7AEB | 0x7AF8 | 0x7AF9 => Some(("Alder Lake-S", 100_000_000)),
        0xA0D8..=0xA0DD => Some(("Raptor Lake-P", 100_000_000)),
        0x7A4C..=0x7A4F | 0x7A7C | 0x7A7D => Some(("Raptor Lake-S", 100_000_000)),
        0x54E8..=0x54EB => Some(("Alder Lake-N", 100_000_000)),
        0x7E50..=0x7E52 | 0x7E78..=0x7E7A => Some(("Meteor Lake-P", 100_000_000)),
        0x34E8..=0x34EB | 0x34C5 | 0x34C6 => Some(("Ice Lake-LP", 100_000_000)),
        0x4DE8..=0x4DEB | 0x4DC5 | 0x4DC6 => Some(("Jasper Lake", 100_000_000)),
        // Broxton and Gemini Lake I2C runs from a 133 MHz input clock (Linux
        // intel-lpss-pci bxt_i2c_info). Deriving the SCL counts from 100 MHz
        // clocks the bus a third faster than programmed, out of fast-mode spec.
        0x5AC2 | 0x5AC4 | 0x5AC6 | 0x5AEE => Some(("Broxton", 133_000_000)),
        0x1AC2 | 0x1AC4 | 0x1AC6 | 0x1AEE => Some(("Broxton-P", 133_000_000)),
        0x31AC | 0x31AE | 0x31B0 | 0x31B2 => Some(("Gemini Lake", 133_000_000)),
        0x31B4 | 0x31B6 | 0x31B8 | 0x31BA => Some(("Gemini Lake", 133_000_000)),
        _ => None,
    }
}
/// Broker class id for an ACPI GPIO community controller (mirrors the
/// kernel's ids::GPIO_CTRL).
pub const CLASS_GPIO_CTRL: u32 = 0x0080;
/// Intel pinctrl community register holding PADBAR: the offset, inside the
/// community's MMIO window, of the first pad's configuration registers.
pub const GPIO_PADBAR: u64 = 0x00C;
/// Bytes of configuration registers per pad on the Broxton/Gemini Lake
/// family (PADCFG0 + PADCFG1).
pub const GPIO_PAD_STRIDE: u64 = 8;
/// PADCFG0 bit 1 is GPIORXSTATE: the live electrical level of the pad,
/// read-only. An i2c-HID device holds its interrupt line active (low) while
/// an input report waits and releases it once the report is read, which
/// makes this bit a doorbell that needs no interrupt routing, no latch
/// configuration, and no writes to GPIO hardware at all.
pub const GPIO_RXSTATE: u32 = 1 << 1;

/// Index of an LPSS I2C function among its family's controllers (the `n` in
/// the firmware's I2Cn device name), derived from the PCI device id layout.
/// Lets the driver match the ACPI `_CRS` ResourceSource name against an
/// enumerated PCI function. Only families whose id blocks are index-ordered
/// are mapped; None means the name cannot be matched and probing decides.
pub fn controller_index(device: u16) -> Option<u8> {
    match device {
        // Gemini Lake: I2C0..I2C7 = 0x31AC, 0x31AE, .. 0x31BA (even ids).
        0x31AC..=0x31BA if device & 1 == 0 => Some(((device - 0x31AC) / 2) as u8),
        _ => None,
    }
}

pub const IC_DATA_CMD_READ: u32 = 1 << 8;
pub const IC_DATA_CMD_STOP: u32 = 1 << 9;
pub const IC_DATA_CMD_RESTART: u32 = 1 << 10;
pub const IC_INTR_TX_ABRT: u32 = 1 << 6;
pub const IC_STATUS_TFE: u32 = 1 << 2;
pub const IC_STATUS_RFNE: u32 = 1 << 3;
pub const IC_STATUS_MST_ACTIVITY: u32 = 1 << 5;
pub const IC_ENABLE_ENABLE: u32 = 1;
pub const IC_CLR_TX_ABRT: u64 = 0x54;
pub const TIMEOUT_ITERS: usize = 250_000;
pub const TX_FIFO_DEPTH: u32 = 64;
pub const RX_FIFO_DEPTH: u32 = 64;
pub const IC_CON_MASTER_MODE: u32 = 1 << 0;
pub const IC_CON_SPEED_FAST: u32 = 2 << 1;
pub const IC_CON_RESTART_EN: u32 = 1 << 5;
pub const IC_CON_SLAVE_DISABLE: u32 = 1 << 6;
