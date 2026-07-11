use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, InputKind, PaintBuffer};
use nonos_app_skeleton::input::{KEY_LEFT, KEY_RIGHT};
use nonos_app_skeleton::discover::lookup_service;
use nonos_libc::mk_time_millis;
use crate::viewer::manifest::manifest;
use crate::viewer::state::ViewerState;
use crate::viewer::viewport::{zoom_at, clamp_pan_mode, FitMode};
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
        match event.kind {
            InputKind::Wheel => on_wheel(&mut self.st, &event),
            InputKind::ButtonDown => {
                self.st.dragging = true;
                self.st.drag_x = event.x;
                self.st.drag_y = event.y;
                EventOutcome::Idle
            }
            InputKind::PointerAbs => on_pointer(&mut self.st, &event),
            InputKind::ButtonUp => { self.st.dragging = false; EventOutcome::Idle }
            InputKind::KeyDown => on_key(&mut self.st, event.code),
            _ => EventOutcome::Idle,
        }
    }

    fn paint(&mut self, fb: &mut PaintBuffer) { render::paint(&mut self.st, fb); }

    fn on_tick(&mut self) -> bool {
        let repainted = crate::viewer::arg::poll_open(&mut self.st);
        if self.st.slideshow_on {
            let now = now_ms();
            if now.saturating_sub(self.st.last_advance_ms) >= self.st.interval_ms {
                load::step(&mut self.st, 1);
                self.st.last_advance_ms = now;
                return true;
            }
        }
        repainted
    }

    fn tick_interval_ms(&self) -> i64 { 150 }
}

fn now_ms() -> u64 { mk_time_millis().max(0) as u64 }

fn on_wheel(st: &mut ViewerState, event: &InputEvent) -> EventOutcome {
    if st.view_w == 0 { return EventOutcome::Idle; }
    let Some(img) = st.img.as_ref() else { return EventOutcome::Idle };
    let (iw, ih) = (img.w, img.h);
    let factor = if event.delta_y > 0 { 1.25 } else { 0.8 };
    zoom_at(&mut st.view, st.fit_mode, iw, ih, st.view_w, st.view_h, event.x, event.y, factor);
    EventOutcome::Repaint
}

fn on_pointer(st: &mut ViewerState, event: &InputEvent) -> EventOutcome {
    if !st.dragging { return EventOutcome::Idle; }
    st.view.pan_x += (event.x - st.drag_x) as f32;
    st.view.pan_y += (event.y - st.drag_y) as f32;
    st.drag_x = event.x;
    st.drag_y = event.y;
    if let Some(img) = st.img.as_ref() {
        clamp_pan_mode(&mut st.view, st.fit_mode, img.w, img.h, st.view_w, st.view_h);
    }
    EventOutcome::Repaint
}

fn zoom_center(st: &mut ViewerState, factor: f32) {
    let Some(img) = st.img.as_ref() else { return };
    let (iw, ih) = (img.w, img.h);
    let (cx, cy) = ((st.view_w / 2) as i32, (st.view_h / 2) as i32);
    zoom_at(&mut st.view, st.fit_mode, iw, ih, st.view_w, st.view_h, cx, cy, factor);
}

fn reset_view(st: &mut ViewerState) {
    st.view.zoom = 1.0;
    st.view.pan_x = 0.0;
    st.view.pan_y = 0.0;
}

fn on_key(st: &mut ViewerState, code: u32) -> EventOutcome {
    match code {
        KEY_LEFT => { load::step(st, -1); return EventOutcome::Repaint; }
        KEY_RIGHT => { load::step(st, 1); return EventOutcome::Repaint; }
        _ => {}
    }
    if code > 0x7F { return EventOutcome::Idle; }
    match code as u8 {
        b'+' | b'=' => zoom_center(st, 1.25),
        b'-' | b'_' => zoom_center(st, 0.8),
        b'0' => reset_view(st),
        b'f' => { st.fit_mode = FitMode::Fit; reset_view(st); }
        b'1' => { st.fit_mode = FitMode::Actual; reset_view(st); }
        b'w' => { st.fit_mode = FitMode::Fill; reset_view(st); }
        b'r' => load::rotate(st),
        b'h' => load::flip_h(st),
        b'v' => load::flip_v(st),
        b'i' => st.info_visible = !st.info_visible,
        b'?' => st.help_visible = !st.help_visible,
        b' ' => { st.slideshow_on = !st.slideshow_on; st.last_advance_ms = now_ms(); }
        b'[' => st.interval_ms = st.interval_ms.saturating_sub(500).max(1000),
        b']' => st.interval_ms = (st.interval_ms + 500).min(15000),
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
