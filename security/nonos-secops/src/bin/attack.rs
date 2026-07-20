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

//! nonos-attack: the red-team harness for the NONOS attestation. It builds
//! genuine attested images and mounts the attacks a shipped image must survive,
//! running each against the exact verification the bootloader does before jump.
//! An attack passes only when the attestation refuses it, so a run is evidence.
//!
//!   nonos-attack battery [--json]   the categorized attack battery (default)
//!   nonos-attack fuzz [iterations]  fuzz the untrusted trailer parser
//!
//! Exit 0 means every attack was refused and the parser stayed total. Any other
//! exit is a real finding: the gate let something through, or the parser broke.

use nonos_secops::{battery, fuzz};
use std::{env, process::exit};

fn run_battery(as_json: bool) -> bool {
    let findings = battery();
    let held = findings.iter().all(|f| f.refused);
    if as_json {
        let body: Vec<String> = findings.iter().map(|f| f.json()).collect();
        println!("{{\"tool\":\"nonos-attack\",\"held\":{held},\"findings\":[{}]}}", body.join(","));
    } else {
        println!("NONOS attestation red-team battery\n");
        for f in &findings {
            f.print();
        }
        println!();
        if held {
            println!("all {} attacks refused; the attestation held", findings.len());
        } else {
            let bypassed: Vec<&str> =
                findings.iter().filter(|f| !f.refused).map(|f| f.id).collect();
            eprintln!("attestation did NOT hold; the gate let through: {}", bypassed.join(", "));
        }
    }
    held
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str).unwrap_or("battery") {
        "battery" => exit(if run_battery(args.iter().any(|a| a == "--json")) { 0 } else { 1 }),
        "--json" => exit(if run_battery(true) { 0 } else { 1 }),
        "fuzz" => {
            let iterations = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20_000);
            println!("NONOS trailer parser fuzz, {iterations} iterations\n");
            exit(if fuzz(iterations) { 0 } else { 1 });
        }
        other => {
            eprintln!("unknown mode {other:?}");
            eprintln!("usage: nonos-attack [battery [--json] | fuzz [iterations]]");
            exit(2);
        }
    }
}
