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

#![no_std]
#![no_main]

extern crate alloc;

use nonos_libc::{heap_init, mk_exit};
use nonos_std::collections::{BTreeMap, HashMap};
use nonos_std::fs;
use nonos_std::sync::{Arc, Mutex};
use nonos_std::time::Instant;
use nonos_std::vec::Vec;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    let start = Instant::now();
    let mut items: Vec<u32> = Vec::new();
    items.push(7);
    items.push(35);
    let mut counts: BTreeMap<&str, u32> = BTreeMap::new();
    counts.insert("nonos", items.iter().copied().sum());
    let total = counts.get("nonos").copied().unwrap_or(0);
    let mut seen: HashMap<&str, u32> = HashMap::default();
    for word in ["nonos", "std", "nonos"] {
        *seen.entry(word).or_insert(0) += 1;
    }
    let hits = seen.get("nonos").copied().unwrap_or(0);
    let shared = Arc::new(Mutex::new(0u32));
    {
        let mut guard = shared.lock().unwrap();
        *guard = total + hits;
    }
    let locked = *shared.lock().unwrap();
    let _ = fs::write(b"/std_proof.txt", b"nonos_std fs works\n");
    let stored = fs::read(b"/std_proof.txt").map(|d| d.len()).unwrap_or(0);
    let elapsed = start.elapsed().as_millis();
    let pid = nonos_std::process::id();
    nonos_std::println!(
        "nonos_std live: total {total}, hits {hits}, locked {locked}, pid {pid}, file {stored} bytes, {elapsed} ms"
    );
    mk_exit(0)
}
