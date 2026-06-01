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

pub fn run() -> Result<(), String> {
    println!("nonos-capsule — Cargo-to-Capsule");
    println!("  new <name>                      scaffold a capsule project");
    println!("  build [dir]                     build the payload ELF");
    println!("  manifest [dir]                  resolve and print the manifest");
    println!("  sign [dir]                      sign into dir/dist (Ed25519 + ML-DSA)");
    println!("  install <pkg> --cert C --policy P  verify and install to the store");
    println!("  run <name> --cert C --policy P  verify an installed capsule");
    println!("  inspect <name>                  show installed capsule metadata");
    println!("  remove <name>                   remove an installed capsule");
    Ok(())
}
