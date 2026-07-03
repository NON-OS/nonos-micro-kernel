# nonos_browser capabilities

What this capsule actually does, and where its edges are.

## Renders

- HTML through a real box engine: block and inline formatting, anonymous
  block wrapping, margins with simple collapsing, padding, borders,
  backgrounds, border-radius, width/height (treated as border-box),
  text-align, line-height, real TTF metrics for wrap and paint.
- Flexbox: row and column, justify-content, align-items (stretch fills the
  cross axis), gap, flex-grow. Reverse directions map to their forward
  counterparts.
- Grid subset: grid-template-columns with px / % / fr / auto, repeat() and
  minmax() (max side wins), gap, row-major auto placement. No spans, no
  areas, no named lines.
- position static/relative/absolute (fixed acts as absolute), top/right/
  bottom/left, z-index ordering, overflow:hidden clipping (vertical clip
  needs a fixed height).
- Selectors: tag/#id/.class compounds, descendant and child combinators,
  [attr] and [attr=value], selector lists, specificity. @media min-width/
  max-width evaluated against the fixed viewport; print blocks never apply.
- Images: PNG/JPEG/BMP fetched over the page's socket, decoded, aspect-fit
  blitted. GIF is recognized but not decoded. Sizing comes from CSS or a
  capped default; natural dimensions do not reflow the page.
- Form widgets: input/textarea/select/button render as boxes; click to
  focus, type to edit, enter or a submit control submits urlencoded GET or
  POST through the same HTTP/TLS machine as navigation.

## Runs

- Scripts execute against the live DOM at load, on clicks (bubbling), on
  input, on submit, and on setTimeout/setInterval ticks (50ms granularity).
- DOM API: getElementById, querySelector/querySelectorAll (full matcher),
  createElement, appendChild/removeChild/remove, get/set/remove/hasAttribute,
  textContent, innerHTML (parsed by the page parser), style.*, classList,
  className/id/value, parentNode, children.
- fetch(url).then(cb) / fetch(url, cb): one request at a time over the
  capsule's socket capability; the callback gets status, ok, body text and
  parsed json. JSON.parse / JSON.stringify.
- Language: functions and closures, var/let/const, if/while/for/for-of,
  arrays with push/pop/shift/join/indexOf/includes/slice/concat/map/filter/
  forEach/find, object literals, string methods, ternary, template
  literals, === / !==, typeof. No classes, no async/await, no try/catch,
  no arrow functions, no regex.

## Out of scope

Floats, CSS transforms/transitions/animations, :hover, WASM, WebGL, video
and audio, service workers, cookies and storage, and V8-class SPA
frameworks. Unknown CSS properties and unsupported JS syntax are skipped,
never faked.

## Limits and containment

Every parser reads through bounds-checked access and hard caps: 60k DOM
nodes, 4096 CSS rules, 20k boxes, 400 levels of recursion, 200k JS tokens,
5M interpreter steps per run (1M per event), 512 listeners, 256 timers, 16
queued fetches, 100k array slots, 32 levels of JSON. The crate compiles
with no panic!/unwrap/expect anywhere; every fallible path returns
Option/Result and fails closed. All network I/O rides the net.sockets IPC
capability granted in the capsule manifest; scripts cannot open any other
channel. Runtime fuzzing needs the OS image and stays a QEMU job; the
static audit of every index and slice in the parse paths stands in for it
here.
