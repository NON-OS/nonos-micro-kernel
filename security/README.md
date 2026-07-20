# NONOS security operations

This directory holds the offensive and defensive tooling for the NONOS
attestation. The tools are not a simulation. They link the same `nonos-stark`
verifier the bootloader links, operate on the same image byte layout the boot
path parses, and reach the same verdict the machine reaches before it jumps into
the kernel. A green run here is evidence about the shipped gate, not about a
model of it.

## Threat model

NONOS attests two layers with a transparent post-quantum FRI-STARK over the
Goldilocks field. There is no trusted setup and no curve to break with a quantum
computer. Both layers commit to a Merkle root and prove membership of a context.

- Kernel self-attestation. Before the bootloader jumps, it measures the kernel
  region, binds the measurement to the boot epoch, and verifies a trailer proving
  that measurement is a member of the enrolled root. Enrollment and verification
  live in `nonos-stark`, shared by the prover and the bootloader so they agree by
  construction.
- Capsule attestation. At spawn, the kernel binds a capsule's ELF hash, its
  granted capabilities, and the policy epoch, and verifies membership against the
  policy root before it runs.

The attacker we defend against can rewrite the flashed image at rest, ship a
different kernel, steal a genuine trailer, enroll their own kernel under their
own root, corrupt a trailer in transit, and feed a malformed trailer to the
parser to try to crash the boot. The gate must refuse all of it, and the parser
must stay total on any byte string.

## Tools

### nonos-defend (blue team)

Verify an attested image before you flash it, the same check the bootloader runs.

```
nonos-defend <attested-image> <kernel-attest-root.bin>
```

Exit 0 means the kernel self-attestation holds against the enrolled root. A
non-zero exit means do not flash the image.

### nonos-attack (red team)

Mount the attacks a shipped image must survive and confirm the gate refuses each
one. An attack "passes" only when the attestation refuses it.

```
nonos-attack battery [--json]   run the categorized attack battery (default)
nonos-attack fuzz [iterations]  fuzz the untrusted trailer parser
```

The battery covers integrity (tamper a flashed kernel byte, truncate the image),
impersonation (a foreign kernel under a stolen trailer, an empty kernel),
forgery (a trailer under a different root, a bit flipped inside the trailer), and
malformed input (an undersized image). The fuzzer drives the untrusted
`deserialize_proof_ext` parser with random bytes and mutations of a real trailer,
including absurd length prefixes, and asserts the parser never panics or loops.
A boot-time denial of service is a real finding, so the fuzzer exits non-zero if
any input breaks the parser.

Exit 0 means every attack was refused and the parser stayed total. Use `--json`
for machine-readable findings in CI.

## Running the suite

```
security/tests/run.sh
```

The harness builds the crate, runs the battery, checks the JSON report, and runs
a fuzz pass. It is the same sequence CI runs against a release image.
