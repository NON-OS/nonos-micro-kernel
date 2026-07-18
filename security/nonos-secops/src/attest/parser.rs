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

//! The parser-robustness oracle the fuzzer drives.

use nonos_stark::air::deserialize_proof_ext;

/// Feed a byte string to the untrusted money-grade proof deserializer and report
/// whether it completed rather than panicking. The gate parses adversarial input
/// before it verifies, so a parser that can be driven to panic is a boot-time
/// denial of service. This wraps the exact deserializer the bootloader links.
pub fn proof_parser_is_total(bytes: &[u8]) -> bool {
    std::panic::catch_unwind(|| {
        let _ = deserialize_proof_ext(bytes);
    })
    .is_ok()
}
