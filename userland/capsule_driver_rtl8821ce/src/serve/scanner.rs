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

//! The background scanner: a running, deduplicated picture of the networks in
//! range that the serve loop fills a little at a time between requests, so a scan
//! request is answered instantly from the cache instead of blocking the caller for
//! a full channel sweep. It only advances while the radio is up and not
//! associated, so hopping channels never disturbs a live connection.

use nonos_wifi_core::dot11::parse::parse_beacon;

use crate::fw::dma::Grant;
use crate::link::RtlLink;
use crate::phy::channel::{set_rf, Bw};
use crate::regs::Regs;
use crate::scan;

use super::{SCAN_CHANNELS, SCAN_FRAME_MAX};

/// Idle steps to dwell on one channel before hopping. With the serve loop's idle
/// timeout this is a couple of hundred milliseconds a channel, enough to catch
/// beacons (which arrive about ten times a second) without a long sweep.
const DWELL_STEPS: u32 = 2;
/// Frames to drain from the ring on one pass before yielding back to the serve
/// loop, so a busy channel never starves request handling.
const DRAIN_PER_STEP: usize = 16;

pub(super) struct Scanner {
    results: scan::ScanResults,
    frame: [u8; SCAN_FRAME_MAX],
    ch_idx: usize,
    dwell: u32,
    tuned: bool,
    // Counters, so a serial-less boot can see where the receive path stops: how
    // many passes ran, how many raw frames the ring delivered, and how many of
    // those parsed as beacons.
    pub(super) steps: u32,
    pub(super) raw: u32,
    pub(super) beacons: u32,
}

impl Scanner {
    pub(super) fn new() -> Self {
        Self {
            results: scan::ScanResults::new(),
            frame: [0u8; SCAN_FRAME_MAX],
            ch_idx: 0,
            dwell: DWELL_STEPS,
            tuned: false,
            steps: 0,
            raw: 0,
            beacons: 0,
        }
    }

    // Drain whatever the hardware has DMA-ed onto the ring on the current channel.
    // Run every serve-loop pass, request or not, so heavy net_core polling can
    // never starve the scan of the ring. Tunes the synthesizer on the first pass.
    pub(super) fn drain(&mut self, link: &mut RtlLink<Regs, Grant, Grant>, regs: &Regs) {
        if !self.tuned {
            set_rf(regs, SCAN_CHANNELS[self.ch_idx], Bw::W20);
            self.tuned = true;
        }
        self.steps = self.steps.saturating_add(1);
        for _ in 0..DRAIN_PER_STEP {
            match link.poll_raw(&mut self.frame) {
                Some(n) => {
                    self.raw = self.raw.saturating_add(1);
                    if let Some(b) = parse_beacon(&self.frame[..n]) {
                        self.beacons = self.beacons.saturating_add(1);
                        self.results.add(b.bssid, b.ssid, b.rsn);
                    }
                }
                None => break,
            }
        }
    }

    // Advance the channel hop. Called only on an idle pass, where the serve loop's
    // receive timeout has actually elapsed, so the dwell counts real time and each
    // channel gets long enough to catch a beacon before moving on.
    pub(super) fn advance(&mut self, regs: &Regs) {
        if self.dwell > 0 {
            self.dwell -= 1;
            return;
        }
        self.ch_idx = (self.ch_idx + 1) % SCAN_CHANNELS.len();
        set_rf(regs, SCAN_CHANNELS[self.ch_idx], Bw::W20);
        self.dwell = DWELL_STEPS;
    }

    pub(super) fn cache(&self) -> &scan::ScanResults {
        &self.results
    }
}
