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

// On-screen device evidence for headless bring-up. When the kernel is built
// with NONOS_DEVICE_CENSUS=1 it renders the enumerated broker table to the
// framebuffer and holds, so a machine with no serial and no working input can
// still report which input controllers its firmware exposed.

mod buf;
mod label;
mod line;
mod lpss;
mod render;
mod verdict;

pub use render::render_and_hold;
