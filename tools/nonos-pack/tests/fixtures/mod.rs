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

mod exec;
mod load;
mod mutate;
mod paths;

pub use exec::{capsule_sign, pack_gui_demo, trust_policy, unpack};
pub use load::{first_elf_byte_offset, gui_demo_container_and_seeds, seal_ed25519_only};
pub use mutate::{append_trailer_entry, corrupt_signature};
pub use paths::gui_demo_paths;
