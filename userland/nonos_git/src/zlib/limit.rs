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
//! How far a stream is allowed to expand.

/// Ceiling on what one object may inflate to.
///
/// Deflate expands by roughly a thousand to one at its worst, so a few
/// kilobytes of hostile input can ask for gigabytes. Nothing git stores comes
/// near this: the largest object in a clone of this kernel is under 34 MB.
pub const MAX_INFLATED: usize = 256 * 1024 * 1024;
