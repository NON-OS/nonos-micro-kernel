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
    // The QuickJS engine that ran this page's scripts. It keeps the page's
    // listeners and closure state alive so later UI events dispatch into it. The
    // engine holds a pointer into `page_dom`, which keeps its address for the
    // page's life, so a navigation drops the engine before replacing the DOM.
    pub engine: Option<nonos_qjs::Engine>,
    // Whether the settings panel behind the menu button is open.
    pub settings_open: bool,
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
    // Hops taken by the in-flight image fetch; bounds 3xx chasing.
    pub image_redirects: u8,
    // Alternates the free socket between script-issued fetches and images so a
    // page whose JS never stops requesting cannot starve image loading.
    pub img_turn: bool,
    // A TLS connection held open between image fetches, so a run of same-host
    // images pays a single handshake.
    pub keep: Option<crate::browser::fetch::KeptConn>,
    // Declared @font-face sources still to fetch, and the keys ever queued so
    // a face is fetched at most once per page.
    pub font_queue: Vec<(u32, String)>,
    pub font_seen: Vec<u32>,
    // Current content width in pixels, tracked from the paint surface so the
    // page reflows when the window resizes instead of holding a fixed width.
    pub viewport_w: u32,
    // External stylesheets: URLs still to fetch, and the CSS text gathered so
    // far. Applied on top of the page's inline <style> at each re-layout.
    pub css_queue: Vec<String>,
    pub page_css: String,
    // External <script src> bundles still to fetch. Each is evaluated in the
    // page engine as it arrives, in document order, so framework bundles run.
    pub script_queue: Vec<String>,
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
            engine: None,
            settings_open: false,
            script_queue: Vec::new(),
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
            image_redirects: 0,
            img_turn: false,
            keep: None,
            font_queue: Vec::new(),
            font_seen: Vec::new(),
            viewport_w: crate::browser::manifest::WIDTH,
            css_queue: Vec::new(),
            page_css: String::new(),
            css_cache: None,
        }
    }
}
