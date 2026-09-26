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

//! The one list of capabilities, and what is generated from it.

macro_rules! capability_table {
    ($($(#[$meta:meta])* $name:ident = $bit:expr,)+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Capability {
            $($(#[$meta])* $name,)+
        }

        impl Capability {
            /// Where this capability sits in a `caps_bits` mask.
            pub(crate) const fn bit(self) -> u64 {
                match self {
                    $(Self::$name => $bit,)+
                }
            }

            /// Every capability there is, in bit order.
            pub const fn all() -> &'static [Capability] {
                &[$(Self::$name,)+]
            }

            /// Derived from the list, so it cannot report a count the
            /// list does not have. It once reported 23 against 27.
            pub const fn count() -> usize {
                Self::all().len()
            }
        }
    };
}

pub(super) use capability_table;
