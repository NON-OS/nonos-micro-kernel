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

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use crate::decode::{self, Decoder};
use crate::loader;
use crate::model::TrackMeta;
use crate::track_fmt::format_of;
use crate::transport::Transport;
use crate::waveform::{from_samples, Waveform};

const DEFAULT_PATH: &[u8] = b"/audio/boot_tone.wav";

pub fn load_default(transport: &mut Transport, meta: &mut TrackMeta) -> Waveform {
    let bytes = match loader::load(DEFAULT_PATH) {
        Ok(b) => b,
        Err(_) => return from_samples(&[], 96),
    };
    let mut probe = match decode::open(bytes.clone()) {
        Ok(d) => d,
        Err(_) => return from_samples(&[], 96),
    };
    meta.title = String::from("Boot Tone");
    meta.artist = String::from("NONOS");
    meta.format = format_of(&*probe);
    let pcm = drain(&mut *probe);
    if let Ok(dec) = decode::open(bytes) {
        let _ = transport.open(dec);
    }
    from_samples(&pcm, 96)
}

fn drain(dec: &mut dyn Decoder) -> Vec<i16> {
    let mut out: Vec<i16> = Vec::new();
    let mut buf = [0i16; 4096];
    loop {
        let n = dec.next(&mut buf);
        if n == 0 {
            break;
        }
        out.extend_from_slice(&buf[..n]);
    }
    out
}
