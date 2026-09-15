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

use alloc::{string::String, vec::Vec};

use super::layout::SIDE_ROW_H;
use super::screen::Screen;

// A section label is shorter than a navigable row. Groups are parted either by
// a bare gap or by a hairline that claims a band of its own.
pub const LABEL_H: u32 = 26;
pub const SEC_GAP: u32 = 14;
pub const RULE_H: u32 = 19;

// Downloads is a directory rather than a surface, so its row navigates Browse.
pub const DOWNLOADS: &str = "/downloads/";

/// Where a sidebar row navigates: a top-level surface, or a directory that the
/// Browse surface opens.
#[derive(Clone)]
pub enum SideHit {
    Screen(Screen),
    Path(String),
}

/// What a laid-out line is, so the painter can pick its mark from the same list
/// the hit-test reads rather than inferring one from an empty label.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SideKind {
    Label,
    Rule,
    Nav,
    Storage,
}

/// One laid-out sidebar line. A section label and a rule carry no `hit` and are
/// inert; `h` is the band the line owns, which a click is tested against.
pub struct SideRow {
    pub y: u32,
    pub h: u32,
    pub label: String,
    pub kind: SideKind,
    pub hit: Option<SideHit>,
}

/// The fixed navigation block, in drawn order.
pub const NAV: [(&str, Screen); 4] = [
    ("Home", Screen::Home),
    ("Recents", Screen::Recents),
    ("Shared", Screen::Shared),
    ("Tags", Screen::Tags),
];

/// Accumulates rows against a running y, so a section only ever states its own
/// contents and never its offset.
pub struct Rows {
    pub rows: Vec<SideRow>,
    pub y: u32,
}

impl Rows {
    pub fn label(&mut self, text: &str) {
        self.push(LABEL_H, String::from(text), SideKind::Label, None);
    }

    /// The hairline between two groups. It owns its band so the rule is drawn at
    /// the middle of a stated height rather than floating in a bare gap.
    pub fn rule(&mut self) {
        self.push(RULE_H, String::new(), SideKind::Rule, None);
    }

    pub fn row(&mut self, label: String, hit: SideHit) {
        self.push(SIDE_ROW_H, label, SideKind::Nav, Some(hit));
    }

    /// Places a line at an explicit `y` without advancing the running cursor,
    /// for the card pinned to the foot instead of stacked with the groups.
    pub fn pinned(&mut self, y: u32, h: u32, kind: SideKind, hit: Option<SideHit>) {
        self.rows.push(SideRow { y, h, label: String::new(), kind, hit });
    }

    fn push(&mut self, h: u32, label: String, kind: SideKind, hit: Option<SideHit>) {
        self.rows.push(SideRow { y: self.y, h, label, kind, hit });
        self.y += h;
    }
}
