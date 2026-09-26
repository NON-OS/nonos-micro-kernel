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


//! Copying a parent's spans into the child it just made.

use crate::linux::guest::{Guest, Region};
use nonos_libc::peer::{mk_peer_map, mk_peer_write, PEER_PROT_EXEC, PEER_PROT_WRITE};

/// Every span, mapped into the child and then filled from the parent.
pub(super) fn copy_spans(guest: &mut Guest, child: u32) -> bool {
    let spans = guest.regions.clone();
    for span in spans {
        if mk_peer_map(child, span.at, span.len, prot_of(&span)) < 0 {
            return false;
        }
        if !copy_one(guest, child, span.at, span.len) {
            return false;
        }
    }
    true
}

fn prot_of(span: &Region) -> u64 {
    let mut prot = 0;
    if span.write {
        prot |= PEER_PROT_WRITE;
    }
    if span.exec {
        prot |= PEER_PROT_EXEC;
    }
    prot
}

fn copy_one(guest: &Guest, child: u32, at: u64, len: u64) -> bool {
    let Some(bytes) = guest.read(at, len as usize) else {
        return false;
    };
    mk_peer_write(child, at, &bytes) >= 0
}
