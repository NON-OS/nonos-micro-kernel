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
//
// Host-side harness for the rail's formatters: the em-dash rendering of a
// metric with no source, and the address writers. Compiled with the host
// toolchain:
//   rustc --edition 2021 --test tests/rail_fmt_host.rs -o /tmp/rail_fmt_host && /tmp/rail_fmt_host
//
// Split out of `rail_host.rs`, which covers the scrolled-column geometry.

#[path = "../src/term"]
mod term {
    pub mod util {
        pub mod copy_into;
        pub mod format_u64;

        pub use copy_into::copy_into;
        pub use format_u64::format_u64;
    }
}

#[path = "../src/paint"]
mod paint {
    pub mod rail_fmt;

    pub mod rail_metric;

    pub mod rail_addr;
}

#[path = "../src/rail"]
mod rail {
    pub mod disk;
    pub mod mem;
    pub mod metrics;
    pub mod net;
    pub mod value;
}

use paint::rail_addr::{ipv4_into, ipv4_pfx};
use paint::rail_fmt::{mib_into, u32_into};
use paint::rail_metric::{one, pair, DASH};
use rail::mem::summarize;
use rail::metrics::Proc;
use rail::value::Metric;

#[test]
fn a_metric_with_no_source_renders_as_a_dash_rather_than_a_zero() {
    let mut b = [0u8; 48];
    assert_eq!(one(&mut b, Metric::<u64>::Unknown, mib_into), DASH);
    assert_eq!(one(&mut b, Metric::<u64>::Unsupported, mib_into), DASH);
    assert_eq!(one(&mut b, Metric::Known(0u32), u32_into), "0", "a measured zero is not a dash");
    assert_eq!(one(&mut b, Metric::<u32>::Unsupported, u32_into), DASH);
}

#[test]
fn a_pair_dashes_each_half_independently() {
    let mut b = [0u8; 48];
    let m = summarize(&[Proc { pid: 1, mem_kb: 3952, ..Proc::EMPTY }]);
    assert_eq!(pair(&mut b, m.used_kb, m.total_kb, mib_into), "3.8 MB / —");
    assert_eq!(pair(&mut b, m.total_kb, m.swap_used_kb, mib_into), "— / —");
    assert_eq!(pair(&mut b, Metric::Known(0u64), Metric::Known(1024u64), mib_into), "0.0 MB / 1.0 MB");
}

#[test]
fn the_ipv4_formatter_writes_a_dotted_quad_with_its_prefix() {
    let mut b = [0u8; 48];
    let n = ipv4_into(&mut b, [10, 0, 2, 15]);
    assert_eq!(core::str::from_utf8(&b[..n]).unwrap(), "10.0.2.15");
    let n = ipv4_into(&mut b, [255, 255, 255, 0]);
    assert_eq!(core::str::from_utf8(&b[..n]).unwrap(), "255.255.255.0");
    assert_eq!(ipv4_pfx(&mut b, Metric::Known([10, 0, 2, 15]), Metric::Known(24)), "10.0.2.15/24");
    assert_eq!(ipv4_pfx(&mut b, Metric::Known([1, 2, 3, 4]), Metric::Unknown), "1.2.3.4");
    assert_eq!(ipv4_pfx(&mut b, Metric::Unknown, Metric::Known(24)), DASH);
}

