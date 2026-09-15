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

// The app_skeleton sidecar codec, included verbatim so the blob format the
// file manager persists is proven here rather than re-implemented. The crate
// root aliases itself to `nonos_app_skeleton` so capsule source that imports
// `nonos_app_skeleton::sidecar` resolves to this module.

#[path = "../../../app_skeleton/src/sidecar/decode.rs"]
mod decode;
#[path = "../../../app_skeleton/src/sidecar/encode.rs"]
mod encode;

pub use decode::decode;
pub use encode::{encode, SIDECAR_MAGIC, SIDECAR_VERSION};
