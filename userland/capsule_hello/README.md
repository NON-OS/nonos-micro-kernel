# capsule_hello

## Role

`capsule_hello` is a minimal graphical app capsule using the shared
`nonos_app_skeleton` window path.

```text
hello app -> toolkit -> compositor -> driver.virtio_gpu
```

## Microkernel contract

- `MkIpcCall` registers the window and requests toolkit/compositor work.
- `MkIpcRecv` receives app input events through the skeleton event inbox.
- `MkSurfaceRegister`, `MkSurfaceAttach`, and `MkSurfacePresent` present the
  app surface through the toolkit/compositor path.
- `MkExit` terminates the capsule.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x1819`: CoreExec, IPC, Memory,
GraphicsDisplayQuery, and GraphicsSurfaceCreate. It requests no filesystem,
network, hardware broker, crypto, admin, DMA, PIO, IRQ, or debug capability.

## Persistence

No persistent state. The app reconstructs all UI state in memory on spawn.

## Evidence Status

Partially proven. The capsule participates in committed fleet attestation;
runtime app behavior needs boot-level evidence.
