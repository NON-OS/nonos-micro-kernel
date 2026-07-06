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

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone)]
pub struct ProxyConfig {
    pub host: String,
    pub port: u16,
}

pub enum View {
    Home,
    Page,
}

pub struct State {
    pub address: String,
    pub address_focused: bool,
    pub status: String,
    pub pending_nav: Option<String>,
    pub document: Option<crate::browser::layout::doc::RenderDocument>,
    pub box_doc: Option<crate::browser::layout::boxmodel::BoxDocument>,
    pub page_dom: Option<crate::browser::dom::Dom>,
    pub world: Option<crate::browser::js::World>,
    pub focus: Option<usize>,
    pub pending_post: Option<String>,
    pub scroll: u32,
    pub sockets_port: u32,
    pub view: View,
    pub fetch: Option<crate::browser::fetch::types::Fetch>,
    pub base: Option<crate::browser::url::Url>,
    pub redirect_count: u8,
    pub history: Vec<String>,
    pub hist_index: i32,
    pub suppress_history_push: bool,
    pub retries: u8,
    pub proxy: Option<ProxyConfig>,
    pub images: crate::browser::image::Store,
    pub image_queue: Vec<String>,
    // In-flight image fetches, run concurrently on their own sockets so an
    // image-heavy page fills in parallel instead of one raster at a time.
    pub img_fetches: Vec<crate::browser::fetch::types::Fetch>,
    // Current content width in pixels, tracked from the paint surface so the
    // page reflows when the window resizes instead of holding a fixed width.
    pub viewport_w: u32,
    // External stylesheets: URLs still to fetch, and the CSS text gathered so
    // far. Applied on top of the page's inline <style> at each re-layout.
    pub css_queue: Vec<String>,
    pub page_css: String,
    // Author rules parsed once and reused across relayouts when the CSS text
    // is unchanged, so JS-driven relayouts skip re-parsing the whole sheet.
    pub css_cache: Option<crate::browser::css::CssCache>,
}

impl State {
    pub fn new() -> Self {
        State {
            address: String::new(),
            address_focused: true,
            status: String::from("ready"),
            pending_nav: None,
            document: None,
            box_doc: None,
            page_dom: None,
            world: None,
            focus: None,
            pending_post: None,
            scroll: 0,
            sockets_port: 0,
            view: View::Home,
            fetch: None,
            base: None,
            redirect_count: 0,
            history: Vec::new(),
            hist_index: -1,
            suppress_history_push: false,
            retries: 0,
            proxy: None,
            images: crate::browser::image::Store::new(),
            image_queue: Vec::new(),
            img_fetches: Vec::new(),
            viewport_w: crate::browser::manifest::WIDTH,
            css_queue: Vec::new(),
            page_css: String::new(),
            css_cache: None,
        }
    }
}
