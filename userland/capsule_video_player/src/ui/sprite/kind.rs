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

//! Identifier for every cached icon mask. The order matches `table::BUILDERS`.

#[derive(Clone, Copy)]
pub enum Glyph {
    Play,
    Pause,
    Prev,
    Next,
    Rewind,
    Repeat,
    Shuffle,
    Volume,
    Mute,
    Home,
    Library,
    Playlist,
    Files,
    Gear,
    Video,
    Disc,
    Back,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    Plus,
    Close,
    Check,
    Dots,
    Cc,
    Clock,
    Fullscreen,
    Pip,
    Search,
    List,
    Info,
}
