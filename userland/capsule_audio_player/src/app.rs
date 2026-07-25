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
use crate::track::load_default;
use crate::transport::{Fed, FeedSink, State, Transport};
use crate::ui;
use crate::waveform::Waveform;

const WINDOW_ID: u32 = 0x5245_534E;
const INPUT_MASK: u32 = (1 << 0) | (1 << 3) | (1 << 5);

struct NullSink;
impl FeedSink for NullSink {
    fn open(&mut self, _format: u16) -> Result<(), &'static str> { Ok(()) }
    fn feed(&mut self, _pcm: &[i16]) -> Fed { Fed::WouldBlock }
    fn pause(&mut self) {}
    fn close(&mut self) {}
}
pub struct PlayerApp { transport: Transport, meta: TrackMeta, waveform: Waveform }

impl PlayerApp {
    pub fn new() -> Self {
        let sink: Box<dyn FeedSink> = match AudioClient::connect() {
            Ok(c) => Box::new(c),
            Err(_) => Box::new(NullSink),
        };
        let mut transport = Transport::new(sink);
        let mut meta = TrackMeta { title: String::new(), artist: String::new(), format: String::new() };
        let waveform = load_default(&mut transport, &mut meta);
        PlayerApp { transport, meta, waveform }
    }
}

impl App for PlayerApp {
    fn manifest(&self) -> AppManifest {
        AppManifest {
            title: b"Resonare", window_id: WINDOW_ID, kind: WindowKind::Normal,
            initial_x: 360, initial_y: 240, width: 480, height: 320, input_kind_mask: INPUT_MASK,
        }
    }
    fn on_event(&mut self, _event: InputEvent) -> EventOutcome { EventOutcome::Idle }
    fn paint(&mut self, fb: &mut PaintBuffer) {
        let v = self.transport.view(&self.meta);
        let l = ui::layout(fb.width, fb.height);
        ui::paint_player(fb, &v, &self.waveform, &l);
    }
    fn on_tick(&mut self) -> bool { self.transport.pump(); self.transport.state() == State::Playing }
    fn busy(&self) -> bool { self.transport.state() == State::Playing }
    fn tick_interval_ms(&self) -> i64 { if self.transport.state() == State::Playing { 10 } else { 500 } }
}
