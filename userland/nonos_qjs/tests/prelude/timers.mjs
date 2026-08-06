// NONOS Operating System (AGPL-3.0-or-later)
// Timers dropped their delay and fired in registration order, which runs a
// staggered setup out of sequence. The flush also has to be driven by a real
// clock: a queue that moves its own clock to whatever comes next makes a
// repeating timer due again the instant it is requeued.

const flush = now => globalThis.__njs_flush_timers(now);

export function timerChecks(ok) {
  const seen = [];
  globalThis.setTimeout(() => seen.push('late'), 100);
  globalThis.setTimeout(() => seen.push('early'), 10);
  flush(200);
  ok(seen.join(',') === 'early,late', `due order, not registration order: ${seen}`);

  // Nothing runs before its time. Firing early is the same defect as firing
  // in the wrong order: the page's own sequencing stops meaning anything.
  const soon = [];
  globalThis.setTimeout(() => soon.push('x'), 500);
  flush(300);
  ok(soon.length === 0, 'a timer that is not due yet does not run');
  flush(900);
  ok(soon.length === 1, 'and runs once its time arrives');

  // A component that unmounts clears its timer. Without cancelling it still
  // fires, into a tree it has given up.
  const dropped = [];
  const id = globalThis.setTimeout(() => dropped.push('fired'), 5);
  globalThis.clearTimeout(id);
  flush(1000);
  ok(dropped.length === 0, 'a cleared timeout does not fire');

  // setInterval returned 0 and never fired, so anything driven by one did
  // not run at all.
  let ticks = 0;
  const iv = globalThis.setInterval(() => {
    ticks += 1;
    if (ticks === 3) globalThis.clearInterval(iv);
  }, 5);
  // Driven the way the app drives it, one flush per tick. An interval fires
  // at most once per flush and reschedules from the current time, which is
  // what a browser does rather than running catch-up ticks for time that has
  // already gone.
  for (const at of [1010, 1020, 1030, 1040]) flush(at);
  ok(ticks === 3, `an interval repeats and can be stopped: ${ticks}`);

  // The regression that matters most: a repeating timer must not be due
  // again the moment it is requeued. If it is, one interval runs to the
  // iteration cap on every flush and the page never stops long enough to
  // draw, which reads as a browser that has hung.
  let spun = 0;
  const forever = globalThis.setInterval(() => { spun += 1; }, 10);
  flush(1050);
  globalThis.clearInterval(forever);
  ok(spun > 0 && spun <= 4, `an interval fires once per elapsed period, not forever: ${spun}`);

  // A frame callback runs on the next flush rather than being dropped.
  let framed = false;
  globalThis.requestAnimationFrame(() => { framed = true; });
  flush(1060);
  ok(framed, 'a frame callback runs');

  // A throw in one callback must not take the rest of the queue with it.
  const after = [];
  globalThis.setTimeout(() => { throw new Error('boom'); }, 1);
  globalThis.setTimeout(() => after.push('ran'), 2);
  flush(2000);
  ok(after.length === 1, 'a throwing timer does not stop the queue');
}
