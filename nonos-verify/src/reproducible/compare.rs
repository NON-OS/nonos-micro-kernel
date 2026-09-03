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

//! What "the same build" means per artifact. A plain artifact must be
//! byte identical. A NONOSIMG container holds the measured kernel
//! payload, then an ML-DSA hybrid signature, then a STARK proof; both
//! crypto blocks are freshly randomized on every signing run, so two
//! honest builds can never agree on their bytes. The payload must be
//! identical and the crypto blocks must be present on both sides.

use serde_json::{json, Value};
use std::path::Path;

pub(super) enum Kind {
    Bytes,
    Container,
}

pub(super) fn artifacts() -> Vec<(String, Kind)> {
    if let Ok(list) = std::env::var("NONOS_REPRO_ARTIFACTS") {
        return list.split_whitespace().map(|p| (p.to_string(), Kind::Bytes)).collect();
    }
    [
        ("target/x86_64-nonos/release/nonos-kernel", Kind::Bytes),
        ("target/kernel_signed.bin", Kind::Container),
        ("target/kernel_attested.bin", Kind::Container),
        ("nonos-bootloader/target/x86_64-unknown-uefi/release/nonos_boot.efi", Kind::Bytes),
        ("target/esp/EFI/Boot/BOOTX64.EFI", Kind::Bytes),
        ("target/esp/EFI/nonos/kernel.bin", Kind::Container),
        ("target/esp/EFI/nonos/boot.cfg", Kind::Bytes),
        ("target/esp/startup.nsh", Kind::Bytes),
    ]
    .map(|(p, k)| (p.to_string(), k))
    .into_iter()
    .collect()
}

pub(super) fn measure(a: &Path, b: &Path, set: Vec<(String, Kind)>) -> (Vec<Value>, bool) {
    let mut rows = Vec::new();
    let mut all_ok = true;
    for (rel, kind) in set {
        let row = match kind {
            Kind::Bytes => bytes_row(a, b, &rel),
            Kind::Container => container_row(a, b, &rel),
        };
        all_ok &= row["ok"].as_bool() == Some(true);
        rows.push(row);
    }
    (rows, all_ok)
}

fn bytes_row(a: &Path, b: &Path, rel: &str) -> Value {
    match (std::fs::read(a.join(rel)), std::fs::read(b.join(rel))) {
        (Ok(da), Ok(db)) => {
            let (ha, hb) = (hex(&da), hex(&db));
            json!({"path": rel, "class": "bytes", "ok": ha == hb, "a": ha, "b": hb})
        }
        _ => json!({"path": rel, "class": "bytes", "ok": false, "a": "MISSING", "b": "MISSING"}),
    }
}

fn container_row(a: &Path, b: &Path, rel: &str) -> Value {
    let (pa, pb) = (parse(&a.join(rel)), parse(&b.join(rel)));
    match (pa, pb) {
        (Some(ca), Some(cb)) => {
            let ok = ca.payload == cb.payload && ca.crypto_present && cb.crypto_present;
            json!({"path": rel, "class": "container", "ok": ok,
                   "a": ca.payload, "b": cb.payload,
                   "crypto_present": [ca.crypto_present, cb.crypto_present]})
        }
        _ => json!({"path": rel, "class": "container", "ok": false,
                    "a": "MALFORMED", "b": "MALFORMED"}),
    }
}

struct Parsed {
    payload: String,
    crypto_present: bool,
}

// NONOSIMG trailer, 64 bytes at the end of the file: magic, version,
// flags, then the kernel/signature/proof extents. Layout is
// create_image_footer in nonos-bootloader/tools/embed-zk-proof.
fn parse(path: &Path) -> Option<Parsed> {
    let data = std::fs::read(path).ok()?;
    let foot = data.len().checked_sub(64)?;
    let f = &data[foot..];
    if &f[0..8] != b"NONOSIMG" {
        return None;
    }
    let flags = u16::from_le_bytes([f[10], f[11]]);
    let kernel_size = u32::from_le_bytes([f[28], f[29], f[30], f[31]]) as usize;
    let sig_size = u32::from_le_bytes([f[36], f[37], f[38], f[39]]) as usize;
    let proof_size = u32::from_le_bytes([f[44], f[45], f[46], f[47]]) as usize;
    if kernel_size + sig_size + proof_size > foot {
        return None;
    }
    let needs_proof = flags & 1 != 0;
    Some(Parsed {
        payload: hex(&data[..kernel_size]),
        crypto_present: sig_size > 0 && (!needs_proof || proof_size > 0),
    })
}

fn hex(data: &[u8]) -> String {
    blake3::hash(data).to_hex().to_string()
}
