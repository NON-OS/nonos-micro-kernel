# capsule_browser

## Role

`capsule_browser` is the graphical web client capsule. It owns URL parsing,
page fetch orchestration, response decoding, document layout, scroll state, and
surface painting for the browser window.

## Microkernel contract

The capsule runs as a signed userland process and renders through the desktop
toolkit/compositor surface path. Network access is mediated through the network
capsule stack; the browser does not talk to NIC drivers, MMIO, IRQ, DMA, or PIO
directly.

## Authority

The capsule requires only the authority needed to open UI surfaces, exchange IPC
with runtime services, allocate memory, and use the network service path. It
does not hold driver, device enumeration, raw hardware, filesystem, admin,
debug, or broker grant authority.

## Privacy and persistence

Runtime page state, URL state, decoded body buffers, scroll state, and transient
network responses live in memory. The capsule does not persist browsing history,
cache contents, cookies, credentials, downloaded files, or telemetry.

## Runtime lifecycle

Startup creates the browser application state, registers the window surface, and
enters the event loop. Input events update URL, scroll, focus, and navigation
state. Network responses update the decoded document model, and paint passes
draw the current viewport into the owned surface.

## Failure model

Malformed URLs, transient network failures, truncated bodies, unsupported
content, oversized responses, and decoding errors resolve into bounded UI state.
The capsule must not panic, block the compositor, or gain additional authority
when a page fails to load.

## Current implemented surface

- URL state and browser application loop.
- Keyboard and event handling.
- Response body handling for fetched pages.
- Basic document layout and painting.
- Scroll state and viewport rendering.

## State ownership

The browser owns page, URL, scroll, and render state. The compositor owns final
surface composition. Network capsules own packet, DNS, TCP, socket, and driver
state. The kernel owns only scheduling, memory isolation, capability checks, and
IPC mechanics.

## Operating rules

- Keep remote content untrusted.
- Bound decoded body and layout state.
- Keep driver and packet ownership outside the browser.
- Keep persistence out until an explicit encrypted profile store exists.
- Treat rendering as data presentation, not authority.

## Release evidence

Release evidence is deterministic UI startup, successful DNS and TCP fetch
through the network capsule stack, bounded rendering for malformed pages, no
kernel network protocol ownership, and static proof that the browser has no raw
hardware or broker grant path.
