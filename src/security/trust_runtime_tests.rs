use crate::capabilities::Capability;
use crate::security::capsule_manifest::{verify_with_publisher, DeclaredEndpoint, EndpointKind};
use crate::security::nonos_id_cert::{
    decode as decode_id_cert, verify as verify_id_cert, NONOS_PRODUCTION_POLICY,
};
use crate::security::nonos_trust_anchor::{decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY};

const TARGET_TRIPLE: &str = "x86_64-nonos-user";

fn caps(xs: &[Capability]) -> u64 {
    xs.iter().fold(0u64, |bits, cap| bits | cap.bit())
}

fn verify_capsule(
    elf: &[u8],
    cert_bytes: &[u8],
    manifest_bytes: &[u8],
    service: (&str, u32),
    reply: (&str, u32),
    requested_caps: u64,
) {
    let trust = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY).unwrap();
    let cert = decode_id_cert(cert_bytes).unwrap();
    let verified = verify_id_cert(cert_bytes, &trust, &NONOS_PRODUCTION_POLICY, None).unwrap();
    let declared = [
        DeclaredEndpoint { kind: EndpointKind::Service, port: service.1, name: service.0 },
        DeclaredEndpoint { kind: EndpointKind::Reply, port: reply.1, name: reply.0 },
    ];
    verify_with_publisher(
        manifest_bytes,
        cert_bytes,
        &cert,
        &verified,
        &trust,
        &NONOS_PRODUCTION_POLICY,
        elf,
        TARGET_TRIPLE,
        requested_caps,
        &declared,
    )
    .unwrap();
}

#[test]
fn entropy_capsule_runtime_trust_verifies() {
    verify_capsule(
        include_bytes!("../../userland/capsule_entropy/target/x86_64-nonos-user/release/entropy"),
        include_bytes!("../../nonos-data/trust/capsules/entropy.nonos_id_cert.bin"),
        include_bytes!("../../nonos-data/trust/capsules/entropy.manifest.bin"),
        ("entropy_pool", 4100),
        ("endpoint.4294967299", 4101),
        caps(&[Capability::IPC, Capability::Memory, Capability::Crypto]),
    );
}

#[test]
fn virtio_rng_capsule_runtime_trust_verifies() {
    verify_capsule(
        include_bytes!(
            "../../userland/capsule_driver_virtio_rng/target/x86_64-nonos-user/release/driver_virtio_rng"
        ),
        include_bytes!("../../nonos-data/trust/capsules/driver_virtio_rng.nonos_id_cert.bin"),
        include_bytes!("../../nonos-data/trust/capsules/driver_virtio_rng.manifest.bin"),
        ("driver.virtio_rng", 4200),
        ("endpoint.4294967302", 4201),
        caps(&[
            Capability::IPC,
            Capability::Memory,
            Capability::Driver,
            Capability::DeviceEnum,
            Capability::Mmio,
            Capability::Irq,
            Capability::Dma,
            Capability::Debug,
        ]),
    );
}

#[test]
fn settings_capsule_runtime_trust_verifies() {
    verify_capsule(
        include_bytes!("../../userland/capsule_settings/target/x86_64-nonos-user/release/settings"),
        include_bytes!("../../nonos-data/trust/capsules/settings.nonos_id_cert.bin"),
        include_bytes!("../../nonos-data/trust/capsules/settings.manifest.bin"),
        ("app.settings", 4728),
        ("endpoint.app.settings.reply", 4729),
        caps(&[
            Capability::CoreExec,
            Capability::IPC,
            Capability::Memory,
            Capability::GraphicsDisplayQuery,
            Capability::GraphicsSurfaceCreate,
        ]),
    );
}
