# capsule_market

## Role

`capsule_market` is the signed capsule marketplace and install-readiness
service. It verifies marketplace index material, applies trust policy, and
answers whether a capsule release is ready for installation.

```text
operator-supplied index
    |
    | load / verify IPC
    v
market -- trust anchors + install checks --> install-ready result
```

## Microkernel contract

The capsule is a signed IPC service:

- `MkIpcRecv` receives requests on `service:4106:market.index`.
- `MkIpcSend` replies on `reply:4107:endpoint.4294967303`.
- `MkExit` terminates on fatal setup failure.
- The kernel mirror is `src/security/market_capsule`.

The kernel does not parse marketplace indexes or make package policy. It
verifies the capsule, routes IPC, and leaves marketplace decisions in userland.

## Interface contract

| Surface | Purpose |
|---|---|
| load index | ingest a signed marketplace index |
| list/get | return indexed capsule and release metadata |
| install-ready | evaluate trust, signature, platform, and policy checks |
| reject path | return deterministic refusal reasons |

## Authority

The manifest grants `IPC` and `Memory` (`CAPSULE_REQUIRED_CAPS = 0x18`). It has
no driver, MMIO, IRQ, DMA, PIO, filesystem, network, admin, debug, or direct
loader authority.

## Privacy and persistence

The capsule keeps the loaded index in runtime memory. It does not install
capsules by itself, write persistent state, or bypass signature policy. The
operator key set is part of the capsule trust configuration.

## Runtime lifecycle

The capsule starts with trusted operator keys, accepts an index load, verifies
it, stores the accepted index in memory, and answers list/get/install-ready
queries until exit.

## Failure model

Malformed index, untrusted operator, bad signature, unsupported platform,
missing release, and policy mismatch return explicit protocol errors. The
capsule never asks the kernel to install unverified bytes.

## Current implemented surface

- Rejects unsigned or malformed marketplace input.
- Verifies accepted index material against trusted operator keys.
- Exposes list/get/install-ready operations through its IPC protocol.
- Keeps marketplace policy out of the kernel.

## Wire format

Requests carry operation id and index or query payloads. Replies carry status,
metadata records, release records, or install-readiness results. The index
format remains signed and versioned outside the kernel.

## State ownership

The capsule owns loaded index state, trust-evaluation state, and install-ready
decision state. The loader owns final capsule admission. The kernel does not
parse marketplace indexes.

## Operating rules

- Reject unsigned or malformed indexes.
- Keep operator and publisher trust explicit.
- Return deterministic refusal reasons.
- Never fetch or install code directly.

## Release target

The finished market capsule verifies signed indexes, enforces publisher and
operator trust policy, evaluates install readiness, supports rollback-safe
release selection, and exposes deterministic errors for rejected releases. It
never fetches unauthenticated code or bypasses the capsule loader.

## Release evidence

Release evidence is marketplace-index validation for valid index, mutated body,
untrusted operator, rollback selection, and install-ready refusal paths.

## Release checklist

- Valid signed index validation passes.
- Mutated body is rejected.
- Untrusted operator is rejected.
- Rollback and platform checks are covered.
- Loader bypass remains impossible.

## Explicit non-goals today

No network fetcher, payment flow, mutable local package database, direct file
installer, kernel loader bypass, or unsigned development ingest path lives
here.

## Verification

- Build: `make -B nonos-mk-market`
- Host tool smoke: `tools/ci/marketplace_index_smoke.sh`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Security check: unsigned ingest and direct crypto primitive dependencies are
  forbidden by static gates.
