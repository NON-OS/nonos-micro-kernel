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
use alloc::vec::Vec;
use nonos_app_skeleton::{
    App, AppManifest, EventOutcome, InputEvent, InputKind, PaintBuffer, WindowKind,
};

use crate::audio_client::AudioClient;
use crate::library::{Library, Queue};
use crate::model::TrackMeta;
use crate::track::{load_default, load_track};
use crate::transport::{Fed, FeedSink, State, Transport};
use crate::ui;
use crate::ui::{Action, Frame, Scene, UiState, View};
use crate::waveform::Waveform;

const WINDOW_ID: u32 = 0x5245_534E;
const WINDOW_W: u32 = 1440;
const WINDOW_H: u32 = 900;
const INPUT_MASK: u32 = (1 << 0) | (1 << 3) | (1 << 4) | (1 << 5);
const FRAME_TICKS: u32 = 5;

struct NullSink;
impl FeedSink for NullSink {
    fn open(&mut self, _format: u16) -> Result<(), &'static str> {
        Ok(())
    }
    fn feed(&mut self, _pcm: &[i16]) -> Fed {
        Fed::WouldBlock
    }
    fn pause(&mut self) {}
    fn resume(&mut self) {}
    fn close(&mut self) {}
}

pub struct PlayerApp {
    transport: Transport,
    meta: TrackMeta,
    waveform: Waveform,
    library: Library,
    queue: Queue,
    shuffle: bool,
    repeat: bool,
    ui: UiState,
    frame: Frame,
    ticks: u32,
    step: u32,
    dims: (u32, u32),
    retries: u8,
}

impl PlayerApp {
    pub fn new() -> PlayerApp {
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
        PlayerApp {
            transport,
            meta,
            waveform,
            library,
            queue,
            shuffle: false,
            repeat: false,
            ui: UiState::new(),
            frame: Frame::new(),
            ticks: 0,
            step: 0,
            dims: (WINDOW_W, WINDOW_H),
            retries: 24,
        }
    }

    fn rescan(&mut self) -> bool {
        if self.retries == 0 || !self.library.tracks.is_empty() {
            return false;
        }
        self.retries -= 1;
        self.library = Library::scan();
        if self.library.tracks.is_empty() {
            return false;
        }
        self.retries = 0;
        self.queue = Queue::new();
        for i in 0..self.library.tracks.len() {
            self.queue.enqueue(i);
        }
        let Self { library, queue, transport, meta, waveform, .. } = self;
        if let Some(t) = queue.current().and_then(|i| library.get(i)) {
            *waveform = load_track(transport, meta, t);
        }
        true
    }

    fn rows(&self) -> Vec<usize> {
        match self.ui.view {
            View::Library => ui::screen::rows_for(&self.library, &self.queue, self.ui.lib_tab),
            View::Search => ui::state::filtered(&self.library, &self.ui.query),
            _ => Vec::new(),
        }
    }

    fn select(&mut self, i: usize) {
        self.queue.focus(i);
        self.load_index(i);
    }

    fn act(&mut self, a: Action) -> EventOutcome {
        match a {
            Action::Go(v) => self.ui.go(v),
            Action::Select(i) => self.select(i),
            Action::LibTab(t) => {
                self.ui.lib_tab = t;
                self.ui.scroll = 0;
            }
            Action::RailTab(t) => self.ui.rail_tab = t,
            Action::Section(i) => self.ui.set_sec = i,
            Action::Playlist(i) => {
                self.ui.playlist = if self.ui.playlist == Some(i) { None } else { Some(i) };
                self.ui.scroll = 0;
            }
            Action::ClearQuery => {
                self.ui.query.clear();
                self.ui.scroll = 0;
            }
            Action::Ctl(ui::Control::Prev) => self.prev_track(),
            Action::Ctl(ui::Control::Next) => self.next_track(),
            Action::Ctl(ui::Control::Shuffle) => self.toggle_shuffle(),
            Action::Ctl(ui::Control::Repeat) => self.repeat = !self.repeat,
            Action::Ctl(c) => ui::event::apply(&mut self.transport, c),
        }
        EventOutcome::Repaint
    }

    fn on_click(&mut self, x: i32, y: i32) -> EventOutcome {
        let rows = self.rows();
        let n = self.library.tracks.len();
        match ui::hit(&self.ui, self.dims, &rows, n, x, y) {
            Some(a) => self.act(a),
            None => EventOutcome::Idle,
        }
    }

    fn on_move(&mut self, x: i32, y: i32) -> EventOutcome {
        let rows = self.rows();
        let n = self.library.tracks.len();
        let h = match ui::hit(&self.ui, self.dims, &rows, n, x, y) {
            Some(Action::Select(i)) => Some(i),
            _ => None,
        };
        if h == self.ui.hover {
            return EventOutcome::Idle;
        }
        self.ui.hover = h;
        EventOutcome::Repaint
    }

    fn on_wheel(&mut self, delta: i32) -> EventOutcome {
        let len = self.rows().len();
        let visible = ui::screen::lib_visible(ui::geometry::page(&ui::geometry::shell(
            self.dims.0,
            self.dims.1,
        )));
        self.ui.scroll_by(if delta > 0 { -3 } else { 3 }, len, visible);
        EventOutcome::Repaint
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
            title: b"Resonare",
            window_id: WINDOW_ID,
            kind: WindowKind::Normal,
            initial_x: 60,
            initial_y: 40,
            width: WINDOW_W,
            height: WINDOW_H,
            input_kind_mask: INPUT_MASK,
        }
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        match event.kind {
            InputKind::ButtonDown => self.on_click(event.x, event.y),
            InputKind::PointerAbs => self.on_move(event.x, event.y),
            InputKind::Wheel => self.on_wheel(event.delta_y),
            InputKind::KeyDown => ui::event::key(&mut self.transport, event.code),
            _ => EventOutcome::Idle,
        }
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        self.rescan();
        self.dims = (fb.width, fb.height);
        let mut v = self.transport.view(&self.meta);
        v.shuffle = self.shuffle;
        v.repeat = self.repeat;
        let rows = self.rows();
        let playing = self.queue.current();
        let id = playing.and_then(|i| self.library.get(i)).map_or("", |t| t.path.as_str());
        let scene = Scene {
            lib: &self.library,
            queue: &self.queue,
            rows: &rows,
            view: &v,
            wave: &self.waveform,
            playing,
            id,
            step: self.step,
        };
        self.frame.paint(fb, &self.ui, &scene);
    }

    fn on_tick(&mut self) -> bool {
        if self.rescan() {
            return true;
        }
        let was = self.transport.state();
        self.transport.pump();
        if was == State::Playing && self.transport.state() == State::Stopped {
            self.on_eof();
        }
        if !ui::control::playing(&self.transport) {
            return false;
        }
        self.ticks += 1;
        if self.ticks < FRAME_TICKS {
            return false;
        }
        self.ticks = 0;
        self.step = self.step.wrapping_add(1);
        true
    }

    fn busy(&self) -> bool {
        ui::control::playing(&self.transport)
    }

    fn tick_interval_ms(&self) -> i64 {
        if ui::control::playing(&self.transport) {
            10
        } else {
            500
        }
    }
}
