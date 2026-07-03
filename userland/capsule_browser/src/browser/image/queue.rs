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

use crate::browser::layout::boxmodel::Content;
use crate::browser::state::State;
use crate::browser::url;

// Collect every image source in the current page, resolve it against the
// page base, and queue the ones not already known for fetching.
pub fn enqueue_from_doc(state: &mut State) {
    let Some(base) = state.base.clone() else { return };
    let mut fresh: Vec<String> = Vec::new();
    if let Some(doc) = state.box_doc.as_ref() {
        for f in &doc.frags {
            if let Content::Image { src, .. } = &f.content {
                let abs = url::join(&base, src);
                if !state.images.contains(&abs) && !fresh.contains(&abs) {
                    fresh.push(abs);
                }
            }
        }
    }
    for u in fresh {
        state.images.mark_pending(&u);
        state.image_queue.push(u);
    }
}
