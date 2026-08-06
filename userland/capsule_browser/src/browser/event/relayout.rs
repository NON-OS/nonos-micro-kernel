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

use crate::browser::state::State;
use crate::browser::{css, layout};

// Rebuild styles, box tree and display list after a script mutation, an
// external stylesheet arriving, or an image landing. Author CSS (fetched
// stylesheets) cascades after the page's inline <style>.
pub fn relayout(state: &mut State) {
    let Some(dom) = state.page_dom.as_ref() else {
        return;
    };
    let mut css_text = css::collect_css(dom);
    css_text.push_str(&state.page_css);
    let styled = css::compute_cached(dom, &css_text, &mut state.css_cache);
    let root = layout::boxmodel::build(
        dom,
        &styled.styles,
        &styled.bg_images,
        &styled.grids,
        &styled.pseudos,
    );
    let doc = layout::boxmodel::layout(&root, state.viewport_w);
    // The rectangles just produced are what a script gets when it measures an
    // element. Recording them here means a read after a layout sees the
    // layout that happened rather than the one before it.
    if let Some(dom) = state.page_dom.as_mut() {
        dom.record_rects(doc.frags.iter().map(|f| (f.node, f.x, f.y, f.w, f.h)));
    }
    state.box_doc = Some(doc);
    // Queue newly declared web fonts; each face is fetched once and text
    // relayouts with its real metrics when it lands. A data: source, the way
    // icon fonts ship, carries its bytes inline and installs on the spot.
    for (key, src) in crate::browser::fonts::collect_font_faces(&css_text) {
        if state.font_seen.contains(&key) {
            continue;
        }
        state.font_seen.push(key);
        if src.starts_with("data:") {
            if let Some(bytes) = crate::browser::image::data_uri_bytes(&src) {
                let _ = crate::browser::fonts::ingest_font(key, bytes);
            }
        } else if let Some(base) = state.base.as_ref() {
            state.font_queue.push((key, crate::browser::url::join(base, &src)));
        }
    }
    crate::browser::image::enqueue_from_doc(state);
}
