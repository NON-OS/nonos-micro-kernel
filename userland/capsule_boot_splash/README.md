# capsule_boot_splash

## Role

`capsule_boot_splash` is the first visual userland client after handoff. It
waits for the compositor, paints a fullscreen boot splash, optionally displays
attestation detail after keyboard input, and exits so the desktop fleet can
take over.

```text
boot_splash -> compositor -> driver.virtio_gpu
      |
      `-> input_router for optional detail toggle
```

## Microkernel contract

- `MkIpcCall` talks to compositor and input-router services.
- `MkIpcRecvFrom` receives bounded key events.
- `MkSurfaceRegister`, `MkSurfaceAttach`, `MkSurfacePresent`, and
  `MkSurfaceRelease` own the temporary splash surface lifecycle.
- `MkAttestStatus` reads the boot attestation status.
- `MkTimeMillis`, `MkYield`, and `MkExit` bound runtime and exit.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x1819`: CoreExec, IPC, Memory,
GraphicsDisplayQuery, and GraphicsSurfaceCreate. It does not request network,
filesystem, crypto, hardware broker, DMA, PIO, IRQ, admin, or debug authority.

## Persistence

The capsule writes no files and holds no persistent state. It paints only into
its own temporary surface and releases that surface before exit.

## Evidence Status

Partially proven. The capsule is signed and attested with the committed fleet;
full boot-splash visual correctness still depends on QEMU/runtime boot proof.
