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

//! Download a firmware image into the card. The flow: enable download, then
//! for each memory section reset the running checksum and copy it in chunks no
//! larger than one DDMA transfer, each chunk staged into the card's packet
//! buffer and then DMA-ed to its on-chip address, and finally verify the
//! section's checksum. Staging a chunk into the packet buffer is the one
//! hardware-specific step (the card's TX FIFO), so it is injected as a
//! callback that returns the DDMA source address; that keeps the whole
//! orchestration provable against a modeled device with the real firmware.

use super::ddma::{enable_download, record_section, reset_checksum, transfer};
use super::header::parse;
use super::sections::{sections, MAX_CHUNK};
use crate::regs::Mmio;

/// Why a download stopped.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DownloadError {
    /// The image is malformed (short header or sections past its end).
    BadImage,
    /// Staging a chunk into the card's packet buffer failed.
    Stage,
    /// A DDMA transfer never completed (a dead or unpowered card).
    Ddma,
    /// A section's checksum did not validate.
    Checksum,
}

/// Program `fw` into the card. `stage` loads a chunk into the packet buffer and
/// returns the DDMA source address to copy it from, or `None` on failure.
pub fn download<M, F>(mmio: &M, fw: &[u8], mut stage: F) -> Result<(), DownloadError>
where
    M: Mmio,
    F: FnMut(&[u8]) -> Option<u32>,
{
    let hdr = parse(fw).ok_or(DownloadError::BadImage)?;
    let (secs, n) = sections(&hdr, fw.len()).ok_or(DownloadError::BadImage)?;

    enable_download(mmio);
    for sec in &secs[..n] {
        reset_checksum(mmio);
        let data = &fw[sec.offset..sec.offset + sec.len];
        let mut off = 0usize;
        let mut first = true;
        while off < data.len() {
            let end = (off + MAX_CHUNK).min(data.len());
            let chunk = &data[off..end];
            let src = stage(chunk).ok_or(DownloadError::Stage)?;
            if !transfer(mmio, src, sec.dst + off as u32, chunk.len() as u32, first) {
                return Err(DownloadError::Ddma);
            }
            first = false;
            off = end;
        }
        if !record_section(mmio, sec.dst) {
            return Err(DownloadError::Checksum);
        }
    }
    Ok(())
}
