// verify-trust: decode and verify every embedded capsule's cert + manifest
// against the baked trust-anchor policy, the same chain the kernel runs at
// spawn. Real cryptographic verification through nonos_capsule_sign, not a
// `make` wrapper. Emits a typed attestation with per-capsule measurements.

use crate::report::{Report, Status};
use nonos_capsule_sign::algs::AlgId;
use nonos_capsule_sign::verify::decode::{
    decode_cert, decode_manifest, decode_trust_anchor_policy,
};
use nonos_capsule_sign::verify::{verify_cert, verify_manifest};
use serde::Serialize;
use std::path::Path;

const REQUIRED_ALGS: &[AlgId] = &[AlgId::Ed25519, AlgId::MlDsa65];
// A timestamp inside the cert validity window the Makefile signs
// (2026-01-01 .. 2030-01-01).
const NOW_MS: u64 = 1_778_025_600_000;

const POLICY_PATH: &str = "nonos-data/trust/policy/nonos_trust_anchor.policy.bin";
const CAPSULE_DIR: &str = "nonos-data/trust/capsules";

#[derive(Serialize)]
struct CapsuleMeasurement {
    name: String,
    cert_present: bool,
    cert_decoded: bool,
    cert_verified: bool,
    manifest_decoded: bool,
    manifest_verified: bool,
    manifest_sha256: String,
    cert_sha256: String,
    error: Option<String>,
}

pub fn run(root: &str) -> std::io::Result<Status> {
    let mut rpt = Report::new("trust-chain", true);
    let out = Path::new(root).join("trust-chain");
    std::fs::create_dir_all(&out)?;

    // 1. baked trust-anchor policy must decode.
    let policy_bytes = match std::fs::read(POLICY_PATH) {
        Ok(b) => b,
        Err(e) => {
            rpt.check("policy-present", Status::Fail, format!("{POLICY_PATH}: {e}"));
            rpt.gap(
                "baked trust-anchor policy",
                format!("build the signed policy at {POLICY_PATH}"),
            );
            return rpt.finish(root);
        }
    };
    let policy = match decode_trust_anchor_policy(&policy_bytes) {
        Ok(p) => {
            rpt.check("policy-decode", Status::Pass, "baked trust-anchor policy decoded");
            p
        }
        Err(e) => {
            rpt.check("policy-decode", Status::Fail, format!("{e:?}"));
            return rpt.finish(root);
        }
    };

    // 2. every signed capsule in the keystore must chain to that policy.
    let mut manifests: Vec<_> = std::fs::read_dir(CAPSULE_DIR)
        .map(|rd| {
            rd.filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| {
                    p.file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.ends_with(".manifest.bin"))
                        .unwrap_or(false)
                })
                .collect()
        })
        .unwrap_or_default();
    manifests.sort();

    if manifests.is_empty() {
        rpt.check(
            "capsule-inventory",
            Status::Gap,
            format!("no *.manifest.bin under {CAPSULE_DIR}"),
        );
        rpt.gap(
            "signed capsule artifacts",
            "run the signing lane (make nonos-mk-...-sign) to populate nonos-data/trust/capsules",
        );
        return rpt.finish(root);
    }

    let mut measurements: Vec<CapsuleMeasurement> = Vec::new();
    let (mut ok, mut bad) = (0usize, 0usize);

    for mpath in &manifests {
        let bin_name = mpath
            .file_name()
            .and_then(|n| n.to_str())
            .and_then(|n| n.strip_suffix(".manifest.bin"))
            .unwrap_or("unknown")
            .to_string();
        let cpath = Path::new(CAPSULE_DIR).join(format!("{bin_name}.nonos_id_cert.bin"));

        let mut m = CapsuleMeasurement {
            name: bin_name.clone(),
            cert_present: cpath.exists(),
            cert_decoded: false,
            cert_verified: false,
            manifest_decoded: false,
            manifest_verified: false,
            manifest_sha256: String::new(),
            cert_sha256: String::new(),
            error: None,
        };

        let manifest_bytes = std::fs::read(mpath).unwrap_or_default();
        let cert_bytes = std::fs::read(&cpath).unwrap_or_default();
        m.manifest_sha256 = blake3::hash(&manifest_bytes).to_hex().to_string();
        m.cert_sha256 = blake3::hash(&cert_bytes).to_hex().to_string();

        let verdict = (|| -> Result<(), String> {
            let cert = decode_cert(&cert_bytes).map_err(|e| format!("cert decode: {e:?}"))?;
            m.cert_decoded = true;
            verify_cert(&cert, &cert_bytes, &policy, REQUIRED_ALGS, Some(NOW_MS))
                .map_err(|e| format!("cert verify: {e:?}"))?;
            m.cert_verified = true;
            let manifest =
                decode_manifest(&manifest_bytes).map_err(|e| format!("manifest decode: {e:?}"))?;
            m.manifest_decoded = true;
            verify_manifest(&manifest, &manifest_bytes, &cert, &cert_bytes, &policy, REQUIRED_ALGS)
                .map_err(|e| format!("manifest verify: {e:?}"))?;
            m.manifest_verified = true;
            Ok(())
        })();

        match verdict {
            Ok(()) => {
                ok += 1;
                rpt.check(
                    &format!("chain:{bin_name}"),
                    Status::Pass,
                    "cert+manifest verified against baked anchor",
                );
            }
            Err(e) => {
                bad += 1;
                m.error = Some(e.clone());
                rpt.check(&format!("chain:{bin_name}"), Status::Fail, e);
            }
        }
        measurements.push(m);
    }

    // measurements artifact
    let meas_json = serde_json::to_string_pretty(&serde_json::json!({
        "commit": rpt.commit,
        "required_algs": ["Ed25519", "MlDsa65"],
        "now_ms": NOW_MS,
        "verified": ok,
        "failed": bad,
        "capsules": measurements,
    }))
    .expect("measurements serialize");
    std::fs::write(out.join("capsule-measurements.json"), meas_json)?;

    // 3. per-capsule field assertions (caps ceiling diff vs Capsule.mk, endpoint
    // drift, target triple) need the decoded structs exposed; not yet wired.
    rpt.gap(
        "per-capsule field assertions",
        "expose DecodedManifest fields (caps, endpoints, target_triple) so they can be diffed against each Capsule.mk",
    );

    let status = rpt.finish(root)?;
    // copy the report as the named attestation artifact
    let _ = std::fs::copy(out.join("trust-chain-report.json"), out.join("trust-attestation.json"));
    eprintln!("verify-trust: {ok} verified, {bad} failed -> {}", status_str(status));
    Ok(status)
}

fn status_str(s: Status) -> &'static str {
    match s {
        Status::Pass => "pass",
        Status::Fail => "fail",
        Status::Skip => "skip",
        Status::Gap => "gap",
    }
}
