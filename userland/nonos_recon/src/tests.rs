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

//! The parsers against the awkward inputs a scanner sees, and the scan and
//! report flow against a canned probe.

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use crate::ports::{parse_ports, PortError};
use crate::report::{format_report, PortState, ScanRow};
use crate::scan::{scan, Probe};
use crate::target::{parse_target, Target, TargetError};

#[test]
fn a_dotted_quad_parses() {
    assert_eq!(parse_target("10.0.2.15").unwrap().octets, [10, 0, 2, 15]);
    assert_eq!(parse_target("255.255.255.255").unwrap().octets, [255, 255, 255, 255]);
    assert_eq!(parse_target("0.0.0.0").unwrap().octets, [0, 0, 0, 0]);
}

#[test]
fn malformed_targets_are_rejected() {
    assert_eq!(parse_target("10.0.2"), Err(TargetError::Shape));
    assert_eq!(parse_target("10.0.2.15.1"), Err(TargetError::Shape));
    assert_eq!(parse_target("10.0.2."), Err(TargetError::Octet));
    assert_eq!(parse_target("10..2.15"), Err(TargetError::Octet));
    assert_eq!(parse_target("10.0.2.256"), Err(TargetError::Octet));
    assert_eq!(parse_target("10.0.2.abc"), Err(TargetError::Octet));
    assert_eq!(parse_target(""), Err(TargetError::Octet));
    // No overflow: 999 is caught as out of range, not wrapped.
    assert_eq!(parse_target("10.0.2.999"), Err(TargetError::Octet));
}

#[test]
fn a_port_list_parses_sorted_and_deduped() {
    assert_eq!(parse_ports("80").unwrap(), vec![80]);
    assert_eq!(parse_ports("443,22,80").unwrap(), vec![22, 80, 443]);
    // Overlapping range and single collapse to one probe each.
    assert_eq!(parse_ports("80,80,80").unwrap(), vec![80]);
}

#[test]
fn a_port_range_expands() {
    assert_eq!(parse_ports("20-25").unwrap(), vec![20, 21, 22, 23, 24, 25]);
    // A single-port range is just that port.
    assert_eq!(parse_ports("443-443").unwrap(), vec![443]);
    // Overlapping ranges merge.
    assert_eq!(parse_ports("1-3,2-4").unwrap(), vec![1, 2, 3, 4]);
}

#[test]
fn spaces_around_fields_are_tolerated() {
    assert_eq!(parse_ports(" 22 , 80 ").unwrap(), vec![22, 80]);
    assert_eq!(parse_ports("20 - 22").unwrap(), vec![20, 21, 22]);
}

#[test]
fn out_of_range_ports_are_rejected() {
    assert_eq!(parse_ports("0"), Err(PortError::OutOfRange));
    assert_eq!(parse_ports("65536"), Err(PortError::OutOfRange));
    assert_eq!(parse_ports("70000"), Err(PortError::OutOfRange));
    // The top of the range is valid.
    assert_eq!(parse_ports("65535").unwrap(), vec![65535]);
}

#[test]
fn malformed_specs_are_rejected() {
    assert_eq!(parse_ports(""), Err(PortError::Malformed));
    assert_eq!(parse_ports(","), Err(PortError::Malformed));
    assert_eq!(parse_ports("22,,80"), Err(PortError::Malformed));
    assert_eq!(parse_ports("8o"), Err(PortError::Malformed));
    assert_eq!(parse_ports("22-"), Err(PortError::Malformed));
}

#[test]
fn a_backwards_range_is_rejected() {
    assert_eq!(parse_ports("100-1"), Err(PortError::Backwards));
}

#[test]
fn the_whole_range_is_allowed_and_bounded() {
    // The full range fits exactly.
    let all = parse_ports("1-65535").unwrap();
    assert_eq!(all.len(), 65535);
    assert_eq!(all[0], 1);
    assert_eq!(all[65534], 65535);
}

/// A probe that answers from a fixed set of open ports.
struct Canned {
    open: Vec<u16>,
    refused: Vec<u16>,
}

impl Probe for Canned {
    fn probe(&mut self, _t: &Target, port: u16) -> PortState {
        if self.open.contains(&port) {
            PortState::Open
        } else if self.refused.contains(&port) {
            PortState::Closed
        } else {
            PortState::Filtered
        }
    }
}

#[test]
fn a_scan_reports_a_row_per_port_in_order() {
    let target = Target { octets: [10, 0, 2, 15] };
    let ports = parse_ports("22,80,443,8080").unwrap();
    let mut probe = Canned { open: vec![22, 80], refused: vec![443] };
    let rows = scan(&target, &ports, &mut probe);
    assert_eq!(
        rows,
        vec![
            ScanRow { port: 22, state: PortState::Open },
            ScanRow { port: 80, state: PortState::Open },
            ScanRow { port: 443, state: PortState::Closed },
            ScanRow { port: 8080, state: PortState::Filtered },
        ]
    );
}

#[test]
fn the_report_lists_open_and_closed_and_counts_filtered() {
    let rows = vec![
        ScanRow { port: 22, state: PortState::Open },
        ScanRow { port: 80, state: PortState::Open },
        ScanRow { port: 443, state: PortState::Closed },
        ScanRow { port: 8080, state: PortState::Filtered },
        ScanRow { port: 9000, state: PortState::Filtered },
    ];
    let text = format_report(&[10, 0, 2, 15], &rows);
    assert!(text.contains("recon 10.0.2.15\n"));
    assert!(text.contains("  22\topen\n"));
    assert!(text.contains("  443\tclosed\n"));
    // Filtered ports are counted, not listed line by line.
    assert!(!text.contains("8080\tfiltered"));
    assert!(text.contains("scanned 5: 2 open, 1 closed, 2 filtered\n"));
}
