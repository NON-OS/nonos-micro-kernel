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

//! Bind the RTL8821CE to the shared WiFi core. The WPA2 supplicant in
//! `nonos_wifi_core` derives the pairwise and group keys and installs them
//! through the `KeyStore` seam; on this chip that means writing the hardware
//! security CAM, after which the radio does CCMP itself. Group keys take the
//! reserved CAM entries 0..3 by their key index; pairwise keys take an entry
//! from 4 up, one per peer.
//!
//! The `LinkPort` (`RtlLink`) is the Ethernet data path net_core drives. It holds
//! a shared `LinkStation` fixed to `Ccmp::Hardware`: the station frames plaintext
//! 802.11 and the radio does CCMP from the CAM, so we never encrypt in software.
//! Transmit hands the framed MPDU to the TX ring with the CCMP security type set;
//! receive pulls a decrypted MPDU off the RX ring and the station parses it back
//! to Ethernet. Both are checked against the real `LinkPort`/`KeyStore` traits in
//! `rtl8821ce_proofs`.

use nonos_wifi_core::key::KeyStore;
use nonos_wifi_core::netif::LinkPort;
use nonos_wifi_core::station::{Ccmp, LinkStation};

use crate::fw::dma::{DmaMem, Grant};
use crate::regs::Mmio;
use crate::rx::ring::{RxState, RX_BUF_STRIDE, RX_DESC_COUNT};
use crate::rx::{poll_one, program as rx_program};
use crate::sec::{clear_cam, write_cam, Key, CAM_AES};
use crate::tx::desc::{FrameMeta, DESC_RATE_6M, SEC_TYPE_CCMP};
use crate::tx::regs::QSEL_BE;
use crate::tx::ring::{TxState, TX_DESC_COUNT};
use crate::tx::{enqueue, program as tx_program};

/// `RTW_SEC_DEFAULT_KEY_NUM`: CAM entries 0..3 are reserved for group keys keyed
/// by their 802.11 key index; pairwise keys start here.
const PAIRWISE_BASE: u8 = 4;
/// How many pairwise peers this station tracks. A client associates with one AP;
/// a small margin covers a roam that installs a new key before removing the old.
const PAIRWISE_SLOTS: usize = 4;

/// The chip's key store: which peer occupies each pairwise CAM slot, over the
/// register window `M`.
pub struct RtlKeys<M: Mmio> {
    mmio: M,
    pairwise: [Option<[u8; 6]>; PAIRWISE_SLOTS],
}

impl<M: Mmio> RtlKeys<M> {
    pub const fn new(mmio: M) -> Self {
        Self { mmio, pairwise: [None; PAIRWISE_SLOTS] }
    }

    // The slot a peer already holds, or the first free one.
    fn slot_for(&self, peer: &[u8; 6]) -> Option<usize> {
        if let Some(i) = self.pairwise.iter().position(|p| p.as_ref() == Some(peer)) {
            return Some(i);
        }
        self.pairwise.iter().position(Option::is_none)
    }
}

impl<M: Mmio> KeyStore for RtlKeys<M> {
    fn install_ptk(&mut self, key: &[u8; 16], key_id: u8, peer: &[u8; 6]) -> bool {
        let Some(i) = self.slot_for(peer) else {
            return false;
        };
        let entry = Key { key: *key, mac: *peer, key_idx: key_id, pairwise: true };
        write_cam(&self.mmio, PAIRWISE_BASE + i as u8, CAM_AES, &entry);
        self.pairwise[i] = Some(*peer);
        true
    }

    fn install_gtk(&mut self, key: &[u8; 16], key_id: u8) -> bool {
        // Group keys map directly onto the reserved entries 0..3.
        if key_id >= PAIRWISE_BASE {
            return false;
        }
        let entry = Key { key: *key, mac: [0xFF; 6], key_idx: key_id, pairwise: false };
        write_cam(&self.mmio, key_id, CAM_AES, &entry);
        true
    }

    fn remove_key(&mut self, key_id: u8, peer: Option<&[u8; 6]>) {
        match peer {
            Some(p) => {
                if let Some(i) = self.pairwise.iter().position(|q| q.as_ref() == Some(p)) {
                    clear_cam(&self.mmio, PAIRWISE_BASE + i as u8);
                    self.pairwise[i] = None;
                }
            }
            None => clear_cam(&self.mmio, key_id),
        }
    }
}

/// The receive DMA region as the poll path needs to see it: a CPU-readable view
/// of the buffer bytes plus the device address the card re-arms against. The
/// bring-up backs this with the mapped grant; a proof backs it with plain memory.
pub trait RxBuffers {
    /// The buffer bytes, for reading a completed frame out of the ring.
    fn bytes(&self) -> &[u8];
    /// The bus address of byte zero, for re-arming descriptors.
    fn device_addr(&self) -> u64;
}

/// The link net_core sees when the radio never came up. It reports down, holds no
/// MAC, and refuses traffic, so the driver can keep answering (and reporting how
/// far bring-up got) instead of exiting and taking its service registration with
/// it. It touches no hardware, which is why it is safe when the chip is dead.
pub struct DeadLink;

impl LinkPort for DeadLink {
    fn mac(&self) -> Option<[u8; 6]> {
        None
    }
    fn link_up(&self) -> bool {
        false
    }
    fn poll_rx(&mut self, _out: &mut [u8]) -> Option<usize> {
        None
    }
    fn send_tx(&mut self, _frame: &[u8]) -> bool {
        false
    }
}

/// A live broker grant is the receive buffer region on real hardware: its device
/// address re-arms descriptors and its mapped bytes carry the frames the card
/// wrote. The proofs back this with plain memory instead.
impl RxBuffers for Grant {
    fn bytes(&self) -> &[u8] {
        self.as_slice()
    }
    fn device_addr(&self) -> u64 {
        DmaMem::device_addr(self)
    }
}

/// The Ethernet data path net_core drives for this radio. It owns the transmit
/// and receive rings and one `LinkStation`; every WiFi specific (framing, CCMP,
/// the sequence number) lives in the station, so this is the thin ring shim the
/// shared `serve` loop expects. `D` is the descriptor/buffer DMA; `RB` the
/// readable receive buffers.
pub struct RtlLink<M: Mmio, D: DmaMem, RB: RxBuffers> {
    mmio: M,
    tx_ring: D,
    tx_buffers: D,
    tx_state: TxState,
    rx_ring: D,
    rx_buffers: RB,
    rx_state: RxState,
    station: LinkStation,
}

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    /// Build the link over already-mapped DMA and program both rings into the
    /// card. The station starts unassociated, so the link stays down until
    /// `associate` records the AP the supplicant authenticated against.
    pub fn new(
        mmio: M,
        tx_ring: D,
        tx_buffers: D,
        rx_ring: D,
        rx_buffers: RB,
        our_mac: [u8; 6],
    ) -> Self {
        tx_program(&mmio, tx_ring.device_addr());
        rx_program(&mmio, &rx_ring, rx_buffers.device_addr());
        // With the ring addresses set, reset the DMA interface so the card starts
        // advancing its receive write index; otherwise it never delivers a frame.
        crate::mac::reset_trx_dma(&mmio);
        Self {
            mmio,
            tx_ring,
            tx_buffers,
            tx_state: TxState::new(TX_DESC_COUNT),
            rx_ring,
            rx_buffers,
            rx_state: RxState::new(RX_DESC_COUNT),
            station: LinkStation::new(our_mac),
        }
    }

    /// Record association to `bssid`. Hardware CCMP: the station frames plaintext
    /// and the radio encrypts from the CAM key the `KeyStore` installed.
    pub fn associate(&mut self, bssid: [u8; 6]) {
        self.station.associate(bssid, Ccmp::Hardware);
    }

    /// Drop the association; the link reports down and refuses to transmit until
    /// it associates again.
    pub fn deassociate(&mut self) {
        self.station.deassociate();
    }

    /// Pull the next received 802.11 frame off the ring unparsed, for the scan to
    /// read beacons the data path would otherwise drop. Returns its length, or
    /// `None` when nothing is queued.
    pub fn poll_raw(&mut self, out: &mut [u8]) -> Option<usize> {
        poll_one(
            &self.mmio,
            &self.rx_ring,
            self.rx_buffers.bytes(),
            self.rx_buffers.device_addr(),
            &mut self.rx_state,
            out,
        )
    }

    /// Transmit a fully-formed 802.11 frame unencrypted, at the robust early
    /// association rate. The authentication, association and four-way-handshake
    /// frames all go out this way, before any key exists, so the security type is
    /// none. The sequence number is taken from the frame's own header.
    pub fn send_raw(&mut self, mpdu: &[u8]) -> bool {
        let seq = if mpdu.len() >= 24 {
            (u16::from_le_bytes([mpdu[22], mpdu[23]]) >> 4) & 0x0FFF
        } else {
            0
        };
        let meta = FrameMeta {
            qsel: QSEL_BE,
            bmc: false,
            rate: Some(DESC_RATE_6M),
            seq,
            sec_type: 0,
        };
        enqueue(&self.mmio, &self.tx_ring, &self.tx_buffers, &mut self.tx_state, mpdu, &meta)
    }
}

impl<M: Mmio, D: DmaMem, RB: RxBuffers> crate::assoc::Radio for RtlLink<M, D, RB> {
    fn send(&mut self, mpdu: &[u8]) -> bool {
        self.send_raw(mpdu)
    }
    fn recv(&mut self, out: &mut [u8]) -> Option<usize> {
        self.poll_raw(out)
    }
}

impl<M: Mmio, D: DmaMem, RB: RxBuffers> LinkPort for RtlLink<M, D, RB> {
    fn mac(&self) -> Option<[u8; 6]> {
        self.station.is_associated().then(|| self.station.mac())
    }

    fn link_up(&self) -> bool {
        self.station.is_associated()
    }

    fn send_tx(&mut self, frame: &[u8]) -> bool {
        let Some(mpdu) = self.station.tx_frame(frame) else {
            return false;
        };
        // Keep the descriptor sequence consistent with the header the station
        // wrote (seq control lives at offset 22, the number in its top 12 bits).
        let seq = if mpdu.len() >= 24 {
            (u16::from_le_bytes([mpdu[22], mpdu[23]]) >> 4) & 0x0FFF
        } else {
            0
        };
        let meta =
            FrameMeta { qsel: QSEL_BE, bmc: false, rate: None, seq, sec_type: SEC_TYPE_CCMP };
        enqueue(&self.mmio, &self.tx_ring, &self.tx_buffers, &mut self.tx_state, &mpdu, &meta)
    }

    fn poll_rx(&mut self, out: &mut [u8]) -> Option<usize> {
        // Until the station is associated there is no data traffic to hand up, and
        // draining the ring here would consume the beacons the background scanner
        // needs. Leave the ring untouched so the scanner sees every beacon; net_core
        // has nothing to receive on a link that is not up yet.
        if !self.station.is_associated() {
            return None;
        }
        // One receive buffer's worth of scratch for the decrypted MPDU before the
        // station parses it down to Ethernet.
        let mut mpdu = [0u8; RX_BUF_STRIDE];
        let n = poll_one(
            &self.mmio,
            &self.rx_ring,
            self.rx_buffers.bytes(),
            self.rx_buffers.device_addr(),
            &mut self.rx_state,
            &mut mpdu,
        )?;
        let eth = self.station.rx_frame(&mpdu[..n])?;
        let m = eth.len().min(out.len());
        out[..m].copy_from_slice(&eth[..m]);
        Some(m)
    }
}
