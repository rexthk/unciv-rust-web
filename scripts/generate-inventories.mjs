import { mkdir, readFile, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  extractKotlinFields,
  extractKotlinSymbols,
  parseEndpoints,
  parseUniqueTypes,
  sha256File,
  uniqueSorted,
  walkFiles,
} from './lib/inventory.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const upstream = path.resolve(process.env.UNCIV_UPSTREAM_DIR ?? '/Users/rexthk/repo/Unciv');
const inventoryRoot = path.join(repoRoot, 'fixtures/unciv-4.21.10/inventory');
const ledgerRoot = path.join(repoRoot, 'docs/task-ledger');

await mkdir(inventoryRoot, { recursive: true });
await mkdir(ledgerRoot, { recursive: true });

const rulesetRoot = path.join(upstream, 'android/assets/jsons');
const rulesetFiles = await walkFiles(
  rulesetRoot,
  (file) => file.endsWith('.json') && !file.includes(`${path.sep}translations${path.sep}`),
);
const rulesetInventory = [];
for (const file of rulesetFiles) {
  const fileStat = await stat(file);
  const relativePath = path.relative(rulesetRoot, file).split(path.sep).join('/');
  rulesetInventory.push({
    path: relativePath,
    category: path.basename(file, '.json'),
    bytes: fileStat.size,
    sha256: await sha256File(file),
  });
}

const uniqueFile = path.join(
  upstream,
  'core/src/com/unciv/models/ruleset/unique/UniqueType.kt',
);
const uniqueInventory = parseUniqueTypes(await readFile(uniqueFile, 'utf8')).map((entry, index) => ({
  id: `UNIQUE-${String(index + 1).padStart(4, '0')}`,
  ...entry,
  source: 'core/src/com/unciv/models/ruleset/unique/UniqueType.kt',
}));

const saveRoots = [
  path.join(upstream, 'core/src/com/unciv/logic'),
  path.join(upstream, 'core/src/com/unciv/models'),
];
const saveFields = [];
for (const saveRoot of saveRoots) {
  for (const file of await walkFiles(saveRoot, (candidate) => candidate.endsWith('.kt'))) {
    const relativePath = path.relative(upstream, file).split(path.sep).join('/');
    saveFields.push(...extractKotlinFields(await readFile(file, 'utf8'), relativePath));
  }
}
const saveInventory = uniqueSorted(saveFields, ({ file, name }) => `${file}:${name}`);

const endpointSource = await readFile(
  path.join(upstream, 'core/src/com/unciv/logic/multiplayer/apiv2/EndpointImplementations.kt'),
  'utf8',
);
const endpoints = parseEndpoints(endpointSource);
endpoints.push({ method: 'GET', path: 'api/version' });
endpoints.push({ method: 'WS', path: 'api/v2/ws' });
const legacyServerSource = await readFile(
  path.join(upstream, 'server/src/com/unciv/app/server/UncivServer.kt'),
  'utf8',
);
for (const match of legacyServerSource.matchAll(/\b(get|put|post|delete)\("([^"]+)"\)/g)) {
  endpoints.push({ method: match[1].toUpperCase(), path: match[2].replace(/^\//, '') });
}
const endpointInventory = uniqueSorted(endpoints, ({ method, path: endpointPath }) => `${method} ${endpointPath}`);

const screensRoot = path.join(upstream, 'core/src/com/unciv/ui/screens');
const screenFiles = await walkFiles(screensRoot, (file) => file.endsWith('.kt'));
const screens = screenFiles.map((file) => {
  const relativePath = path.relative(screensRoot, file).split(path.sep).join('/');
  return {
    id: relativePath.replace(/\.kt$/, '').replace(/[^A-Za-z0-9]+/g, '-').toUpperCase(),
    path: `core/src/com/unciv/ui/screens/${relativePath}`,
    category: relativePath.includes('/') ? relativePath.split('/')[0] : 'root',
    status: 'pending',
  };
});

const bindingSource = await readFile(
  path.join(upstream, 'core/src/com/unciv/ui/components/input/KeyboardBinding.kt'),
  'utf8',
);
const inputActions = [];
for (const match of bindingSource.matchAll(/^ {4}([A-Z][A-Za-z0-9_]*)\s*\(/gm)) {
  inputActions.push({ id: match[1], source: 'KeyboardBinding.kt', status: 'pending' });
}

const mediaRoots = [
  path.join(upstream, 'android/assets/jsons/translations'),
  path.join(upstream, 'android/assets/sounds'),
];
const media = [];
for (const mediaRoot of mediaRoots) {
  for (const file of await walkFiles(mediaRoot)) {
    const fileStat = await stat(file);
    media.push({
      path: path.relative(path.join(upstream, 'android/assets'), file).split(path.sep).join('/'),
      bytes: fileStat.size,
      sha256: await sha256File(file),
    });
  }
}

const editorRoot = path.join(upstream, 'core/src/com/unciv/ui/screens/mapeditorscreen');
const editorOperations = [];
for (const file of await walkFiles(editorRoot, (candidate) => candidate.endsWith('.kt'))) {
  const relativePath = path.relative(editorRoot, file).split(path.sep).join('/');
  const sourcePath = `core/src/com/unciv/ui/screens/mapeditorscreen/${relativePath}`;
  const symbols = extractKotlinSymbols(await readFile(file, 'utf8'));
  if (symbols.length === 0) {
    editorOperations.push({
      id: relativePath.replace(/\.kt$/, '').replace(/[^A-Za-z0-9]+/g, '-').toUpperCase(),
      path: sourcePath,
      symbol: null,
      status: 'pending',
    });
  }
  for (const symbol of symbols) {
    editorOperations.push({
      id: `${relativePath}#${symbol.kind}-${symbol.name}`
        .replace(/\.kt/, '')
        .replace(/[^A-Za-z0-9]+/g, '-')
        .toUpperCase(),
      path: sourcePath,
      symbol: symbol.name,
      status: 'pending',
    });
  }
}

const inventories = {
  'ruleset-files.json': rulesetInventory,
  'unique-types.json': uniqueInventory,
  'save-fields.json': saveInventory,
  'multiplayer-api.json': endpointInventory,
  'screens.json': screens,
  'input-actions.json': uniqueSorted(inputActions, ({ id }) => id),
  'media.json': media.sort((left, right) => left.path.localeCompare(right.path)),
  'map-editor.json': editorOperations,
};

for (const [name, inventory] of Object.entries(inventories)) {
  await writeJson(path.join(inventoryRoot, name), inventory);
}

const ledger = [];
addTasks(ledger, 'RULE', 'ruleset-file', rulesetInventory, (item) => item.path);
addTasks(ledger, 'UNIQUE', 'unique-type', uniqueInventory, (item) => item.name, true);
addTasks(ledger, 'SAVE', 'save-field', saveInventory, (item) => `${item.file}:${item.name}`);
addTasks(ledger, 'API', 'multiplayer-endpoint', endpointInventory, (item) => `${item.method} ${item.path}`);
addTasks(ledger, 'SCREEN', 'screen', screens, (item) => item.path);
addTasks(ledger, 'INPUT', 'input-action', inputActions, (item) => item.id);
addTasks(ledger, 'EDITOR', 'map-editor', editorOperations, (item) => item.path);
await writeJson(path.join(ledgerRoot, 'compatibility-tasks.json'), ledger);

console.log(
  JSON.stringify(
    {
      rulesetFiles: rulesetInventory.length,
      uniqueTypes: uniqueInventory.length,
      saveFields: saveInventory.length,
      endpoints: endpointInventory.length,
      screens: screens.length,
      inputActions: inputActions.length,
      media: media.length,
      editorOperations: editorOperations.length,
      ledgerTasks: ledger.length,
    },
    null,
    2,
  ),
);

function addTasks(ledger, prefix, kind, items, sourceOf, preserveIds = false) {
  items.forEach((item, index) => {
    ledger.push({
      id: preserveIds ? item.id : `${prefix}-${String(index + 1).padStart(4, '0')}`,
      kind,
      source: sourceOf(item),
      status: 'pending',
    });
  });
}

async function writeJson(file, value) {
  await writeFile(file, `${JSON.stringify(value, null, 2)}\n`);
}
