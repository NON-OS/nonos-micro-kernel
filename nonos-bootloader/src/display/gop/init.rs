// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::state::{FB_FORMAT_BGR, FB_HEIGHT, FB_INITIALIZED, FB_PTR, FB_STRIDE, FB_WIDTH};
use core::sync::atomic::Ordering;
use uefi::prelude::*;
use uefi::proto::console::gop::{GraphicsOutput, ModeInfo, PixelFormat};
use uefi::Identify;

pub fn init_gop(st: &mut SystemTable<Boot>) -> bool {
    let bs = st.boot_services();
    match bs.locate_handle_buffer(uefi::table::boot::SearchType::ByProtocol(&GraphicsOutput::GUID))
    {
        Ok(h) => {
            for &hnd in h.iter() {
                if try_init(bs, hnd) {
                    return true;
                }
            }
            false
        }
        Err(_) => {
            bs.get_handle_for_protocol::<GraphicsOutput>().map(|h| try_init(bs, h)).unwrap_or(false)
        }
    }
}

// A development build can pin the exact mode the firmware should set, e.g.
// "1280x800" so the QEMU window opens at a size that fits the host screen.
// Unset (every hardware build), the picker keeps its largest-mode behaviour.
const PREFERRED_MODE: Option<&str> = option_env!("NONOS_GOP_PREF");

pub(super) fn preferred_mode() -> Option<(usize, usize)> {
    let s = PREFERRED_MODE?;
    let (w, h) = s.split_once('x')?;
    Some((w.parse().ok()?, h.parse().ok()?))
}

fn try_init(bs: &uefi::table::boot::BootServices, h: Handle) -> bool {
    let mut gop = match bs.open_protocol_exclusive::<GraphicsOutput>(h) {
        Ok(g) => g,
        Err(_) => return false,
    };
    let prefer = preferred_mode();
    let mut best: Option<(u32, usize)> = None;
    let mode_count = gop.modes().len();
    for idx in 0..mode_count {
        let mode = match gop.query_mode(idx as u32) {
            Ok(mode) => mode,
            Err(_) => continue,
        };
        let info = mode.info();
        if linear_bgr(info).is_none() {
            continue;
        }
        let (w, ht) = info.resolution();
        if prefer == Some((w, ht)) {
            best = Some((idx as u32, usize::MAX));
            break;
        }
        if w > 1920 || ht > 1080 {
            continue;
        }
        let area = w.saturating_mul(ht);
        if best.map_or(true, |(_, a)| area > a) {
            best = Some((idx as u32, area));
        }
    }
    if let Some((idx, _)) = best {
        if let Ok(mode) = gop.query_mode(idx) {
            if gop.set_mode(&mode).is_ok() && latch_current_mode(&mut gop) {
                return true;
            }
        }
    }
    latch_current_mode(&mut gop)
}

fn latch_current_mode(gop: &mut GraphicsOutput) -> bool {
    let info = gop.current_mode_info();
    let bgr = match linear_bgr(&info) {
        Some(v) => v,
        None => return false,
    };
    let (w, ht) = info.resolution();
    let stride = info.stride();
    if w == 0 || ht == 0 || stride < w {
        return false;
    }
    let mut fb = gop.frame_buffer();
    let fb_addr = fb.as_mut_ptr() as u64;
    if fb_addr == 0 || !fb_covers_mode(fb.size(), stride, ht) {
        return false;
    }
    FB_PTR.store(fb_addr, Ordering::SeqCst);
    FB_WIDTH.store(w as u32, Ordering::SeqCst);
    FB_HEIGHT.store(ht as u32, Ordering::SeqCst);
    FB_STRIDE.store(stride as u32, Ordering::SeqCst);
    FB_FORMAT_BGR.store(bgr, Ordering::SeqCst);
    FB_INITIALIZED.store(true, Ordering::SeqCst);
    true
}

fn linear_bgr(info: &ModeInfo) -> Option<bool> {
    match info.pixel_format() {
        PixelFormat::Rgb => Some(false),
        PixelFormat::Bgr => Some(true),
        PixelFormat::Bitmask | PixelFormat::BltOnly => None,
    }
}

fn fb_covers_mode(fb_size: usize, stride: usize, height: usize) -> bool {
    stride
        .checked_mul(height)
        .and_then(|px| px.checked_mul(core::mem::size_of::<u32>()))
        .map_or(false, |needed| fb_size >= needed)
}
