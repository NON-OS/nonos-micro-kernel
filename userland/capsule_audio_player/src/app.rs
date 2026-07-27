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
use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, InputKind, PaintBuffer, WindowKind};
use crate::audio_client::AudioClient;
use crate::library::{Library, Queue};
use crate::ui::control::{control_at, Control};
use crate::model::TrackMeta;
use crate::track::{load_default, load_track};
use crate::transport::{Fed, FeedSink, State, Transport};
use crate::ui;
use crate::waveform::Waveform;

const WINDOW_ID: u32 = 0x5245_534E;
const WINDOW_W: u32 = 720;
const WINDOW_H: u32 = 520;
const INPUT_MASK: u32 = (1 << 0) | (1 << 3) | (1 << 5);

struct NullSink;
impl FeedSink for NullSink {
    fn open(&mut self, _format: u16) -> Result<(), &'static str> { Ok(()) }
    fn feed(&mut self, _pcm: &[i16]) -> Fed { Fed::WouldBlock }
    fn pause(&mut self) {}
    fn resume(&mut self) {}
    fn close(&mut self) {}
}

#[derive(Clone, Copy, PartialEq)]
enum ViewMode { NowPlaying, Library }

pub struct PlayerApp { transport: Transport, meta: TrackMeta, waveform: Waveform, library: Library, queue: Queue, shuffle: bool, repeat: bool, view: ViewMode, dims: (u32, u32) }

impl PlayerApp {
    pub fn new() -> Self {
        let sink: Box<dyn FeedSink> = match AudioClient::connect() {
            Ok(c) => Box::new(c),
            Err(_) => Box::new(NullSink),
        };
        let mut transport = Transport::new(sink);
        let mut meta = TrackMeta { title: String::new(), artist: String::new(), format: String::new() };
        let library = Library::scan();
        let mut queue = Queue::new();
        for i in 0..library.tracks.len() {
            queue.enqueue(i);
        }
        let waveform = match queue.current().and_then(|i| library.get(i)) {
            Some(t) => load_track(&mut transport, &mut meta, t),
            None => load_default(&mut transport, &mut meta),
        };
        PlayerApp { transport, meta, waveform, library, queue, shuffle: false, repeat: false, view: ViewMode::NowPlaying, dims: (WINDOW_W, WINDOW_H) }
    }

    fn select(&mut self, i: usize) {
        self.queue.focus(i);
        self.load_index(i);
        self.view = ViewMode::NowPlaying;
    }

    fn on_click(&mut self, l: &ui::Layout, x: i32, y: i32) -> EventOutcome {
        if l.now_tab.contains(x, y) {
            self.view = ViewMode::NowPlaying;
            return EventOutcome::Repaint;
        }
        if l.lib_tab.contains(x, y) {
            self.view = ViewMode::Library;
            return EventOutcome::Repaint;
        }
        if self.view == ViewMode::Library {
            return match ui::list_row_at(&l.list, self.library.tracks.len(), x, y) {
                Some(i) => { self.select(i); EventOutcome::Repaint }
                None => EventOutcome::Idle,
            };
        }
        match control_at(l, x, y) {
            Some(Control::Prev) => { self.prev_track(); EventOutcome::Repaint }
            Some(Control::Next) => { self.next_track(); EventOutcome::Repaint }
            Some(Control::Shuffle) => { self.toggle_shuffle(); EventOutcome::Repaint }
            Some(Control::Repeat) => { self.repeat = !self.repeat; EventOutcome::Repaint }
            Some(c) => { ui::event::apply(&mut self.transport, c); EventOutcome::Repaint }
            None => EventOutcome::Idle,
        }
    }

    fn on_eof(&mut self) {
        if self.repeat || self.queue.has_next() {
            self.next_track();
        }
    }

    fn toggle_shuffle(&mut self) {
        self.shuffle = !self.shuffle;
        if self.shuffle {
            self.queue.shuffle(nonos_libc::mk_time_millis() as u64);
        } else {
            self.queue.restore_order();
        }
    }

    fn prev_track(&mut self) {
        if let Some(i) = self.queue.back() {
            self.load_index(i);
        }
    }

    fn next_track(&mut self) {
        if self.repeat {
            if let Some(i) = self.queue.current() {
                self.load_index(i);
            }
            return;
        }
        if let Some(i) = self.queue.advance() {
            self.load_index(i);
        }
    }

    fn load_index(&mut self, i: usize) {
        if let Some(t) = self.library.get(i) {
            self.waveform = load_track(&mut self.transport, &mut self.meta, t);
            self.transport.play();
        }
    }
}

impl App for PlayerApp {
    fn manifest(&self) -> AppManifest {
        AppManifest {
            title: b"Resonare", window_id: WINDOW_ID, kind: WindowKind::Normal,
            initial_x: 150, initial_y: 110, width: WINDOW_W, height: WINDOW_H, input_kind_mask: INPUT_MASK,
        }
    }
    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        let l = ui::layout(self.dims.0, self.dims.1);
        match event.kind {
            InputKind::ButtonDown => self.on_click(&l, event.x, event.y),
            InputKind::KeyDown => ui::event::key(&mut self.transport, event.code),
            _ => EventOutcome::Idle,
        }
    }
    fn paint(&mut self, fb: &mut PaintBuffer) {
        self.dims = (fb.width, fb.height);
        let l = ui::layout(fb.width, fb.height);
        if self.view == ViewMode::Library {
            ui::paint_library(fb, &self.library, &self.queue, self.queue.current(), &l);
        } else {
            let mut v = self.transport.view(&self.meta);
            v.shuffle = self.shuffle;
            v.repeat = self.repeat;
            ui::paint_player(fb, &v, &self.waveform, &l);
        }
    }
    fn on_tick(&mut self) -> bool {
        let was = self.transport.state();
        self.transport.pump();
        if was == State::Playing && self.transport.state() == State::Stopped {
            self.on_eof();
        }
        self.transport.state() == State::Playing
    }
    fn busy(&self) -> bool { self.transport.state() == State::Playing }
    fn tick_interval_ms(&self) -> i64 { if self.transport.state() == State::Playing { 10 } else { 500 } }
}
