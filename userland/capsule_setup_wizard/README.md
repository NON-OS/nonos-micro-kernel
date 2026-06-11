# capsule_setup_wizard

## Role

`capsule_setup_wizard` owns first-boot setup UI. It discovers compositor,
input-router, and policy services, paints a guided setup surface, receives
input events, and submits policy choices through explicit capsule IPC.

```text
setup_wizard -> compositor
      |       -> input_router
      `       -> policy capsule
```

## Microkernel contract

- `MkIpcCall` talks to compositor, input-router, and policy services.
- `MkIpcRecv` receives setup input events.
- `MkSurfaceRegister`, `MkSurfaceAttach`, and `MkSurfacePresent` own the
  setup surface.
- `MkExit` terminates the wizard after completion or failure.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x1819`: CoreExec, IPC, Memory,
GraphicsDisplayQuery, and GraphicsSurfaceCreate. The wizard has no ambient
filesystem, network, hardware broker, DMA, PIO, IRQ, crypto, admin, or debug
authority.

## Persistence

The wizard itself does not persist state. Any durable setup choice must pass
through the policy capsule and its manifest/capability checks.

## Evidence Status

Partially proven. The source contract is now documented; a full claim requires
the setup-wizard boot flow and policy handoff harness to pass.
