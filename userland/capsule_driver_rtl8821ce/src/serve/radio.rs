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

//! The radio the serve loop holds: map the DMA rings, bring the PHY up, and build
//! the `RtlLink` and its CAM key store.

use alloc::boxed::Box;

use crate::constants::regs::REG_MACID;
use crate::efuse;
use crate::fw::dma::Grant;
use crate::link::{RtlKeys, RtlLink};
use crate::phy::channel::Bw;
use crate::phy::{load_all, power_on, rxpath, PhyCond, INTF_PCIE};
use crate::regs::{Mmio, Regs};
use crate::ring::TX_BUF_DESC_SIZE;
use crate::rx::regs::RX_BUF_DESC_SIZE;
use crate::rx::ring::{RX_BUF_STRIDE, RX_DESC_COUNT};
use crate::setup::Mapped;
use crate::status;
use crate::tx::ring::{TX_BUF_STRIDE, TX_DESC_COUNT};

use super::{Stage, DEFAULT_CHANNEL};

/// The radio brought up: its rings, its key store and a register handle for
/// retuning during a scan. Boxed in `Radio` so the down case stays small.
pub(super) struct RadioUp {
    pub(super) link: RtlLink<Regs, Grant, Grant>,
    pub(super) keys: RtlKeys<Regs>,
    pub(super) regs: Regs,
}

/// The radio as the serve loop holds it: either fully up, or down, in which case
/// the loop still answers so the service stays registered and the panel can read
/// the failure stage.
pub(super) enum Radio {
    Up(Box<RadioUp>),
    Down,
}

/// Bring the radio to `Up` when bring-up reached `Ready` and the rings map, else
/// leave it `Down`. Returns the possibly-downgraded stage so a DMA failure at
/// this last step is reported too.
pub(super) fn build_radio(mapped: Option<Mapped>, stage: Stage) -> (Radio, Stage) {
    if stage != Stage::Ready {
        return (Radio::Down, stage);
    }
    let Some(m) = mapped else {
        return (Radio::Down, Stage::NotClaimed);
    };
    let Some((tx_ring, tx_buffers, rx_ring, rx_buffers)) = map_rings(m.device_id, m.claim_epoch)
    else {
        status::line(b"[rtl8821ce] dma ring mapping failed\n");
        return (Radio::Down, Stage::NoDma);
    };
    match phy_setup(&m.regs, m.efuse) {
        Ok(()) => {}
        Err(stage) => return (Radio::Down, stage),
    }
    let our_mac = read_mac(&m.regs);
    let link = RtlLink::new(m.regs, tx_ring, tx_buffers, rx_ring, rx_buffers, our_mac);
    let keys = RtlKeys::new(m.regs);
    (Radio::Up(Box::new(RadioUp { link, keys, regs: m.regs })), Stage::Ready)
}

/// The station MAC from the MAC-ID registers, as `phy_setup` programmed it.
pub(super) fn read_mac(regs: &Regs) -> [u8; 6] {
    let lo = regs.read32(REG_MACID);
    let hi = regs.read32(REG_MACID + 4);
    [lo as u8, (lo >> 8) as u8, (lo >> 16) as u8, (lo >> 24) as u8, hi as u8, (hi >> 8) as u8]
}

/// Map the four DMA regions the data path needs, each rounded up to whole pages.
type Rings = (Grant, Grant, Grant, Grant);
fn map_rings(device_id: u64, claim_epoch: u64) -> Option<Rings> {
    let tx_ring = map(device_id, claim_epoch, TX_DESC_COUNT as usize * TX_BUF_DESC_SIZE)?;
    let tx_buffers = map(device_id, claim_epoch, TX_DESC_COUNT as usize * TX_BUF_STRIDE)?;
    let rx_ring = map(device_id, claim_epoch, RX_DESC_COUNT as usize * RX_BUF_DESC_SIZE)?;
    let rx_buffers = map(device_id, claim_epoch, RX_DESC_COUNT as usize * RX_BUF_STRIDE)?;
    Some((tx_ring, tx_buffers, rx_ring, rx_buffers))
}

// Round a byte count up to whole pages and map it through the broker.
fn map(device_id: u64, claim_epoch: u64, bytes: usize) -> Option<Grant> {
    let pages = ((bytes + 0xFFF) & !0xFFF) as u64;
    crate::fwload::map_dma(device_id, claim_epoch, pages)
}

// Bring the radio up: read the board facts the PHY needs from the efuse, load the
// MAC, baseband, AGC and RF register tables for this cut and RF front-end, hold the
// receive path in reset across the table load, then select the 2.4GHz receive path,
// route the antenna into the receive amplifier, set transmit power, and tune the
// default channel. Without a good efuse read the RF front-end is unknown, so the
// radio is left unconfigured rather than programmed with the wrong tables.
fn phy_setup(regs: &Regs, efuse: Option<efuse::EfuseInfo>) -> Result<(), Stage> {
    // Taken during bring-up, on a freshly powered MAC, which is where rtw88 reads
    // it. Read here instead, after the firmware download and the MAC tables, the
    // efuse control and LDO registers answered zero on real silicon.
    // Preferably the read taken during bring-up, on a freshly powered MAC, which
    // is where rtw88 takes it. If that came back unreadable, try again here, which
    // is where this driver read it when the radio last came up on real hardware.
    // Never fall through with a map that failed validation: an all-ones efuse
    // yields front-end 0x1F, and programming the RF tables from it wedges the chip
    // hard enough that its register window stops answering at all.
    let info = match efuse.or_else(|| efuse::read(regs)) {
        Some(info) => info,
        None => {
            status::line(b"[rtl8821ce] efuse read failed, radio not configured\n");
            return Err(Stage::EfuseFailed);
        }
    };
    // rtw88 refuses a front-end it has no table for rather than programming the
    // nearest one. The radio tables are indexed off this, so a wrong value is not
    // a degraded radio, it is a chip driven with settings for hardware that is not
    // on this board.
    if !rxpath::rfe_is_supported(info.rfe) {
        status::line(b"[rtl8821ce] unsupported rf front-end, radio not configured\n");
        return Err(Stage::EfuseFailed);
    }
    let cond = PhyCond { cut: info.cut, pkg: info.pkg, intf: INTF_PCIE, rfe: info.rfe };
    power_on(regs);
    rxpath::pre_tables(regs);
    load_all(regs, &cond);
    rxpath::post_tables(regs);
    // The chip does not autoload a MAC, and an all-zero one associates but never
    // completes the four-way, so this must happen after the table load. Drawn
    // rather than read from the efuse; no entropy means no radio.
    match crate::station::draw() {
        Some(mac) => program_mac(regs, &mac),
        None => {
            status::line(b"[rtl8821ce] no entropy for station address\n");
            return Err(Stage::NoStationAddress);
        }
    }
    let rfe_btg = rxpath::rfe_is_btg(info.rfe);
    rxpath::set_channel(regs, DEFAULT_CHANNEL, Bw::W20, rfe_btg);
    status::line(b"[rtl8821ce] phy configured\n");
    Ok(())
}

// Write the six-byte station MAC into the MAC-ID registers so the transmitter
// stamps it as the source address and the receiver's physical-address filter
// accepts unicast frames addressed to us.
fn program_mac(regs: &Regs, mac: &[u8; 6]) {
    regs.write32(REG_MACID, u32::from_le_bytes([mac[0], mac[1], mac[2], mac[3]]));
    regs.write32(REG_MACID + 4, u32::from_le_bytes([mac[4], mac[5], 0, 0]));
}
