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

// Real raster images for the document view. Sources discovered during layout
// are fetched through the same socket state machine as pages, decoded from
// PNG/JPEG/BMP into ARGB8888, cached, and scale-blitted into their box.

mod base64;
mod blit;
mod data_uri;
mod decode;
#[cfg(not(feature = "harness"))]
mod fetch;
mod ingest;
#[cfg(not(feature = "harness"))]
mod queue;
mod sniff;
mod store;
mod svg;
mod webp;

pub use blit::blit_into;
pub(crate) use data_uri::data_uri_bytes;
#[cfg(not(feature = "harness"))]
pub use fetch::{follow_redirect, pump};
pub use ingest::ingest;
#[cfg(not(feature = "harness"))]
pub use queue::enqueue_from_doc;
pub use store::{Decoded, Store};

// Record the box a source will be drawn into before ingest, so a vector image
// rasterizes at its display size instead of upscaling a default raster. The
// on-device path does this through the fetch queue; the host render harness has
// no queue and calls this directly.
#[cfg(feature = "harness")]
pub fn note_size(store: &mut Store, url: &str, w: u32, h: u32) {
    store.mark_pending(url);
    store.note_hint(url, w, h);
}
