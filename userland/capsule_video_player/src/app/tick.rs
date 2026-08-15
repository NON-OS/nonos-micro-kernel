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

use nonos_libc::mk_uptime_ms;

use super::state::VideoApp;
use crate::player::Step;

impl VideoApp {
    pub(super) fn advance(&mut self) -> bool {
        self.ensure_open();
        if !self.playing {
            return false;
        }
        let total = match self.file.as_ref() {
            Some(f) => f.index.len() as u32,
            None => return false,
        };
        match self.clock.step(self.next, total, mk_uptime_ms()) {
            Step::End => {
                self.playing = false;
                false
            }
            Step::Wait => false,
            Step::Skip(i) => {
                self.next = i.saturating_add(1);
                false
            }
            Step::Show(i) => {
                let painted = match self.decode_one(i) {
                    Ok(()) => true,
                    Err(e) => {
                        self.status = Some(e);
                        false
                    }
                };
                self.next = i.saturating_add(1);
                painted
            }
        }
    }

    fn decode_one(&mut self, i: u32) -> Result<(), &'static str> {
        let VideoApp { source, file, decoder, frame, .. } = self;
        let source = source.as_mut().ok_or("no source")?;
        let file = file.as_ref().ok_or("no file")?;
        let fr = file.index.get(i as usize).ok_or("frame out of range")?;
        let bytes = source.bytes_at(fr.offset, fr.len)?;
        decoder.decode(bytes, file.video.width, file.video.height, frame)
    }
}
