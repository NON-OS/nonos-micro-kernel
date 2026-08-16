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

use alloc::vec::Vec;

use nonos_avi::AviFile;
use nonos_libc::{mk_getpid, mk_uptime_ms};

use crate::player::{Clock, FrameDecoder, Source};

pub const WINDOW_ID: u32 = 0x5649;
pub const PATH: &[u8] = b"/video.avi";
pub const HEADER_MAX: u32 = 512 * 1024;

pub struct VideoApp {
    pub(super) source: Option<Source>,
    pub(crate) file: Option<AviFile>,
    pub(super) decoder: FrameDecoder,
    pub(crate) clock: Clock,
    pub(super) frame: Vec<u32>,
    pub(super) cols: Vec<u32>,
    pub(super) rect: (u32, u32, u32, u32),
    pub(super) geom: (u32, u32),
    pub(crate) next: u32,
    pub(super) opened: bool,
    pub(crate) playing: bool,
    pub(super) status: Option<&'static str>,
    pub(crate) volume: u32,
    pub(crate) muted: bool,
    pub(crate) dims: (u32, u32),
    pub(crate) force_decode: bool,
}

impl VideoApp {
    pub fn new() -> VideoApp {
        VideoApp {
            source: None,
            file: None,
            decoder: FrameDecoder::new(),
            clock: Clock::new(0, 0, 0),
            frame: Vec::new(),
            cols: Vec::new(),
            rect: (0, 0, 0, 0),
            geom: (0, 0),
            next: 0,
            opened: false,
            playing: true,
            status: None,
            volume: 80,
            muted: false,
            dims: (960, 600),
            force_decode: false,
        }
    }

    pub(super) fn ensure_open(&mut self) {
        if self.opened {
            return;
        }
        self.opened = true;
        if let Err(e) = self.open() {
            self.status = Some(e);
            self.playing = false;
        }
    }

    fn open(&mut self) -> Result<(), &'static str> {
        let mut source = Source::open(mk_getpid(), PATH)?;
        let head = source.read_header(HEADER_MAX)?;
        let file = AviFile::parse(&head).map_err(|_| "not a playable avi")?;
        let pixels = (file.video.width as usize)
            .checked_mul(file.video.height as usize)
            .ok_or("frame dimensions overflow")?;
        self.frame.try_reserve(pixels).map_err(|_| "frame too large")?;
        self.frame.resize(pixels, 0xff00_0000);
        self.clock = Clock::new(mk_uptime_ms(), 0, file.header.micro_sec_per_frame);
        self.source = Some(source);
        self.file = Some(file);
        Ok(())
    }
}
