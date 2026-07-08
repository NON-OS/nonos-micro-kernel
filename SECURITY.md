# Security Policy

NONOS is a from-scratch, capability-based microkernel. Its trust chain (hybrid
Ed25519 and ML-DSA-65 signatures over every loaded capsule, chaining to a baked
anchor) and its per-syscall capability enforcement are the security-critical
core, so reports about them are taken seriously.

## Reporting a vulnerability

Report suspected vulnerabilities privately to team@nonos.systems (or
ek@nonos.systems). Please do not open a public issue for a security bug before
it has been fixed.

Where you can, include:

- the affected component (kernel, bootloader, a capsule, or the build and
  signing chain)
- the commit or release it was found on
- a description and, ideally, a reproduction

## Scope

In scope: memory-safety and isolation breaks, capability-enforcement bypasses,
signature or attestation bypasses in the boot and capsule-load path, and
privilege escalation across the syscall boundary.

Out of scope: issues that require an already-compromised trust anchor or signing
key, and denial of service from a capability the operator explicitly granted.

## Supported versions

The project is pre-1.0 and under active development; fixes land on the main
branch. There is no long-term-support branch yet.
