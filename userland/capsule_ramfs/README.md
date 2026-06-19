# capsule_ramfs

## Role

`capsule_ramfs` is the volatile in-memory file capsule. It provides a small
file namespace to other capsules without putting filesystem policy back into
the kernel and without touching persistent storage.

```text
client capsule
    |
    | file IPC
    v
ramfs -- in-memory store --> volatile file bytes
    |
    `-- reply over IPC
```

## Microkernel contract

The capsule is a normal IPC service:

- `MkIpcRecv` receives requests on `service:4096:ramfs`.
- `MkIpcSend` replies on the caller's reply path.
- `MkExit` is used for fatal startup failure.
- The kernel-side mirror is `src/fs/ramfs_capsule`.

The kernel routes IPC and schedules the process. It does not keep file tables,
file bytes, directory policy, or ramfs mutation logic.

## Interface contract

| Surface | Purpose |
|---|---|
| file protocol | open/read/write/truncate-style memory file operations |
| handle table | runtime mapping of caller handles to in-memory records |
| IPC reply | returns bytes or deterministic errno values |

## Authority

The manifest grants `IPC`, `Memory`, and the service authority represented by
`CAPSULE_REQUIRED_CAPS = 0x38`. It has no driver, MMIO, IRQ, DMA, PIO, network,
admin, debug, or persistent-storage authority.

## Privacy and persistence

All file bytes are memory-resident and vanish when the capsule exits or the
system reboots. The capsule does not write to disk. Clients must treat the
service as volatile scratch state, not durable storage.

## Runtime lifecycle

The capsule starts empty, serves file requests from memory, updates its handle
and content tables, and drops all state when the process exits.

## Failure model

Allocation exhaustion, invalid handle, missing file, and invalid request
payloads return protocol errors. No kernel fallback filesystem is invoked.

## Current implemented surface

- Maintains an in-memory file store.
- Handles open/read/write/truncate-style operations through its protocol.
- Keeps handles and file content in capsule-owned memory.
- Has a kernel mirror and validation profile in the integration matrix.

## Wire format

The ramfs wire protocol is an IPC request/reply envelope carrying operation id,
handle/path metadata, byte ranges, and payload bytes. Replies return either
data bytes or a deterministic errno value. The exact layout lives in
`src/protocol`.

## State ownership

The capsule owns file records, handle records, and file bytes. VFS owns
descriptor routing above it. The kernel owns no ramfs bytes and no ramfs handle
table.

## Operating rules

- Treat all state as volatile.
- Bound memory growth and reject requests that exceed capacity.
- Return explicit errors for invalid handles and malformed requests.
- Never add block-device persistence here.

## Release target

The finished ramfs capsule has bounded memory accounting, deterministic handle
lifetime, robust error mapping, validation coverage for create/read/write/truncate,
and clear teardown semantics. It remains volatile by design and never becomes
a disk filesystem.

## Release evidence

Release evidence is the ramfs validation check covering create, read, write,
truncate, invalid handle, and teardown with no kernel-resident file state.

## Release checklist

- Validation covers create/read/write/truncate/delete or equivalent flows.
- Invalid-handle and malformed-request errors are covered.
- Memory bounds are documented and enforced.
- Kernel has no ramfs byte store.

## Explicit non-goals today

No disk backing, journaling, fsck, permissions database, encryption-at-rest,
mount stack, block driver, or persistent namespace is implemented here.

## Verification

- Build: `make -B nonos-mk-ramfs`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: filesystem mutation logic must stay in userland.
