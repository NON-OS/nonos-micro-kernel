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

use std::path::Path;

use super::tamper::tamper;
use super::verify_capsule::verify_capsule;

pub fn verify_tampered(verifier: &Path, vk: &Path, src: &Path, dst: &Path) -> Result<(), String> {
    tamper(src, dst)?;
    println!("\n  $ verify-proof --capsule term_bad.cap");
    let (ok, text) = verify_capsule(verifier, vk, dst)?;
    print!("{text}");
    if ok {
        return Err("tampered capsule was accepted".into());
    }
    println!("\n  tamper rejected by capsule hash binding\n");
    Ok(())
}
