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

pub fn impl_name(id: usize) -> &'static str {
    match id {
        0 => "Berkeley Boot Loader (BBL)",
        1 => "OpenSBI",
        2 => "Xvisor",
        3 => "KVM",
        4 => "RustSBI",
        5 => "Diosix",
        6 => "Coffer",
        _ => "Unknown",
    }
}
