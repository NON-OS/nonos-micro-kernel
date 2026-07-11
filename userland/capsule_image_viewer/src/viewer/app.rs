use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer};
use nonos_app_skeleton::input::{KEY_LEFT, KEY_RIGHT};
use nonos_app_skeleton::discover::lookup_service;
use crate::viewer::manifest::manifest;
use crate::viewer::state::ViewerState;
use crate::viewer::{load, render};

pub struct ViewerApp { st: ViewerState }

impl ViewerApp {
    pub fn new() -> Self {
        let mut st = ViewerState::new();
        st.owner_pid = lookup_service(b"app.image_viewer").map(|s| s.pid).unwrap_or(0);
        ViewerApp { st }
    }
}

impl App for ViewerApp {
    fn manifest(&self) -> AppManifest { manifest() }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        if !event.is_key_down() { return EventOutcome::Idle; }
        match event.code {
            KEY_LEFT => { load::step(&mut self.st, -1); EventOutcome::Repaint }
            KEY_RIGHT => { load::step(&mut self.st, 1); EventOutcome::Repaint }
            c if c <= 0x7F => match c as u8 {
                b'+' | b'=' => { self.st.view.zoom = (self.st.view.zoom * 1.25).min(16.0); EventOutcome::Repaint }
                b'-' | b'_' => { self.st.view.zoom = (self.st.view.zoom / 1.25).max(0.05); EventOutcome::Repaint }
                b'r' | b'R' => { load::rotate(&mut self.st); EventOutcome::Repaint }
                b'f' | b'F' | b'0' => { self.st.view.zoom = 1.0; self.st.view.pan_x = 0.0; self.st.view.pan_y = 0.0; EventOutcome::Repaint }
                _ => EventOutcome::Idle,
            },
            _ => EventOutcome::Idle,
        }
    }

    fn paint(&mut self, fb: &mut PaintBuffer) { render::paint(&mut self.st, fb); }

    fn on_tick(&mut self) -> bool {
        crate::viewer::arg::poll_open(&mut self.st)
    }

    fn tick_interval_ms(&self) -> i64 { 150 }
}
