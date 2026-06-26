// NONOS verification engine. The GitHub workflows invoke
// `nonos-verify <subcommand>`; all the build orchestration, trust-chain
// verification, adversarial simulation, supply-chain, and
// attestation logic lives here in typed Rust, not in shell.

mod adversarial;
mod attest;
mod build;
mod evidence;
mod release;
mod report;
mod reproducible;
mod runtime;
mod sh;
mod supply_chain;
mod trust_chain;

use report::Status;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("");
    let root = std::env::var("NONOS_VERIFY_REPORTS").unwrap_or_else(|_| "ci-reports".to_string());

    let result = match cmd {
        "build" => build::run(&root),
        "evidence" => evidence::run(&root),
        "trust-chain" => trust_chain::run(&root),
        "adversarial" => adversarial::run(&root),
        "reproducible" => reproducible::run(&root),
        "release" => release::run(&root),
        "runtime" => runtime::run(&root),
        "supply-chain" => supply_chain::run(&root),
        "attest" => attest::run(&root),
        "" | "-h" | "--help" | "help" => {
            eprintln!("nonos-verify: NONOS verification engine\n");
            eprintln!("usage: nonos-verify <subcommand>\n");
            eprintln!("  build          compile profile, section sizes, binary symbol scan");
            eprintln!("  evidence       benchmark/evidence tooling and production bootloader gate");
            eprintln!("  trust-chain    decode + verify every capsule against the baked anchor");
            eprintln!(
                "  adversarial    tamper valid artifacts and assert the verifier rejects them"
            );
            eprintln!("  runtime        bounded QEMU boot proof and serial marker assertions");
            eprintln!("  reproducible   clean worktree double-build hash comparison");
            eprintln!("  release        release bundle manifest, checksums, provenance");
            eprintln!("  supply-chain   advisories, license policy, submodule pins, SBOM");
            eprintln!("  attest         fuse all module reports into one signed attestation");
            std::process::exit(if cmd.is_empty() { 2 } else { 0 });
        }
        other => {
            eprintln!("nonos-verify: unknown subcommand '{other}' (try --help)");
            std::process::exit(2);
        }
    };

    match result {
        Ok(Status::Fail) => std::process::exit(1),
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("nonos-verify: {cmd}: io error: {e}");
            std::process::exit(3);
        }
    }
}
