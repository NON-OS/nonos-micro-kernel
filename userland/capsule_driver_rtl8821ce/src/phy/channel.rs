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

//! Compute the RF path-A frequency-synthesis register (RF register 0x18) for a
//! channel and bandwidth, and pick the RF front-end switch for that channel.
//! Register 0x18 carries the band (2G/5G), the channel number, an RF-synthesizer
//! index for the upper 5GHz sub-bands, and the bandwidth; the driver clears those
//! fields and rewrites them for the target channel while preserving the rest. The
//! field encodings are the rtw88 `RF18_*` masks in `rtw8821c.h` and the logic of
//! `rtw8821c_set_channel_rf`; the computation is checked known-answer in
//! `rtl8821ce_proofs`. Writing the value into the chip and the RF read-modify path
//! build on the RF serial read.

/// `RF18_BAND_MASK`: the band field.
pub const RF18_BAND_MASK: u32 = (1 << 16) | (1 << 9) | (1 << 8);
/// `RF18_BAND_5G`: the 5GHz band value (2G is zero).
pub const RF18_BAND_5G: u32 = (1 << 16) | (1 << 8);
/// `RF18_CHANNEL_MASK`: the channel-number field (low byte).
pub const RF18_CHANNEL_MASK: u32 = 0xFF;
/// `RF18_RFSI_MASK`: the RF-synthesizer index for the upper 5GHz sub-bands.
pub const RF18_RFSI_MASK: u32 = (1 << 18) | (1 << 17);
/// `RF18_RFSI_GE`: channels 100..=140.
pub const RF18_RFSI_GE: u32 = 1 << 17;
/// `RF18_RFSI_GT`: channels above 140.
pub const RF18_RFSI_GT: u32 = 1 << 18;
/// `RF18_BW_MASK`: the bandwidth field.
pub const RF18_BW_MASK: u32 = (1 << 11) | (1 << 10);

/// Channel bandwidth.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bw {
    W20,
    // Wider bandwidths come with 5GHz and HT40/VHT80 channel support; the receive
    // bring-up parks on a 20MHz channel, so they are exercised in the proofs only.
    #[cfg(test)]
    W40,
    #[cfg(test)]
    W80,
}

impl Bw {
    fn rf18(self) -> u32 {
        match self {
            // `RF18_BW_20M` = both bits, 40M = the high bit, 80M = the low bit.
            Bw::W20 => (1 << 11) | (1 << 10),
            #[cfg(test)]
            Bw::W40 => 1 << 11,
            #[cfg(test)]
            Bw::W80 => 1 << 10,
        }
    }
}

/// The RF front-end switch position for a channel. The baseband program that acts
/// on it is a companion of the RF retune that is not built, so this is exercised
/// in the proofs only.
#[cfg(test)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RfSwitch {
    /// 2G with Bluetooth-shared front end.
    Btg,
    /// 2G WLAN.
    Wlg,
    /// 5G WLAN.
    Wla,
}

/// Build RF register 0x18 for `channel` at `bw`, preserving the bits of `old`
/// outside the band, channel, synthesizer-index and bandwidth fields.
pub fn rf18_value(channel: u8, bw: Bw, old: u32) -> u32 {
    let mut v = old & !(RF18_BAND_MASK | RF18_CHANNEL_MASK | RF18_RFSI_MASK | RF18_BW_MASK);
    if channel > 14 {
        v |= RF18_BAND_5G;
    }
    v |= u32::from(channel) & RF18_CHANNEL_MASK;
    if (100..=140).contains(&channel) {
        v |= RF18_RFSI_GE;
    } else if channel > 140 {
        v |= RF18_RFSI_GT;
    }
    v | bw.rf18()
}

/// The RF front-end switch for a channel. `rfe_btg` is true when the board shares
/// its 2G front end with Bluetooth.
#[cfg(test)]
pub fn rf_switch(channel: u8, rfe_btg: bool) -> RfSwitch {
    if channel > 14 {
        RfSwitch::Wla
    } else if rfe_btg {
        RfSwitch::Btg
    } else {
        RfSwitch::Wlg
    }
}

// RF register numbers used to retune the synthesizer (rtw8821c.h).
const RF_REG18: u8 = 0x18; // band/channel/synthesizer/bandwidth
const RF_LUTDBG: u8 = 0xDF;
const RF_XTALX2: u8 = 0xB8;
const RF_LUT_2G: u8 = 0x64;
const RF_LUTDBG_BIT6: u32 = 1 << 6;
const RF_LUT_2G_MASK: u32 = 0xF;
const RF_XTALX2_BIT19: u32 = 1 << 19;

/// Retune the RF synthesizer to `channel` at `bw`: program register 0x18, set the
/// band-specific LUT debug bit (and the 2G LUT), then toggle the crystal-doubler
/// reload bit. This is the RF-register half of rtw88 `rtw8821c_set_channel_rf`;
/// the antenna front-end switch (a baseband program) is its companion. The
/// register program is checked against a modeled device in `rtl8821ce_proofs`.
pub fn set_rf<M: crate::regs::Mmio>(mmio: &M, channel: u8, bw: Bw) {
    use super::rf;
    let rf18 = rf::read_a(mmio, RF_REG18);
    let value = rf18_value(channel, bw, rf18);

    if channel <= 14 {
        rf::write_masked_a(mmio, RF_LUTDBG, RF_LUTDBG_BIT6, 1);
        rf::write_masked_a(mmio, RF_LUT_2G, RF_LUT_2G_MASK, 0xF);
    } else {
        rf::write_masked_a(mmio, RF_LUTDBG, RF_LUTDBG_BIT6, 0);
    }

    rf::write_a(mmio, RF_REG18, value);

    // Reload the crystal doubler: drop then raise the reload bit.
    rf::write_masked_a(mmio, RF_XTALX2, RF_XTALX2_BIT19, 0);
    rf::write_masked_a(mmio, RF_XTALX2, RF_XTALX2_BIT19, 1);
}
