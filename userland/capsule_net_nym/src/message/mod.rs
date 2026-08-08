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

//! The message layer that sits between a request and a Sphinx packet.
//!
//! A Sphinx packet does not carry an application request directly. It carries
//! a fragment of a padded message which names its own type, says who may be
//! replied to, and hands over the routes a reply can take. Skipping this layer
//! puts bytes on the wire that reach an exit and mean nothing to it.

mod fragment;
mod fragment_parse;
mod plaintext;
mod prepare;
mod repliable;
mod types;

pub use fragment::{Fragment, MAX_FRAGMENTS, UNLINKED_HEADER_LEN};
pub use fragment_parse::parse;
pub use plaintext::{FRAGMENT_PER_PACKET, PLAINTEXT_PER_PACKET};
pub use prepare::{prepare, prepare_built, Prepared};
pub use repliable::{pad_to_packets, repliable_additional_surbs, repliable_data, unpad};
pub use types::{SENDER_TAG_SIZE, TAG_ADDITIONAL_SURBS, TAG_DATA, TYPE_REPLIABLE};
