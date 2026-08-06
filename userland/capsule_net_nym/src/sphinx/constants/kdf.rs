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

/// Archimedes, transliterated. Fixed by the network: it is HKDF `info`, so a
/// single character different yields different keys at every hop.
pub const EXPANDED_SHARED_SECRET_HKDF_INFO: &[u8] =
    b"Dwste mou enan moxlo arketa makru kai ena upomoxlio gia na ton topothetisw kai tha kinisw thn gh.";

pub const EXPANDED_SHARED_SECRET_HKDF_SALT: &[u8] = b"";
pub const PAYLOAD_KEY_HKDF_INFO: &[u8] = b"sphinx-payload-key-V01-CS01-HKDF:SHA256-INFO";
pub const PAYLOAD_KEY_HKDF_SALT: &[u8] = b"sphinx-payload-key-V01-CS01-HKDF:SHA256-SALT";
