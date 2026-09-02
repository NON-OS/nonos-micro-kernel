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

/// A rail figure that may have no value to show. `Unknown` is a live source
/// with nothing in it right now, such as an interface holding no lease;
/// `Unsupported` is a standing fact about NONOS, which publishes no load
/// average, swap, IPv6, byte counters or mount capacity. Both must reach the
/// painter as an em-dash, never as a zero.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Metric<T> {
    Known(T),
    Unknown,
    Unsupported,
}

impl<T: Copy> Metric<T> {
    pub fn value(self) -> Option<T> {
        match self {
            Metric::Known(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_known(self) -> bool {
        matches!(self, Metric::Known(_))
    }

    pub fn is_unsupported(self) -> bool {
        matches!(self, Metric::Unsupported)
    }
}
