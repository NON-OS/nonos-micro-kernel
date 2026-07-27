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

use super::proto::{E_INVAL, E_OK};
use crate::mark::mark;
use crate::mixer::{Mixer, BYTES};
use crate::sink::Sink;

pub fn forward(mixer: &Mixer, sink: &Sink, request_id: u32) -> i32 {
    let mut out = [0u8; BYTES];
    mixer.write_bytes(&mut out);
    if sink.write_pcm(&out, request_id) {
        mark("[AUDIO] served\n");
        E_OK
    } else {
        E_INVAL
    }
}
