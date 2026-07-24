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

use super::proto::{Request, E_INVAL, E_OK, HDR_LEN};
use super::pump::PumpState;
use super::serve::forward;
use super::streams::StreamTable;
use super::tone;
use crate::mixer::{Mixer, SAMPLES};
use crate::sink::Sink;

const TONE_PAYLOAD: usize = 12;
const PCM_HDR: usize = 8;
const STREAM_FEED_HDR: usize = 8;

fn payload<'a>(req: &Request, msg: &'a [u8]) -> &'a [u8] {
    let avail = msg.len().saturating_sub(HDR_LEN);
    let plen = (req.payload_len as usize).min(avail);
    &msg[HDR_LEN..HDR_LEN + plen]
}

pub fn play_tone(req: &Request, msg: &[u8], mixer: &mut Mixer, sink: &Sink) -> i32 {
    let b = payload(req, msg);
    if b.len() < TONE_PAYLOAD {
        return E_INVAL;
    }
    let freq = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
    let ms = u32::from_le_bytes([b[4], b[5], b[6], b[7]]);
    let gain = u16::from_le_bytes([b[8], b[9]]);
    let mut buf = [0i16; SAMPLES];
    tone::synth(freq, ms, gain, &mut buf);
    mixer.add(&buf);
    forward(mixer, sink, req.request_id)
}

pub fn play_pcm(req: &Request, msg: &[u8], mixer: &mut Mixer, sink: &Sink) -> i32 {
    let b = payload(req, msg);
    if b.len() < PCM_HDR {
        return E_INVAL;
    }
    let nframes = u32::from_le_bytes([b[4], b[5], b[6], b[7]]) as usize;
    let samples = nframes.saturating_mul(2);
    let need = samples.saturating_mul(2);
    if samples == 0 || samples > SAMPLES || b.len() < PCM_HDR + need {
        return E_INVAL;
    }
    let mut buf = [0i16; SAMPLES];
    let p = &b[PCM_HDR..PCM_HDR + need];
    let mut i = 0usize;
    while i < samples {
        buf[i] = i16::from_le_bytes([p[i * 2], p[i * 2 + 1]]);
        i += 1;
    }
    mixer.add(&buf[..samples]);
    forward(mixer, sink, req.request_id)
}

pub fn stream_open(req: &Request, msg: &[u8], table: &mut StreamTable) -> (i32, u32) {
    let b = payload(req, msg);
    let format = if b.len() >= 2 { u16::from_le_bytes([b[0], b[1]]) } else { 0 };
    match table.open(format) {
        Some(id) => (E_OK, id),
        None => (E_INVAL, 0),
    }
}

pub fn stream_feed(
    req: &Request,
    msg: &[u8],
    table: &mut StreamTable,
    pump: &mut PumpState,
    sink: &Sink,
) -> i32 {
    let b = payload(req, msg);
    if b.len() < STREAM_FEED_HDR {
        return E_INVAL;
    }
    let stream_id = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
    let nframes = u32::from_le_bytes([b[4], b[5], b[6], b[7]]) as usize;
    let samples = nframes.saturating_mul(2);
    let need = samples.saturating_mul(2);
    if samples == 0 || samples > SAMPLES || b.len() < STREAM_FEED_HDR + need {
        return E_INVAL;
    }
    let mut buf = [0i16; SAMPLES];
    let p = &b[STREAM_FEED_HDR..STREAM_FEED_HDR + need];
    let mut i = 0usize;
    while i < samples {
        buf[i] = i16::from_le_bytes([p[i * 2], p[i * 2 + 1]]);
        i += 1;
    }
    let status = table.feed(stream_id, &buf[..samples]);
    if status == E_OK {
        super::pump::step(pump, table, sink);
    }
    status
}

pub fn stream_pause(req: &Request, msg: &[u8], table: &mut StreamTable) -> i32 {
    let b = payload(req, msg);
    if b.len() < 4 {
        return E_INVAL;
    }
    let id = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
    if table.set_paused(id, true) { E_OK } else { E_INVAL }
}

pub fn stream_close(req: &Request, msg: &[u8], table: &mut StreamTable) -> i32 {
    let b = payload(req, msg);
    if b.len() < 4 {
        return E_INVAL;
    }
    let id = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
    if table.close(id) { E_OK } else { E_INVAL }
}
