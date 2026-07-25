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
use alloc::string::String;
use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer, WindowKind};
use crate::audio_client::AudioClient;
use crate::model::TrackMeta;
use crate::transport::{Fed, FeedSink, State, Transport};

const WINDOW_ID: u32 = 0x5245_534E;
const BG: u32 = 0xFF10161C; const FG: u32 = 0xFFE8EEF4;
const BAR: u32 = 0xFF3A9BDC; const TRK: u32 = 0xFF223040;

struct NullSink;
impl FeedSink for NullSink {
    fn open(&mut self, _format: u16) -> Result<(), &'static str> { Ok(()) }
    fn feed(&mut self, _pcm: &[i16]) -> Fed { Fed::WouldBlock }
    fn pause(&mut self) {}
    fn close(&mut self) {}
}
pub struct PlayerApp { transport: Transport, meta: TrackMeta }

impl PlayerApp {
    pub fn new() -> Self {
        let sink: Box<dyn FeedSink> = match AudioClient::connect() {
            Ok(c) => Box::new(c),
            Err(_) => Box::new(NullSink),
        };
        let meta = TrackMeta { title: String::new(), artist: String::new(), format: String::new() };
        PlayerApp { transport: Transport::new(sink), meta }
    }
}

impl App for PlayerApp {
    fn manifest(&self) -> AppManifest {
        AppManifest {
            title: b"Resonare", window_id: WINDOW_ID, kind: WindowKind::Normal,
            initial_x: 360, initial_y: 240, width: 480, height: 320, input_kind_mask: 0,
        }
    }
    fn on_event(&mut self, _event: InputEvent) -> EventOutcome { EventOutcome::Idle }
    fn paint(&mut self, fb: &mut PaintBuffer) {
        let v = self.transport.view(&self.meta);
        fb.clear(BG);
        fb.text(24, 32, v.title.as_bytes(), FG);
        fb.text(24, 56, v.artist.as_bytes(), FG);
        fb.text(24, 80, v.format.as_bytes(), FG);
        let label = match v.state { State::Playing => b"PLAYING" as &[u8], State::Paused => b"PAUSED", State::Stopped => b"STOPPED" };
        fb.text(24, 104, label, FG);
        let pw = if v.dur_ms == 0 { 0 } else { (432u64 * v.pos_ms as u64 / v.dur_ms as u64) as u32 };
        fb.fill_rect(24, 160, 432, 12, TRK);
        fb.fill_rect(24, 160, pw, 12, BAR);
        let vw = (432u32 * v.volume_q15.max(0) as u32) / 0x8000;
        fb.fill_rect(24, 200, 432, 12, TRK);
        fb.fill_rect(24, 200, vw, 12, BAR);
    }
    fn on_tick(&mut self) -> bool { self.transport.pump(); self.transport.state() == State::Playing }
    fn busy(&self) -> bool { self.transport.state() == State::Playing }
    fn tick_interval_ms(&self) -> i64 { if self.transport.state() == State::Playing { 10 } else { 500 } }
}
