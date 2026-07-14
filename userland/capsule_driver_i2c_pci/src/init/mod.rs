mod scl;

use crate::constants::*;
use crate::regs::Regs;

#[derive(Clone, Copy)]
pub struct InitState {
    pub comp_type: u32,
    pub comp_param: u32,
    pub enabled: u32,
    pub status: u32,
}

pub fn bring_up(regs: Regs, clock_hz: u32) -> Result<InitState, &'static str> {
    lpss_deassert_reset(regs);
    // Prove MMIO actually reaches the DesignWare core before trusting any other
    // register. All-zeros means the function is unpowered or clock-gated (every
    // later read would return 0 too, so disable() below would "succeed"
    // instantly and a dead controller would bring up clean); all-ones means the
    // BAR is not decoding. Either way this is a power/mapping fault, not a
    // device fault, and binding the controller would only mask it.
    let comp_type = regs.read32(IC_COMP_TYPE);
    if comp_type == 0 || comp_type == 0xFFFF_FFFF {
        return Err("i2c-pci: mmio dead, comp_type read 0/all-ones");
    }
    disable(regs)?;
    regs.write32(
        IC_CON,
        IC_CON_MASTER_MODE | IC_CON_SPEED_FAST | IC_CON_RESTART_EN | IC_CON_SLAVE_DISABLE,
    );
    // The DesignWare master emits no SCL clock until the HCNT/LCNT pairs are
    // programmed. Both speed modes are set while the controller is disabled;
    // IC_CON selects fast mode, but the standard pair is required to be valid.
    program_clock(regs, clock_hz);
    regs.write32(IC_RX_TL, 0);
    regs.write32(IC_TX_TL, 0);
    regs.write32(IC_INTR_MASK, 0);
    let _ = regs.read32(IC_CLR_INTR);
    let comp_param = regs.read32(IC_COMP_PARAM_1);
    let enabled = regs.read32(IC_ENABLE_STATUS);
    let status = regs.read32(IC_STATUS);
    Ok(InitState { comp_type, comp_param, enabled, status })
}

// Writes the standard and fast SCL count pairs plus the SDA hold time. Only
// valid while IC_ENABLE is 0; these registers are read-only once enabled.
fn program_clock(regs: Regs, clock_hz: u32) {
    let ss = scl::standard(clock_hz);
    let fs = scl::fast(clock_hz);
    regs.write32(IC_SS_SCL_HCNT, ss.hcnt);
    regs.write32(IC_SS_SCL_LCNT, ss.lcnt);
    regs.write32(IC_FS_SCL_HCNT, fs.hcnt);
    regs.write32(IC_FS_SCL_LCNT, fs.lcnt);
    regs.write32(IC_FS_SPKLEN, scl::fs_spklen(clock_hz));
    regs.write32(IC_SDA_HOLD, scl::sda_hold(clock_hz));
}

// Releases the Intel LPSS reset that gates the DesignWare core. On Skylake and
// later the wrapper powers up with the function and iDMA resets asserted, which
// leaves the core dead. Writing LPSS_PRIV_RESETS = 0x7 deasserts them. This has
// to run once, before the first DesignWare register access in disable(). The
// functional clock is already running on these parts, so no divider setup is
// needed here. On non-LPSS hardware this offset is simply not present, but the
// driver only ever binds to Intel LPSS controllers, so the write is always
// valid when we reach here.
fn lpss_deassert_reset(regs: Regs) {
    regs.write32(LPSS_PRIV_RESETS, LPSS_PRIV_RESETS_DEASSERT);
}

fn disable(regs: Regs) -> Result<(), &'static str> {
    regs.write32(IC_ENABLE, 0);
    for _ in 0..TIMEOUT_ITERS {
        if regs.read32(IC_ENABLE_STATUS) & IC_ENABLE_ENABLE == 0 {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err("i2c-pci: controller disable timeout")
}
