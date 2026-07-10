// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the reserved core-service predicate that blocks runtime squatting
//! of keyring/crypto/vfs/entropy/market/net.* names and their ports.
use crate::reserved::is_reserved_service;

#[test]
fn every_core_service_name_is_reserved() {
    for name in ["keyring", "entropy_pool", "crypto_pool", "vfs_pool", "market.index"] {
        assert!(is_reserved_service(name, 9999), "core name {name} must be reserved");
    }
}

#[test]
fn all_net_service_names_are_reserved() {
    for name in [
        "net.core", "net.l2", "net.ip", "net.udp", "net.tcp", "net.dns", "net.dhcp.client",
        "net.sockets", "net.nym", "net.ntp.client",
    ] {
        assert!(is_reserved_service(name, 9999), "net name {name} must be reserved");
    }
}

#[test]
fn core_and_net_ports_are_reserved_regardless_of_name() {
    // Squatting by port with a different name must also be refused.
    for port in [4098u32, 4100, 4102, 4104, 4106, 4107, 4400, 4430, 4482, 4499] {
        assert!(is_reserved_service("some.app.service", port), "port {port} must be reserved");
    }
}

#[test]
fn ordinary_app_names_and_ports_are_allowed() {
    // A normal capsule service on its own name/port is not blocked.
    for (name, port) in
        [("com.app.editor", 5000u32), ("my.game", 6001), ("shell.terminal", 4097), ("x", 4108)]
    {
        assert!(!is_reserved_service(name, port), "{name}:{port} must be registrable");
    }
}
