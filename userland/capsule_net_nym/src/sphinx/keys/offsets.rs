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

use super::super::constants::{INTEGRITY_MAC_KEY_SIZE, PAYLOAD_KEY_SIZE, STREAM_CIPHER_KEY_SIZE};

pub(super) const STREAM_CIPHER_KEY_AT: usize = 0;
pub(super) const INTEGRITY_MAC_KEY_AT: usize = STREAM_CIPHER_KEY_AT + STREAM_CIPHER_KEY_SIZE;
pub(super) const PAYLOAD_KEY_AT: usize = INTEGRITY_MAC_KEY_AT + INTEGRITY_MAC_KEY_SIZE;
pub(super) const BLINDING_FACTOR_AT: usize = PAYLOAD_KEY_AT + PAYLOAD_KEY_SIZE;
