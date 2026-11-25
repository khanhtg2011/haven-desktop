const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const rootDir = path.resolve(__dirname, '..');

test('package.json is isolated to the Electron desktop client', () => {
  const pkg = JSON.parse(fs.readFileSync(path.join(rootDir, 'package.json'), 'utf8'));
  assert.equal(pkg.name, 'haven-desktop');
  assert.equal(pkg.main, 'src/main/main.js');
  assert.ok(pkg.devDependencies?.electron, 'electron devDependency must be present');
  assert.equal(pkg.devDependencies?.['@tauri-apps/cli'], undefined);

  const scriptNames = Object.keys(pkg.scripts || {});
  assert.equal(
    scriptNames.some((name) => name.includes('tauri')),
    false,
    'package.json scripts must not contain Tauri commands'
  );
});

test('electron-builder.yml and CI workflow target Electron builds for haven-desktop', () => {
  const builderYml = fs.readFileSync(path.join(rootDir, 'electron-builder.yml'), 'utf8');
  assert.match(builderYml, /owner:\s*khanhtg2011/);
  assert.match(builderYml, /repo:\s*haven-desktop/);

  const buildWorkflow = fs.readFileSync(
    path.join(rootDir, '.github/workflows/build.yml'),
    'utf8'
  );
  assert.match(buildWorkflow, /-\s*main\b/);
  assert.match(buildWorkflow, /electron-builder/);
  assert.equal(
    fs.existsSync(path.join(rootDir, '.github/workflows/build-tauri.yml')),
    false,
    'build-tauri.yml must not exist in haven-desktop'
  );
  assert.equal(
    fs.existsSync(path.join(rootDir, 'src/tauri-bridge')),
    false,
    'src/tauri-bridge must not exist in haven-desktop'
  );
});
