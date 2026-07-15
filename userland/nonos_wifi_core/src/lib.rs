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

//! Shared WiFi core. The chip-independent halves of a WiFi driver live here so
//! the Intel and Realtek drivers hold one copy, not two: the net_core link
//! adapter (`netif`) with its `LinkPort` trait, the key-installation seam
//! (`key::KeyStore`) that lets CCMP be software on one chip and hardware on
//! another, and the plaintext frame contract (`frame`) the two exchange. The
//! 802.11/WPA2 brains (mlme, wpa supplicant, ccmp default, eapol, dot11) live
//! here too; the drivers implement `LinkPort` and `KeyStore` over their own
//! rings and never fork the shared logic.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod ccmp;
pub mod dot11;
pub mod eapol;
pub mod frame;
pub mod key;
pub mod mlme;
pub mod netif;
pub mod wpa;
