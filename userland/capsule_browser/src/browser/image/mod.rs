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
mod fetch;
mod ingest;
mod queue;
mod sniff;
mod store;
mod svg;
mod webp;

pub use blit::blit_into;
pub use fetch::{follow_redirect, pump};
pub use ingest::ingest;
pub use queue::enqueue_from_doc;
pub use store::{Decoded, Store};
