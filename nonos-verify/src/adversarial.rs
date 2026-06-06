// redteam: adversarial proofs against the real trust-chain verifier. We take a
// genuinely valid capsule, tamper it in each way an attacker would, and assert
// nonos_capsule_sign REJECTS it. A tampered artifact that VERIFIES is a real
// vulnerability and fails CI. This is a true attack simulation, not a grep.

use crate::report::{Report, Status};
use nonos_capsule_sign::algs::AlgId;
use nonos_capsule_sign::verify::decode::{decode_cert, decode_manifest, decode_trust_anchor_policy};
use nonos_capsule_sign::verify::{verify_cert, verify_manifest};
use serde::Serialize;
use std::path::Path;

const REQUIRED_ALGS: &[AlgId] = &[AlgId::Ed25519, AlgId::MlDsa65];
const NOW_MS: u64 = 1_778_025_600_000;
const POLICY_PATH: &str = "nonos-data/trust/policy/nonos_trust_anchor.policy.bin";
const CAPSULE_DIR: &str = "nonos-data/trust/capsules";

#[derive(Serialize)]
struct AttackResult {
    attack: String,
    expectation: &'static str, // always "denied"
    outcome: String,           // "denied" or "ACCEPTED"
    contained: bool,
    detail: String,
}

/// A full decode+verify of a (cert, manifest) pair against a policy. Ok means
/// the chain accepted the artifact; Err means it was rejected.
fn verify_chain(
    cert_bytes: &[u8],
    manifest_bytes: &[u8],
    policy_bytes: &[u8],
    now_ms: u64,
    algs: &[AlgId],
) -> Result<(), String> {
    let policy = decode_trust_anchor_policy(policy_bytes).map_err(|e| format!("policy decode: {e:?}"))?;
    let cert = decode_cert(cert_bytes).map_err(|e| format!("cert decode: {e:?}"))?;
    verify_cert(&cert, cert_bytes, &policy, algs, Some(now_ms)).map_err(|e| format!("cert verify: {e:?}"))?;
    let manifest = decode_manifest(manifest_bytes).map_err(|e| format!("manifest decode: {e:?}"))?;
    verify_manifest(&manifest, manifest_bytes, &cert, cert_bytes, &policy, algs)
        .map_err(|e| format!("manifest verify: {e:?}"))?;
    Ok(())
}

fn flip_mid(mut bytes: Vec<u8>) -> Vec<u8> {
    if !bytes.is_empty() {
        let i = bytes.len() / 2;
        bytes[i] ^= 0xFF;
    }
    bytes
}

pub fn run(root: &str) -> std::io::Result<Status> {
    let mut rpt = Report::new("adversarial", true);
    let out = Path::new(root).join("adversarial");
    std::fs::create_dir_all(&out)?;

    let policy_bytes = match std::fs::read(POLICY_PATH) {
        Ok(b) => b,
        Err(e) => {
            rpt.check("baseline", Status::Gap, format!("policy missing: {e}"));
            rpt.gap("attack baseline", "signed trust-anchor policy + capsule artifacts must exist");
            return rpt.finish(root);
        }
    };

    // Find a genuinely valid victim (and a second distinct cert for cross-binding).
    let mut pairs: Vec<(String, Vec<u8>, Vec<u8>)> = Vec::new();
    if let Ok(rd) = std::fs::read_dir(CAPSULE_DIR) {
        let mut names: Vec<String> = rd
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
            .filter_map(|n| n.strip_suffix(".manifest.bin").map(|s| s.to_string()))
            .collect();
        names.sort();
        for name in names {
            let c = std::fs::read(Path::new(CAPSULE_DIR).join(format!("{name}.nonos_id_cert.bin"))).unwrap_or_default();
            let m = std::fs::read(Path::new(CAPSULE_DIR).join(format!("{name}.manifest.bin"))).unwrap_or_default();
            if verify_chain(&c, &m, &policy_bytes, NOW_MS, REQUIRED_ALGS).is_ok() {
                pairs.push((name, c, m));
            }
            if pairs.len() >= 2 {
                break;
            }
        }
    }

    if pairs.is_empty() {
        rpt.check("baseline", Status::Gap, "no capsule verifies cleanly to attack from");
        rpt.gap("attack baseline", "a built+signed capsule set so the verifier has a valid victim to tamper");
        return rpt.finish(root);
    }

    let (victim, vcert, vmanifest) = pairs[0].clone();
    rpt.check("baseline", Status::Pass, format!("valid victim: {victim}"));

    // Each attack must be DENIED. The closure returns Ok if the chain ACCEPTED
    // the tampered artifact, which is the failure (a real vuln).
    let mut attacks: Vec<(String, Box<dyn Fn() -> Result<(), String>>)> = Vec::new();

    attacks.push((
        "cert-bitflip".into(),
        Box::new({
            let c = flip_mid(vcert.clone());
            let m = vmanifest.clone();
            let p = policy_bytes.clone();
            move || verify_chain(&c, &m, &p, NOW_MS, REQUIRED_ALGS)
        }),
    ));
    attacks.push((
        "manifest-bitflip".into(),
        Box::new({
            let c = vcert.clone();
            let m = flip_mid(vmanifest.clone());
            let p = policy_bytes.clone();
            move || verify_chain(&c, &m, &p, NOW_MS, REQUIRED_ALGS)
        }),
    ));
    attacks.push((
        "policy-bitflip".into(),
        Box::new({
            let c = vcert.clone();
            let m = vmanifest.clone();
            let p = flip_mid(policy_bytes.clone());
            move || verify_chain(&c, &m, &p, NOW_MS, REQUIRED_ALGS)
        }),
    ));
    attacks.push((
        "cert-expired".into(),
        Box::new({
            let c = vcert.clone();
            let m = vmanifest.clone();
            let p = policy_bytes.clone();
            move || verify_chain(&c, &m, &p, u64::MAX, REQUIRED_ALGS)
        }),
    ));
    attacks.push((
        "cert-premature".into(),
        Box::new({
            let c = vcert.clone();
            let m = vmanifest.clone();
            let p = policy_bytes.clone();
            move || verify_chain(&c, &m, &p, 1, REQUIRED_ALGS)
        }),
    ));
    attacks.push((
        "cert-truncated".into(),
        Box::new({
            let c = vcert[..vcert.len() / 2].to_vec();
            let m = vmanifest.clone();
            let p = policy_bytes.clone();
            move || verify_chain(&c, &m, &p, NOW_MS, REQUIRED_ALGS)
        }),
    ));
    if pairs.len() >= 2 {
        let other_cert = pairs[1].1.clone();
        attacks.push((
            "cross-binding".into(),
            Box::new({
                let c = other_cert;
                let m = vmanifest.clone();
                let p = policy_bytes.clone();
                move || verify_chain(&c, &m, &p, NOW_MS, REQUIRED_ALGS)
            }),
        ));
    } else {
        rpt.gap("cross-binding attack", "needs a second distinct verified capsule");
    }

    let mut results: Vec<AttackResult> = Vec::new();
    for (name, f) in attacks {
        match f() {
            Err(why) => {
                rpt.check(&format!("attack:{name}"), Status::Pass, format!("DENIED ({why})"));
                results.push(AttackResult {
                    attack: name,
                    expectation: "denied",
                    outcome: "denied".into(),
                    contained: true,
                    detail: why,
                });
            }
            Ok(()) => {
                rpt.check(&format!("attack:{name}"), Status::Fail, "ACCEPTED a tampered artifact (vulnerability)");
                results.push(AttackResult {
                    attack: name,
                    expectation: "denied",
                    outcome: "ACCEPTED".into(),
                    contained: false,
                    detail: "the verifier accepted a tampered artifact".into(),
                });
            }
        }
    }

    let denied = results.iter().filter(|r| r.contained).count();
    let attestation = serde_json::to_string_pretty(&serde_json::json!({
        "commit": rpt.commit,
        "victim": victim,
        "attacks_total": results.len(),
        "attacks_denied": denied,
        "results": results,
    }))
    .expect("attestation serialize");
    std::fs::write(out.join("security-attestation.json"), attestation)?;

    let status = rpt.finish(root)?;
    eprintln!("redteam: {denied}/{} attacks denied", results.len());
    Ok(status)
}
