# capsule_vfs

## Role

`capsule_vfs` is the virtual filesystem routing capsule. It owns file-handle
and descriptor-facing policy above concrete storage services such as ramfs and
future block-backed filesystems.

```text
application capsule
    |
    | VFS IPC
    v
vfs -- fd table / routing policy --> backing filesystem capsule
```

## Microkernel contract

The capsule is an IPC service:

- `MkIpcRecv` receives requests on `service:4104:vfs_pool`.
- `MkIpcSend` replies on `reply:4105:endpoint.4294967301`.
- `MkExit` terminates on fatal setup failure.
- The kernel mirror is `src/fs/vfs_capsule`.

The kernel does not hold per-application file descriptor policy. It routes IPC
and starts the signed capsule.

## Interface contract

| Surface | Purpose |
|---|---|
| descriptor table | caller-visible handle state in userland |
| routing layer | dispatch file operations to backing filesystem capsules |
| errno mapping | return deterministic errors without kernel file policy |

## Authority

The manifest grants `IPC` and `Memory` (`CAPSULE_REQUIRED_CAPS = 0x18`). It has
no driver, MMIO, IRQ, DMA, PIO, network, admin, debug, or direct block-device
authority.

## Privacy and persistence

The VFS stores runtime handle state and routing metadata. It does not persist
file contents on its own. Durable data belongs to backing filesystem/storage
capsules when those are present.

## Runtime lifecycle

The capsule starts with an empty descriptor table, accepts file-operation
requests, routes or resolves them in userland, and drops all descriptor state
on exit.

## Failure model

Invalid descriptor, missing route, malformed path, unsupported operation, and
backing-service failure return protocol errors. The kernel does not open files
on behalf of the capsule.

## Current implemented surface

- Owns the userland file descriptor table logic.
- Exposes VFS protocol handlers over IPC.
- Is embedded, spawned, and validation-covered through the kernel mirror.
- Keeps file policy out of kernel process state.

## Wire format

VFS requests carry operation id, caller-visible descriptor or path metadata,
offset/length fields, and optional payload bytes. Replies carry status,
descriptor ids, metadata, or data bytes.

## State ownership

The capsule owns descriptor tables, routing state, and path normalization
state. Backing filesystem capsules own file contents. The kernel owns no file
descriptor table.

## Operating rules

- Keep descriptor ownership caller-scoped.
- Route storage operations to backing filesystem capsules.
- Return deterministic errors for missing routes and invalid descriptors.
- Never add block-driver or filesystem persistence here.

## Release target

The finished VFS capsule owns descriptor routing, mount-table policy, path
normalization, per-caller handle isolation, and IPC dispatch to backing
filesystem capsules. It has validation coverage for open/read/write/close/error
paths and keeps file policy out of kernel PCB state.

## Release evidence

Release evidence is VFS validation coverage for open/read/write/close/error paths
and static proof that file policy is absent from kernel PCB state.

## Release checklist

- Open/read/write/close validation passes.
- Invalid descriptor and missing route errors are covered.
- Backing filesystem routing is tested.
- Static gate confirms kernel PCB has no file policy table.

## Explicit non-goals today

No block driver, persistent filesystem, journaling, encryption-at-rest,
network filesystem, user database, or device node policy lives directly here.

## Verification

- Build: `make -B nonos-mk-vfs`
- Validation: `nonos-mk-vfs-test`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: VFS policy must remain a userland capsule service.
