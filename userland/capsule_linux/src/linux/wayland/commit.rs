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


//! `wl_surface.commit`: the only request that reaches the screen.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::ops::ev;
use super::out::Event;
use super::present::present;

/// A commit with a buffer is the only thing that reaches the screen.
pub fn commit(guest: &mut Guest, id: u32) {
    let Some(buffer) = surface_buffer(guest, id) else {
        return;
    };
    present(guest, buffer);
    Event::new(buffer, ev::BUFFER_RELEASE).send(&mut guest.display.to_client);
    let owed = take_frames(guest, id);
    for callback in owed {
        Event::new(callback, ev::CALLBACK_DONE).u32(0).send(&mut guest.display.to_client);
    }
}

fn surface_buffer(guest: &Guest, id: u32) -> Option<u32> {
    guest.scene.surfaces.iter().find(|s| s.id == id)?.pending
}

fn take_frames(guest: &mut Guest, id: u32) -> Vec<u32> {
    match guest.scene.surfaces.iter_mut().find(|s| s.id == id) {
        Some(s) => core::mem::take(&mut s.frames),
        None => Vec::new(),
    }
}
