# capsule_wallpaper

## Role

`capsule_wallpaper` is a production desktop capsule. It owns wallpaper surface
creation and presentation policy above the graphics syscall surface.

```text
wallpaper service
    |
    | graphics Mk calls
    v
surface create -> map -> fill -> present -> destroy
    |
    `-- transient desktop background surface
```

## Microkernel contract

The capsule calls the graphics surface API exposed through libc:

- display dimensions
- surface create
- surface map
- full-surface present
- surface destroy
- `MkExit` for completion status

The kernel-side spawn path is feature gated through `nonos-capsule-wallpaper`.

## Interface contract

| Call | Purpose |
|---|---|
| display dimensions | discover framebuffer dimensions |
| surface create/map/present/destroy | exercise the graphics surface lifecycle |
| `MkExit` | completion status |

## Authority

The capsule must declare only the graphics, IPC, and memory authority needed by
the graphics contract.

## Privacy and persistence

The capsule writes a solid color into a transient mapped surface. It does not
read user files, inspect windows, persist pixels, capture input, or store
display state.

## Runtime lifecycle

The capsule creates one background surface, fills it, presents it, and exits
after handing the desktop a stable background.

## Failure model

Graphics `ENOTSUP` exits cleanly. Any failed surface operation exits non-zero.

## Current implemented surface

- Creates, maps, fills, presents, and destroys a graphics surface.
- Exits after the wallpaper presentation path completes.

## Wire format

There is no long-running IPC wire protocol. The visible artifacts are graphics
syscall return values and PASS/FAIL markers emitted through `MkDebug`.

## State ownership

The capsule owns one transient surface id and mapped surface pointer during the
smoke. The graphics backend owns framebuffer mapping. No wallpaper pixels are
persisted.

## Operating rules

- Treat `ENOTSUP` as parked graphics, not success of rendering.
- Destroy the surface on every mapped failure path.
- Do not add desktop policy to this validation capsule.

## Release target

The finished wallpaper capsule, if retained, is a signed graphics validation
artifact with an explicit manifest, feature-gated spawn, deterministic surface
lifecycle, and no desktop policy. If a real wallpaper service is needed, it
should be promoted as a separate UI capsule with storage and permissions
defined up front.

## Release evidence

Release evidence is the graphics validation marker sequence, surface lifecycle
proof, and static proof that framebuffer mapping remains kernel-owned.

## Release checklist

- Surface create/map/present/destroy validation passes.
- Failure markers identify the failed graphics step.
- Static gate confirms no direct framebuffer mapping in userland.
- Parked status is removed only with a real manifest and spawn contract.

## Explicit non-goals today

No compositor, window manager, image loader, theme engine, desktop shell,
input handling, persistent wallpaper storage, or production spawn path lives
here.

## Verification

- Build/validation target: `nonos-mk-wallpaper-test` when the graphics validation slice
  is active.
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Promotion check: this capsule must stay marked parked until it has a real
  manifest, capability mask, and production spawn contract.
