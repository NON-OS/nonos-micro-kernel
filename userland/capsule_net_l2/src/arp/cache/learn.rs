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

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Learn {
    Insert,
    Refresh,
    Reject,
}

pub fn decide(existing_mac: Option<[u8; 6]>, sender_mac: [u8; 6], solicited: bool) -> Learn {
    match existing_mac {
        Some(m) if m == sender_mac => Learn::Refresh,
        Some(_) => Learn::Reject,
        None if solicited => Learn::Insert,
        None => Learn::Reject,
    }
}
