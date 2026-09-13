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

//! The receipt as a person reads it.
//!
//! Network reachability gets its own column instead of sitting inside a list
//! of twenty names. It is what nearly every reader is checking, and a property
//! you have to count out of a list is one nobody checks twice.

use crate::decode::{capability_names, reaches_network, Authority};
use crate::entries::Entry;
use crate::hexcode::hex;

/// Enough to compare against a published build by eye, not enough to mistake
/// for the whole measurement.
fn short(measurement: &[u8; 32]) -> String {
    hex(&measurement[..6])
}

/// Call only after the fold has matched.
pub fn render(entries: &[Entry]) -> String {
    let mut out = format!("{} capsules, all folded into the signed root\n\n", entries.len());
    out.push_str("  pid  measurement   network  capabilities\n");
    out.push_str("  ---  ------------  -------  ------------\n");
    for e in entries {
        let net = if reaches_network(e.caps) { "YES" } else { "no" };
        out.push_str(&format!(
            "  {:>3}  {}  {:^7}  {}\n",
            e.pid,
            short(&e.measurement),
            net,
            capability_names(e.caps).join(", ")
        ));
    }
    out.push_str(&summary(entries));
    out
}

fn summary(entries: &[Entry]) -> String {
    let networked = entries.iter().filter(|e| reaches_network(e.caps)).count();
    let vendor = entries.iter().filter(|e| e.authority == Authority::Vendor).count();
    let publisher = entries.iter().filter(|e| e.authority == Authority::Publisher).count();
    let developer = entries.len() - vendor - publisher;
    format!(
        "\n{} of {} capsules can reach the network. {} were proved by the vendor \
         key, {} by a key enrolled on that machine, and {} carry a publisher \
         signature with no proof behind it.\n",
        networked,
        entries.len(),
        vendor,
        developer,
        publisher,
    )
}
