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

//! The editor as a small IDE shell: an activity bar and file-explorer sidebar,
//! a strip of document tabs, the code pane, and a status bar. It owns several
//! open documents and the file tree; input and painting are delegated to the
//! `ws_event` and `ws_paint` modules.

use alloc::vec;
use alloc::vec::Vec;

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer};

use super::manifest::manifest;
use super::menubar::TitleSpan;
use super::ribbon::RibbonCell;
use super::sb_entry::SbEntry;
use super::sb_menu::SbMenu;
use super::state::State;
use super::tabbar::TabSpan;
use super::tree::FileTree;

pub struct Editor {
    pub(super) docs: Vec<State>,
    pub(super) active: usize,
    pub(super) tree: FileTree,
    pub(super) sidebar_open: bool,
    pub(super) owner_pid: u32,
    // Tab pixel spans from the last paint, used to hit-test tab-strip clicks.
    pub(super) tab_layout: Vec<TabSpan>,
    // Window size from the last paint; input events do not carry it, so the
    // event router reuses these to reconstruct panel rectangles.
    pub(super) last_w: u32,
    pub(super) last_h: u32,
    // Explorer file management: the open context menu and the inline name
    // entry for create/rename, at most one of each at a time.
    pub(super) menu: Option<SbMenu>,
    pub(super) mb_open: Option<usize>,
    pub(super) mb_layout: Vec<TitleSpan>,
    // Formatting ribbon: the pill whose dropdown is open, and the control cells
    // from the last paint, whose widths track the labels the caret dictates.
    pub(super) rb_open: Option<usize>,
    pub(super) rb_layout: Vec<RibbonCell>,
    pub(super) entry: Option<SbEntry>,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            docs: vec![State::new()],
            active: 0,
            tree: FileTree::new(),
            sidebar_open: true,
            owner_pid: 0,
            tab_layout: Vec::new(),
            last_w: 0,
            last_h: 0,
            menu: None,
            mb_open: None,
            mb_layout: Vec::new(),
            rb_open: None,
            rb_layout: Vec::new(),
            entry: None,
        }
    }

    pub(super) fn doc(&mut self) -> &mut State {
        let i = self.active.min(self.docs.len().saturating_sub(1));
        &mut self.docs[i]
    }
}

impl App for Editor {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        self.handle_event(event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        self.paint_shell(fb);
    }
}
