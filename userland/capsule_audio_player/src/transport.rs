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
use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::audio_client::FeedResult;
use crate::decode::Decoder;
use crate::resample::{Resampler, OUT_RATE};
const MAX_FEED_SAMPLES: usize = 2048;
pub trait FeedSink { fn feed(&mut self, pcm: &[i16]) -> FeedResult; }
#[derive(PartialEq, Clone, Copy)]
pub enum State { Stopped, Playing, Paused }
pub struct Transport<S: FeedSink> {
    state: State, pos_frames: u64, dur_frames: u64, volume_q15: i32,
    sink: S, resampler: Resampler, decoder: Option<Box<dyn Decoder>>, pending: Vec<i16>,
}
impl<S: FeedSink> Transport<S> {
    pub fn new(sink: S) -> Self { Self { state: State::Stopped, pos_frames: 0, dur_frames: 0, volume_q15: 0x8000, sink, resampler: Resampler::new(OUT_RATE, 2), decoder: None, pending: Vec::new() } }
    pub fn open(&mut self, decoder: Box<dyn Decoder>) {
        let info = decoder.info(); self.resampler = Resampler::new(info.rate, info.channels);
        self.dur_frames = info.total_frames.map(|t| t * OUT_RATE as u64 / info.rate as u64).unwrap_or(0);
        self.pos_frames = 0; self.pending.clear(); self.decoder = Some(decoder); self.state = State::Stopped;
    }
    pub fn play(&mut self) { self.state = State::Playing; } pub fn seek_frames(&mut self, f: u64) { self.pos_frames = f.min(self.dur_frames); }
    pub fn pause(&mut self) { if self.state == State::Playing { self.state = State::Paused; } }
    pub fn stop(&mut self) { self.state = State::Stopped; self.pos_frames = 0; self.pending.clear(); } pub fn set_volume(&mut self, q15: i32) { self.volume_q15 = q15.clamp(0, 0x8000); }
    pub fn state(&self) -> State { self.state } pub fn pos_frames(&self) -> u64 { self.pos_frames } pub fn dur_frames(&self) -> u64 { self.dur_frames }
    pub fn volume_q15(&self) -> i32 { self.volume_q15 } pub fn sink(&self) -> &S { &self.sink } pub fn pending_len(&self) -> usize { self.pending.len() }
    pub fn pump(&mut self) {
        if self.state != State::Playing || self.decoder.is_none() { return; }
        if let FeedResult::WouldBlock = self.drain_pending() { return; }
        let mut src = [0i16; 2048];
        let n = self.decoder.as_mut().unwrap().next(&mut src);
        if n == 0 { self.state = State::Stopped; emit_eof(); return; }
        let mut buf = Vec::new(); self.resampler.process(&src[..n], &mut buf);
        let v = self.volume_q15; for s in buf.iter_mut() { *s = ((*s as i32 * v) >> 15).clamp(-32768, 32767) as i16; }
        let acc = self.push_chunks(&buf);
        if acc < buf.len() { self.pending.extend_from_slice(&buf[acc..]); }
    }
    fn push_chunks(&mut self, buf: &[i16]) -> usize {
        let mut i = 0;
        while i < buf.len() {
            let end = (i + MAX_FEED_SAMPLES).min(buf.len());
            match self.sink.feed(&buf[i..end]) {
                FeedResult::Fed => { self.pos_frames += ((end - i) / 2) as u64; i = end; }
                FeedResult::WouldBlock => break,
            }
        }
        i
    }
    fn drain_pending(&mut self) -> FeedResult {
        let pend = core::mem::take(&mut self.pending);
        let acc = self.push_chunks(&pend);
        if acc < pend.len() { self.pending.extend_from_slice(&pend[acc..]); return FeedResult::WouldBlock; }
        FeedResult::Fed
    }
}
#[cfg(target_os = "none")]
fn emit_eof() { crate::mark::mark("[PLAYER] eof\n"); }
#[cfg(not(target_os = "none"))]
fn emit_eof() {}
