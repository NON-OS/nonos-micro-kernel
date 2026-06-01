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

use nonos_manifest::{build_sign_args, cap_mask, parse_document};

const VALID: &str = r#"
name = "hello_nonos"
namespace = "systems.nonos.hello"
version = "1.2.3"
target = "x86_64-nonos-user"
cert = "keys/publisher.cert"
required_caps = ["CoreExec", "Memory", "GraphicsSurfaceCreate"]
optional_caps = []
pub_seed_ed25519 = "keys/ed25519.seed"
pub_seed_mldsa65 = "keys/mldsa65.seed"

[[endpoint]]
kind = "service"
port = 5000
name = "hello.svc"
"#;

#[test]
fn parses_valid_document() {
    let m = parse_document(VALID).expect("valid manifest must parse");
    assert_eq!(m.name, "hello_nonos");
    assert_eq!(m.version, (1, 2, 3));
    assert_eq!(m.required_caps.len(), 3);
    assert_eq!(m.endpoints.len(), 1);
    assert_eq!(m.pub_seeds.len(), 2);
}

#[test]
fn resolves_caps_and_builds_signer_args() {
    let m = parse_document(VALID).unwrap();
    let mask = cap_mask(&m.required_caps).unwrap();
    assert_eq!(mask, 1 | 16 | 4096);
    let args = build_sign_args(&m, "out/app.elf", "out/app.nmf").unwrap();
    assert!(args.contains(&"sign-manifest".to_string()));
    assert!(args.contains(&"--required-caps".to_string()));
    assert!(args.contains(&"0x1011".to_string()));
}

#[test]
fn rejects_unknown_capability() {
    assert!(cap_mask(&["CoreExec".to_string(), "Bogus".to_string()]).is_err());
}

#[test]
fn rejects_malformed_document() {
    assert!(parse_document("name = unquoted_value").is_err());
    assert!(parse_document("not_a_key_value_line").is_err());
    assert!(parse_document("version = \"1.x\"").is_err());
}
