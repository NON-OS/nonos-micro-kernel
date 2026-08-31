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

use core::sync::atomic::{AtomicU32, Ordering};

use crate::snake::state::Screen;

use super::target::{self, Target};

// The one thing a painter needs from the pointer. Widgets ask `is` with the
// same tag and index the geom module gave their rect, so the lit shape is
// always the shape the hit test would have answered for.
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    None,
    HomeAction,
    HomeCard,
    ModeChip,
    DiffChip,
    Toggle,
    Start,
    Foot,
    PauseAction,
    OverAction,
    RankBack,
}

static TAG: AtomicU32 = AtomicU32::new(Tag::None as u32);
static INDEX: AtomicU32 = AtomicU32::new(0);

pub fn is(tag: Tag, index: usize) -> bool {
    TAG.load(Ordering::Relaxed) == tag as u32 && INDEX.load(Ordering::Relaxed) == index as u32
}

pub fn update(screen: Screen, x: i32, y: i32) -> bool {
    let (tag, index) = target::at(screen, x, y).map_or((Tag::None, 0), of);
    let moved = !is(tag, index);
    TAG.store(tag as u32, Ordering::Relaxed);
    INDEX.store(index as u32, Ordering::Relaxed);
    moved
}

fn of(target: Target) -> (Tag, usize) {
    match target {
        Target::HomeAction(i) => (Tag::HomeAction, i),
        Target::HomeCard(i) => (Tag::HomeCard, i),
        Target::Chip(0, i) => (Tag::ModeChip, i),
        Target::Chip(_, i) => (Tag::DiffChip, i),
        Target::Toggle(i) => (Tag::Toggle, i),
        Target::Start => (Tag::Start, 0),
        Target::Foot(i) => (Tag::Foot, i),
        Target::PauseAction(i) => (Tag::PauseAction, i),
        Target::OverAction(i) => (Tag::OverAction, i),
        Target::RankBack => (Tag::RankBack, 0),
    }
}
