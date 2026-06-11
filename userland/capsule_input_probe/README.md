# capsule_input_probe

## Role

`capsule_input_probe` is an input-stack test capsule. It subscribes to input
events, renders observed events to a compositor surface, and supports the
input end-to-end harness.

```text
input drivers -> input_router -> input_probe -> compositor -> driver.virtio_gpu
```

## Microkernel contract

- `MkIpcCall` talks to compositor and input-router services.
- `MkIpcRecv` receives routed input events.
- `MkSurfaceRegister`, `MkSurfaceAttach`, and `MkSurfacePresent` publish the
  probe surface.
- `MkExit` terminates the capsule.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x1819`: CoreExec, IPC, Memory,
GraphicsDisplayQuery, and GraphicsSurfaceCreate. The capsule is a test
consumer of input events and has no direct device, IRQ, PIO, DMA, filesystem,
network, crypto, admin, or debug authority.

## Persistence

No persistent state. Event history is in RAM only and is lost on exit or reboot.

## Evidence Status

Partially proven. Source-level contract exists; full proof requires the PS/2
and xHCI input boot harnesses to pass on the target.
