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

//! Drive the firmware download on real hardware. This is the thin plumbing the
//! provable download core cannot be: it maps the two DMA regions the reserved
//! page path needs (a staging buffer for one chunk plus its TX descriptor, and
//! the beacon-queue buffer-descriptor ring), points the card at the ring, then
//! brackets `fw::download` with the prologue and epilogue. The firmware image is
//! embedded so the capsule carries its own blob. Everything downstream of the
//! `stage` closure is proven off-silicon in `rtl8821ce_proofs`; the DMA mapping
//! and the card actually running the firmware are the on-silicon boundary.

use nonos_libc::{mk_dma_map, DmaMapOut};

use crate::fw::dma::{DmaMem, Grant};
use crate::fw::header::parse;
use crate::fw::regs::{REG_TXBD_DESA_BCNQ, TX_DESC_SIZE};
use crate::fw::{download, prep, rsvd, DownloadError, MAX_CHUNK};
use crate::regs::{Mmio, Regs};
use crate::status;

/// The RTL8821CE firmware image, redistributed by linux-firmware and carried in
/// the capsule so no filesystem is needed at bring-up.
const FW: &[u8] = include_bytes!("../firmware/rtw8821c_fw.bin");

/// Staging buffer: one DDMA chunk plus its 48-byte TX descriptor, rounded to
/// whole pages. Two pages cover the 0x1000-byte chunk and the descriptor.
const STAGE_BYTES: u64 = 0x2000;
/// The beacon-queue buffer-descriptor ring. One page is ample for the single
/// descriptor firmware download reuses.
const RING_BYTES: u64 = 0x1000;

// The staging buffer must hold the largest chunk plus its TX descriptor.
const _: () = assert!(STAGE_BYTES as usize >= MAX_CHUNK + TX_DESC_SIZE as usize);

/// Why the firmware download could not be set up or completed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FwLoadError {
    /// The embedded image is not an RTL8821C firmware, or is malformed.
    BadImage,
    /// A DMA region could not be mapped through the broker.
    Dma,
    /// The download itself failed; carries the stage it stopped at.
    Download(DownloadError),
    /// The firmware never reported ready after the sections were delivered.
    NotReady,
}

/// Download the embedded firmware into the powered chip. `device_id` and
/// `claim_epoch` are the same claim the register window was mapped under.
pub fn load(regs: &Regs, device_id: u64, claim_epoch: u64) -> Result<(), FwLoadError> {
    // Confirm the embedded blob is this chip's firmware, and note its version on
    // the console, before touching the card.
    let hdr = parse(FW).ok_or(FwLoadError::BadImage)?;
    if !hdr.is_for_8821c() {
        return Err(FwLoadError::BadImage);
    }
    let (major, minor) = hdr.version();
    status::line(b"[rtl8821ce] firmware v");
    status::number(major as u32);
    status::line(b".");
    status::number(minor as u32);
    status::line(b"\n");

    let Some(stage_buf) = map_dma(device_id, claim_epoch, STAGE_BYTES) else {
        status::line(b"[rtl8821ce] fw: staging DMA map failed\n");
        return Err(FwLoadError::Dma);
    };
    let Some(ring) = map_dma(device_id, claim_epoch, RING_BYTES) else {
        status::line(b"[rtl8821ce] fw: ring DMA map failed\n");
        return Err(FwLoadError::Dma);
    };

    // The card addresses staged pages with a 32-bit descriptor field, so a
    // buffer above 4GB would be silently truncated and the beacon DMA would read
    // the wrong memory. Report the high half of each address: a non-zero value
    // here is the reason a staged page never validates on a large-memory machine.
    let stage_hi = (stage_buf.device_addr() >> 32) as u32;
    let ring_hi = (ring.device_addr() >> 32) as u32;
    if stage_hi != 0 || ring_hi != 0 {
        status::line(b"[rtl8821ce] fw: DMA buffer above 4GB (stage hi=");
        status::number(stage_hi);
        status::line(b" ring hi=");
        status::number(ring_hi);
        status::line(b"), 32-bit descriptor cannot address it\n");
    }

    // Point the card at the beacon-queue ring before the first kick reads it.
    regs.write32(REG_TXBD_DESA_BCNQ, ring.device_addr() as u32);
    regs.write32(REG_TXBD_DESA_BCNQ + 4, (ring.device_addr() >> 32) as u32);

    let backup = prep::begin(regs);
    let result = download(regs, FW, |chunk| rsvd::stage_chunk(regs, &stage_buf, &ring, chunk));
    let finished = prep::finish(regs, backup);

    // Name the exact sub-step that failed on the console, so a serial-less boot
    // says which of staging, DDMA, checksum or the 8051 boot went wrong.
    if let Err(e) = result {
        status::line(match e {
            DownloadError::BadImage => b"[rtl8821ce] fw: bad image\n".as_slice(),
            DownloadError::Stage => b"[rtl8821ce] fw: reserved-page staging failed\n".as_slice(),
            DownloadError::Ddma => b"[rtl8821ce] fw: ddma transfer failed\n".as_slice(),
            DownloadError::Checksum => b"[rtl8821ce] fw: section checksum failed\n".as_slice(),
        });
        return Err(FwLoadError::Download(e));
    }
    // The close-out separates a bad-checksum download (the DDMA delivered wrong
    // bytes) from an 8051 that never booted, and prints the control register so
    // its individual bits say how far the handshake reached without a serial port.
    match finished {
        prep::Finish::Ready => Ok(()),
        prep::Finish::ChecksumBad(ctrl) => {
            status::line(b"[rtl8821ce] fw: hardware checksum not ok, ctrl=0x");
            status::hex16(ctrl);
            status::line(b" (ddma delivered bad data)\n");
            Err(FwLoadError::NotReady)
        }
        prep::Finish::NotReady(ctrl) => {
            status::line(b"[rtl8821ce] fw: 8051 not ready, ctrl=0x");
            status::hex16(ctrl);
            status::line(b"\n");
            Err(FwLoadError::NotReady)
        }
    }
}

/// Map one DMA region and wrap it as a coherent buffer with its device address.
/// Shared with the serving stage, which maps the transmit and receive rings the
/// same way.
pub(crate) fn map_dma(device_id: u64, claim_epoch: u64, bytes: u64) -> Option<Grant> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    if mk_dma_map(device_id, claim_epoch, bytes, 0, &mut out) < 0 {
        return None;
    }
    Some(Grant::new(out.user_va, out.device_addr, out.length as usize))
}
