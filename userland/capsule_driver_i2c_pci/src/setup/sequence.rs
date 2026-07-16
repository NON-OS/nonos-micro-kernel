use nonos_libc::{mk_device_release, mk_irq_ack};

use super::{claim, irq, mmio};
use crate::constants::{controller_index, GPIO_PADBAR, GPIO_PAD_STRIDE};
use crate::discover::{
    find_controllers, find_gpio_community, find_touchpad_addrs, AcpiTouchpad, Found,
    MAX_CONTROLLERS, MAX_TARGETS,
};
use crate::driver::{Doorbell, Driver};
use crate::init::bring_up;
use crate::regs::Regs;
use crate::transaction::probe_hid;

pub fn run() -> Result<Driver, &'static str> {
    let mut cands = [Found::default(); MAX_CONTROLLERS];
    let n = find_controllers(&mut cands);
    if n == 0 {
        return Err("i2c-pci: controller not found");
    }
    // Several LPSS I2C hosts are present on Gemini Lake and only one carries
    // the touchpad — and multi-SKU firmware declares several candidate pads of
    // which only the fitted one exists. Bind the controller whose bus actually
    // answers one of the candidate addresses, and remember which address it
    // was so the HID driver talks to the device that is really there.
    let mut targets = [AcpiTouchpad::default(); MAX_TARGETS];
    let t = find_touchpad_addrs(&mut targets);
    for dev in &cands[..n] {
        if let Ok(mut driver) = bring_up_one(*dev) {
            if t == 0 {
                return Ok(driver);
            }
            let hit = targets[..t].iter().find(|tg| probe_hid(&driver, tg.addr).unwrap_or(false));
            if let Some(tg) = hit {
                driver.bound_by_probe = true;
                driver.bound_addr = tg.addr;
                attach_doorbell(&mut driver, tg);
                return Ok(driver);
            }
            let _ = mk_device_release(driver.device_id);
        }
    }
    // No bus answered any candidate, which this early in boot can just mean
    // the device is still asleep. Prefer a controller the firmware named in a
    // candidate's _CRS over a blind first-fit; the HID driver keeps reprobing
    // until the device wakes.
    for tg in &targets[..t] {
        let Some(idx) = tg.controller_idx else { continue };
        for dev in &cands[..n] {
            if !dev.is_acpi && controller_index(dev.pci_device) == Some(idx) {
                if let Ok(mut driver) = bring_up_one(*dev) {
                    driver.bound_addr = tg.addr;
                    attach_doorbell(&mut driver, tg);
                    return Ok(driver);
                }
            }
        }
    }
    // Last resort so a single-controller box still works.
    for dev in &cands[..n] {
        if let Ok(mut driver) = bring_up_one(*dev) {
            if let Some(tg) = targets[..t].first() {
                driver.bound_addr = tg.addr;
                attach_doorbell(&mut driver, tg);
            }
            return Ok(driver);
        }
    }
    Err("i2c-pci: no controller answered")
}

// Claim and map the GPIO community that carries the pad's interrupt, so the
// pad's live RX line level can serve as the "fresh report" doorbell.
// Best-effort and read-only: a platform without the ACPI data, or whose pad
// register lands outside the window, simply leaves the driver polling timed.
fn attach_doorbell(driver: &mut Driver, tg: &AcpiTouchpad) {
    let Some(uid) = tg.gpio_community else { return };
    let Some(dev) = find_gpio_community(uid) else { return };
    let Ok(epoch) = claim::claim(dev.device_id) else { return };
    let Ok(m) = mmio::map(dev, epoch) else { return };
    let regs = Regs::new(m.user_va);
    let padbar = u64::from(regs.read32(GPIO_PADBAR));
    let cfg_offset = padbar + u64::from(tg.gpio_pin) * GPIO_PAD_STRIDE;
    if padbar == 0 || padbar == 0xFFFF_FFFF || cfg_offset + 4 > m.length {
        return;
    }
    // An all-ones read means the window is not decoding this pad; leave the
    // doorbell off rather than sensing garbage.
    if regs.read32(cfg_offset) == 0xFFFF_FFFF {
        return;
    }
    driver.doorbell = Some(Doorbell { regs, cfg_offset });
}

fn bring_up_one(dev: Found) -> Result<Driver, &'static str> {
    let claim_epoch = claim::claim(dev.device_id)?;
    if !dev.is_acpi {
        super::pci::enable(dev.device_id, claim_epoch)?;
    }
    let mmio = mmio::map(dev, claim_epoch)?;
    let irq = irq::bind(dev, claim_epoch);
    let regs = Regs::new(mmio.user_va);
    let init = bring_up(regs, dev.clock_hz)?;
    if irq.grant_id != 0 {
        let _ = mk_irq_ack(irq.grant_id);
    }
    Ok(Driver {
        doorbell: None,
        device_id: dev.device_id,
        pci_device: dev.pci_device,
        claim_epoch,
        mmio_grant: mmio.grant_id,
        irq_grant: irq.grant_id,
        irq_vector: irq.vector,
        clock_hz: dev.clock_hz,
        family: dev.family,
        comp_type: init.comp_type,
        comp_param: init.comp_param,
        enabled: init.enabled,
        status: init.status,
        bound_by_probe: false,
        bound_addr: 0,
        regs,
    })
}
