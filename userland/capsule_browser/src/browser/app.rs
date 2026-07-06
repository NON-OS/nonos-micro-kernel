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

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer};

use super::event::on_event;
use super::manifest::manifest;
use super::paint::paint;
use super::state::State;

pub struct Browser {
    state: State,
}

impl Browser {
    pub fn new() -> Self {
        Browser { state: State::new() }
    }
}

impl App for Browser {
    fn manifest(&self) -> AppManifest {
        manifest()
    }
    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        on_event(&mut self.state, event)
    }
    fn paint(&mut self, fb: &mut PaintBuffer) {
        // The window can be resized after the page laid out; when the surface
        // width changes, reflow the document to the new width so it fills the
        // window instead of leaving a fixed-width column.
        if fb.width != 0 && fb.width != self.state.viewport_w {
            self.state.viewport_w = fb.width;
            crate::browser::event::relayout(&mut self.state);
        }
        paint(&self.state, fb);
    }
    fn on_tick(&mut self) -> bool {
        // A pending navigation preempts everything: drop the old page's page
        // and image fetches and load the new document before any pool work, so
        // stale image URLs are not fetched against the outgoing page.
        if let Some(target) = self.state.pending_nav.take() {
            if let Some(job) = self.state.fetch.take() {
                let _ = crate::browser::net::socket_close(self.state.sockets_port, job.handle);
            }
            for job in self.state.img_fetches.drain(..) {
                let _ = crate::browser::net::socket_close(self.state.sockets_port, job.handle);
            }
            if let Err(msg) = crate::browser::fetch::load(&mut self.state, &target) {
                self.state.status = alloc::string::String::from(msg);
                self.state.document = Some(crate::browser::fetch::render_error(msg));
                self.state.box_doc = None;
                self.state.page_dom = None;
                self.state.world = None;
                self.state.view = crate::browser::state::View::Page;
            }
            return true;
        }
        // The image pool runs on its own sockets, so advance it and refill free
        // slots every tick, in parallel with the page fetch rather than behind
        // it. This is what lets an image-heavy page fill in quickly.
        let mut busy = crate::browser::fetch::step_images(&mut self.state);
        busy |= crate::browser::image::pump(&mut self.state);
        // The page/css/js fetch keeps its own single socket.
        if self.state.fetch.is_some() {
            return crate::browser::fetch::step(&mut self.state) || busy;
        }
        if crate::browser::event::js_tick(&mut self.state) {
            return true;
        }
        if crate::browser::fetch::css_pump(&mut self.state) {
            return true;
        }
        if crate::browser::fetch::js_pump(&mut self.state) {
            return true;
        }
        busy
    }
    fn tick_interval_ms(&self) -> i64 {
        50
    }
}
