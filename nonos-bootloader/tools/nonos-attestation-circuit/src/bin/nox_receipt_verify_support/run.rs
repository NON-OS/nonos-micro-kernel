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

use std::path::Path;

use nonos_attestation_circuit::nox::{check_receipt, hex32};

use super::args::parse;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse();
    let receipt: serde_json::Value = serde_json::from_slice(&std::fs::read(&args.receipt)?)?;
    let verifier = check_receipt(
        &receipt,
        Path::new(&args.verifying_key),
        Path::new(&args.transcript),
        args.artifact.as_deref().map(Path::new),
    )?;
    println!("{}", hex32(&verifier));
    Ok(())
}
