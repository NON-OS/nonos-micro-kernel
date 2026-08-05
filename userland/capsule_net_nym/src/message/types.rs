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

//! Constants the message layer is defined in terms of.

/// A message that carries a sender tag and reply surbs, so the far end can
/// answer without ever learning who asked.
pub const TYPE_REPLIABLE: u8 = 1;

/// Content tag for a request with data attached, as opposed to one that only
/// tops up the far end's supply of reply surbs.
pub const TAG_DATA: u8 = 0;

/// Content tag for a message carrying nothing but reply blocks, sent when the
/// far end has asked for more.
pub const TAG_ADDITIONAL_SURBS: u8 = 1;

/// Bytes of the tag a far end quotes to reach us again.
pub const SENDER_TAG_SIZE: usize = 16;
