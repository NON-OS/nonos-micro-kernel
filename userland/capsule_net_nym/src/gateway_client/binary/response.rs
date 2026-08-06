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

/// A message the gateway pushed to us from the mixnet.
pub const KIND_PUSHED_MIX_MESSAGE: u8 = 1;

/// Unknown kinds are ignored rather than guessed at: the kind byte is outside
/// the authenticated region.
pub fn is_pushed_message(kind: u8) -> bool {
    kind == KIND_PUSHED_MIX_MESSAGE
}
