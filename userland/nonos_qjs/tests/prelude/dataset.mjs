// NONOS Operating System (AGPL-3.0-or-later)
// dataset is how markup hands a value to the script that reads it.

import { mkEl } from './element.mjs';

export function datasetChecks(ok) {
  const el = mkEl();
  const ds = globalThis.__njs_dataset(el);

  // The property is camel and the attribute is kebab. Storing the property
  // name as written would put an attribute in the tree markup cannot match.
  ds.userId = 7;
  ok(el.getAttribute('data-user-id') === '7', 'a write becomes a kebab attribute');
  ok(ds.userId === '7', 'and reads back as a string');
  ok(ds.missing === undefined, 'an absent key is undefined, not null');

  el.setAttribute('data-two-part-name', 'v');
  ok(ds.twoPartName === 'v', 'every dash converts, not just the first');

  ok('userId' in ds, 'the presence check sees it');
  ok(!('nothing' in ds), 'and does not see what is absent');

  // Enumeration must skip attributes that are not data, or a component
  // reading its own dataset picks up class and id as if they were its own.
  el.setAttribute('class', 'row');
  ok(Object.keys(ds).sort().join(',') === 'twoPartName,userId',
     `only data attributes enumerate: ${Object.keys(ds)}`);

  delete ds.userId;
  ok(el.getAttribute('data-user-id') === null, 'deleting removes the attribute');
}
