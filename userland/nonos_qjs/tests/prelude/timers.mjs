// NONOS Operating System (AGPL-3.0-or-later)
// Timers used to drop the delay and fire in registration order, which runs a
// staggered setup out of sequence.

export function timerChecks(ok) {
  const seen = [];
  globalThis.setTimeout(() => seen.push('late'), 100);
  globalThis.setTimeout(() => seen.push('early'), 10);
  globalThis.__njs_flush_timers();
  ok(seen.join(',') === 'early,late', `due order, not registration order: ${seen}`);

  // A component that unmounts clears its timer. Without cancelling it still
  // fires, into a tree it has given up.
  const dropped = [];
  const id = globalThis.setTimeout(() => dropped.push('fired'), 5);
  globalThis.clearTimeout(id);
  globalThis.__njs_flush_timers();
  ok(dropped.length === 0, 'a cleared timeout does not fire');

  // setInterval returned 0 and never fired at all, so anything driven by one
  // simply did not run.
  let ticks = 0;
  const iv = globalThis.setInterval(() => {
    ticks += 1;
    if (ticks === 3) globalThis.clearInterval(iv);
  }, 5);
  globalThis.__njs_flush_timers();
  ok(ticks === 3, `an interval repeats and can be stopped: ${ticks}`);

  // A frame callback runs on the next flush rather than being dropped.
  let framed = false;
  globalThis.requestAnimationFrame(() => { framed = true; });
  globalThis.__njs_flush_timers();
  ok(framed, 'a frame callback runs');

  // A throw in one callback must not take the rest of the queue with it.
  const after = [];
  globalThis.setTimeout(() => { throw new Error('boom'); }, 1);
  globalThis.setTimeout(() => after.push('ran'), 2);
  globalThis.__njs_flush_timers();
  ok(after.length === 1, 'a throwing timer does not stop the queue');
}
