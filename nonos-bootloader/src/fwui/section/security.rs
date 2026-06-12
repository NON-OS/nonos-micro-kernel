// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::flag::flag;
use super::info::info;
use super::row::Row;
use crate::fwui::data::Sys;
use crate::fwui::theme;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub fn security(sys: &Sys) -> Vec<Row> {
    let (sb, sbc) = flag(sys.secure_boot, "ENABLED", "DISABLED", false);
    let (mb, mbc) = flag(sys.measured_boot, "ACTIVE", "INACTIVE", true);
    let (pk, pkc) = flag(sys.platform_key, "VERIFIED", "ABSENT", true);
    let (db, dbc) = flag(sys.sig_db, "VALID", "ABSENT", true);
    let (rng, rngc) = flag(sys.rng, "AVAILABLE", "ABSENT", false);
    let (ed, edc) = flag(sys.ed25519, "PASS", "FAIL", false);
    let (bl, blc) = flag(sys.blake3, "PASS", "FAIL", false);
    vec![
        info(b"SECURE BOOT", sb, sbc, b"UEFI Secure Boot state from the SecureBoot variable."),
        info(b"MEASURED BOOT", mb, mbc, b"TPM 2.0 PCR measurement availability."),
        info(b"PLATFORM KEY", pk, pkc, b"Presence of the UEFI Platform Key."),
        info(b"SIGNATURE DB", db, dbc, b"Validity of the loaded signature database."),
        info(b"HARDWARE RNG", rng, rngc, b"RDRAND/RDSEED entropy source availability."),
        info(b"ED25519 SELFTEST", ed, edc, b"Signature primitive power-on self test."),
        info(b"BLAKE3 SELFTEST", bl, blc, b"Hash primitive power-on self test."),
        info(
            b"PRODUCTION KEYS",
            format!("{}", sys.keys),
            theme::TEXT,
            b"Embedded production verification key count.",
        ),
    ]
}
