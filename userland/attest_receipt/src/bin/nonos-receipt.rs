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

//! Check a receipt from a NØNOS machine. Exits 0 when it verified, 1 when it
//! did not, 2 on bad usage. The forms are in `cli`.

use std::path::Path;
use std::process::ExitCode;

use attest_receipt::{parse_args, parse_entries, render, verify_root, Mode};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = match parse_args(&args) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(2);
        }
    };
    match mode {
        Mode::Root { entries, root } => {
            let Some(bytes) = read(&entries) else { return ExitCode::from(2) };
            if let Err(m) = verify_root(&bytes, root) {
                return refused(&m.describe());
            }
            verified(&bytes, "against the signed root")
        }
    }
}

fn read(path: &Path) -> Option<Vec<u8>> {
    std::fs::read(path).map_err(|e| eprintln!("nonos-receipt: {}: {e}", path.display())).ok()
}

fn refused(why: &str) -> ExitCode {
    eprintln!("REFUSED: {why}");
    ExitCode::from(1)
}

fn verified(bytes: &[u8], how: &str) -> ExitCode {
    match parse_entries(bytes) {
        Err(e) => refused(&e.describe()),
        Ok(entries) => {
            println!("VERIFIED {how}\n");
            print!("{}", render(&entries));
            ExitCode::SUCCESS
        }
    }
}
