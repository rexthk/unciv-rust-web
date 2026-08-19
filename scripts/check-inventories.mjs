import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import path from 'node:path';

const root = path.resolve('fixtures/unciv-4.21.10/inventory');
const expectations = {
  'ruleset-files.json': 40,
  'unique-types.json': 600,
  'save-fields.json': 100,
  'multiplayer-api.json': 20,
  'screens.json': 150,
  'input-actions.json': 40,
  'media.json': 80,
  'map-editor.json': 20,
};

for (const [name, minimum] of Object.entries(expectations)) {
  const values = JSON.parse(await readFile(path.join(root, name), 'utf8'));
  if (!Array.isArray(values) || values.length < minimum) {
    throw new Error(`${name} expected at least ${minimum} entries, found ${values.length}`);
  }
}

const rulesetFiles = JSON.parse(await readFile(path.join(root, 'ruleset-files.json'), 'utf8'));
for (const rulesetFile of rulesetFiles) {
  const fixture = path.resolve('fixtures/unciv-4.21.10/rulesets', rulesetFile.path);
  const hash = createHash('sha256').update(await readFile(fixture)).digest('hex');
  if (hash !== rulesetFile.sha256) {
    throw new Error(`Ruleset fixture checksum mismatch: ${rulesetFile.path}`);
  }
}

console.log('Compatibility inventories satisfy minimum coverage checks.');
