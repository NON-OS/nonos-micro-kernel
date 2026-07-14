// SCL clock count computation for the DesignWare I2C master.
//
// The controller generates no clock until the HCNT/LCNT pairs are programmed.
// Counts follow the proven monolithic driver's divider: split the bit period
// evenly between SCL high and low (half the input clock per bit edge) minus the
// controller's fixed overhead. The earlier tHIGH/tLOW form ran the Fast-mode bus
// near 500kHz, over the 400kHz budget, which a real ELAN NAKs; this keeps it in
// spec on hardware.

#[derive(Clone, Copy)]
pub struct SclCounts {
    pub hcnt: u32,
    pub lcnt: u32,
}

const SS_HZ: u32 = 100_000;
const FS_HZ: u32 = 400_000;
const HCNT_OFFSET: u32 = 7;
const LCNT_OFFSET: u32 = 1;

fn counts(clk_hz: u32, bus_hz: u32) -> SclCounts {
    let half = clk_hz / bus_hz / 2;
    SclCounts {
        hcnt: half.saturating_sub(HCNT_OFFSET).max(6),
        lcnt: half.saturating_sub(LCNT_OFFSET).max(8),
    }
}

pub fn standard(clk_hz: u32) -> SclCounts {
    counts(clk_hz, SS_HZ)
}

pub fn fast(clk_hz: u32) -> SclCounts {
    counts(clk_hz, FS_HZ)
}

// Fast-mode spike-suppression length in input-clock cycles (~100ns). The
// DesignWare core needs this programmed or short glitches corrupt reads.
pub fn fs_spklen(clk_hz: u32) -> u32 {
    (clk_hz / 10_000_000).max(1)
}

pub fn sda_hold(clk_hz: u32) -> u32 {
    const HOLD_NS: u64 = 300;
    let ticks = (clk_hz as u64).saturating_mul(HOLD_NS) / 1_000_000_000;
    ticks.clamp(1, u16::MAX as u64) as u32
}
