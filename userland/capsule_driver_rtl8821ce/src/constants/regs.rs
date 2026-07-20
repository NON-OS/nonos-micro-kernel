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

//! A few MAC registers used to confirm the chip answers after power-on, from
//! the stable rtw88 MAC map.

/// System configuration: after power-on this reads back a plausible chip id
/// (not all-zero, not all-ones), which is the first sign the MMIO window is
/// live and the MAC is powered.
pub const REG_SYS_CFG1: usize = 0x00FC;

/// The station MAC address, six bytes at `0x0610`. The efuse autoload populates
/// it during power-on, so a plain read after bring-up returns the address burned
/// into this card.
pub const REG_MACID: usize = 0x0610;

/// The BSSID of the network the station is linked to, six bytes at `0x0618`.
/// Until this is set (together with the network-type field below) the MAC stays
/// in no-link mode: it receives beacons and management frames but drops unicast
/// data frames from the access point, so the WPA2 handshake never sees the AP's
/// first EAPOL data frame.
pub const REG_BSSID: usize = 0x0618;

/// The MAC command register `REG_CR` (`0x0100`). Its low byte holds the TX/RX
/// engine enables; bits 16-17 hold the network type for port 0.
pub const REG_CR: usize = 0x0100;
/// Network-type field mask and shift within `REG_CR`, and the "managed, linked"
/// value that puts the MAC into infrastructure-station mode so it accepts the
/// access point's unicast data frames. Mirrors rtw88 `RTW_NET_MGD_LINKED`.
pub const NETTYPE_SHIFT: u32 = 16;
pub const NETTYPE_MASK: u32 = 0x3 << NETTYPE_SHIFT;
pub const NET_TYPE_LINKED: u32 = 2;

/// A read of all-ones means the BAR is not decoding; all-zero means the MAC is
/// unpowered. Either is a dead read.
pub fn mmio_dead(v: u32) -> bool {
    v == 0 || v == 0xFFFF_FFFF
}
