/**
 * Haven Discord UI & Layout Integrity Test Suite
 * Run with: node tests/discord_ui_test.js
 */

const https = require('https');
const assert = require('assert');

const SERVER_HOST = 'haven.khanhs.io.vn';

function fetchUrl(path) {
  return new Promise((resolve, reject) => {
    https.get({
      hostname: SERVER_HOST,
      path: path,
      headers: { 'User-Agent': 'Haven-Automated-Auditor/1.0' }
    }, (res) => {
      let data = '';
      res.on('data', chunk => data += chunk);
      res.on('end', () => resolve({ statusCode: res.statusCode, data }));
    }).on('error', reject);
  });
}

async function runTests() {
  console.log('🧪 Starting Haven Discord UI & Mobile Drawer Tests...\n');
  let passed = 0;
  let total = 0;

  function test(name, fn) {
    total++;
    try {
      fn();
      console.log(`  ✅ [PASS] ${name}`);
      passed++;
    } catch (err) {
      console.error(`  ❌ [FAIL] ${name}: ${err.message}`);
    }
  }

  // 1. Fetch app.html
  console.log('📡 Fetching https://' + SERVER_HOST + '/app ...');
  const appRes = await fetchUrl('/app');
  assert.strictEqual(appRes.statusCode, 200, 'Expected HTTP 200 from /app');
  const html = appRes.data;

  // Test 1: Single user-bar inside sidebar-bottom
  test('DOM: Exactly one user-bar exists and is inside sidebar-bottom', () => {
    const userBarMatches = html.match(/class=["'][^"']*\buser-bar\b(?!-)[^"']*["']/g) || [];
    assert.strictEqual(userBarMatches.length, 1, `Found ${userBarMatches.length} user-bar instances, expected exactly 1`);
    
    const sidebarBottomIndex = html.indexOf('class="sidebar-bottom"');
    const userBarIndex = html.indexOf('class="user-bar"');
    const asideCloseIndex = html.indexOf('</aside>', sidebarBottomIndex);
    assert.ok(sidebarBottomIndex !== -1, 'sidebar-bottom element exists');
    assert.ok(userBarIndex > sidebarBottomIndex && userBarIndex < asideCloseIndex, 'user-bar must be inside sidebar-bottom');
  });

  // Test 2: sidebar-header integrity
  test('DOM: sidebar-header contains only brand and close button without user bar', () => {
    const headerStart = html.indexOf('class="sidebar-header"');
    const headerEnd = html.indexOf('</div>', headerStart);
    const headerChunk = html.substring(headerStart, headerStart + 600);
    assert.ok(!headerChunk.includes('user-bar'), 'sidebar-header must not contain user-bar');
    assert.ok(!headerChunk.includes('id="logout-btn"'), 'sidebar-header must not contain logout button');
  });

  // Test 3: Tag Depth Balance within aside.sidebar
  test('DOM: aside.sidebar tag nesting is perfectly balanced (depth = 0 at </aside>)', () => {
    const asideStart = html.indexOf('<aside class="sidebar"');
    const asideEnd = html.indexOf('</aside>', asideStart);
    assert.ok(asideStart !== -1 && asideEnd !== -1, 'aside.sidebar tags exist');
    const sidebarHtml = html.substring(asideStart, asideEnd);
    
    // Count opening and closing divs
    const openDivs = (sidebarHtml.match(/<div[\s>]/g) || []).length;
    const closeDivs = (sidebarHtml.match(/<\/div>/g) || []).length;
    assert.strictEqual(openDivs, closeDivs, `Div tags mismatch: ${openDivs} open vs ${closeDivs} close`);
  });

  // Test 4: Single upload button and clean composer
  test('Composer: Exactly one upload button with plus icon and placeholder', () => {
    const uploadBtns = html.match(/id=["']upload-btn["']/g) || [];
    assert.strictEqual(uploadBtns.length, 1, `Found ${uploadBtns.length} upload-btn, expected 1`);
    assert.ok(html.includes('Nhập tin nhắn... (/ cho lệnh)'), 'Message placeholder must be Vietnamese');
  });

  // Test 5: Clutter buttons stashed in hidden container
  test('Composer: Clutter buttons are stashed in hidden container', () => {
    assert.ok(html.includes('id="composer-stashed-buttons"'), 'composer-stashed-buttons container must exist');
    assert.ok(html.includes('id="composer-plus-btn"'), 'composer-plus-btn must be preserved in stashed container');
    assert.ok(html.includes('id="gif-btn"'), 'gif-btn preserved in stashed container');
    assert.ok(html.includes('id="poll-btn"'), 'poll-btn preserved in stashed container');
  });

  // Test 6: Braid Menu and legacy bottom bar banished
  test('Layout: Legacy sidebar-bottom-bar is absent from app.html', () => {
    assert.ok(!html.includes('class="sidebar-bottom-bar"'), 'sidebar-bottom-bar must not exist');
    assert.ok(!html.includes('id="braid-more-btn"'), 'braid-more-btn must not exist in HTML');
  });

  // 2. Fetch style.css and discord-theme.css
  console.log('\n📡 Fetching https://' + SERVER_HOST + '/css/style.css?v=5.8.0 ...');
  const styleRes = await fetchUrl('/css/style.css?v=5.8.0');
  test('Styles: style.css is HTTP 200 and imports discord-theme.css with v=5.8.0', () => {
    assert.strictEqual(styleRes.statusCode, 200, 'Expected HTTP 200 from style.css');
    assert.ok(styleRes.data.includes("discord-theme.css?v=5.8.0"), 'style.css must import discord-theme.css?v=5.8.0');
  });

  console.log('📡 Fetching https://' + SERVER_HOST + '/css/discord-theme.css?v=5.8.0 ...');
  const discordThemeRes = await fetchUrl('/css/discord-theme.css?v=5.8.0');
  test('Styles: discord-theme.css is HTTP 200 with Discord color tokens and flex rules', () => {
    assert.strictEqual(discordThemeRes.statusCode, 200, 'Expected HTTP 200 from discord-theme.css');
    assert.ok(discordThemeRes.data.includes('#5865F2'), 'Discord accent #5865F2 must be defined');
    assert.ok(discordThemeRes.data.includes('#313338'), 'Discord bg #313338 must be defined');
    assert.ok(discordThemeRes.data.includes('#2b2d31'), 'Discord sidebar bg #2b2d31 must be defined');
  });

  // 3. Fetch app-channels.js
  console.log('\n📡 Fetching https://' + SERVER_HOST + '/js/modules/app-channels.js?v=5.8.0 ...');
  const channelsRes = await fetchUrl('/js/modules/app-channels.js?v=5.8.0');
  test('Voice: app-channels.js supports double-tap touch and double-click mouse voice join', () => {
    assert.strictEqual(channelsRes.statusCode, 200, 'Expected HTTP 200 from app-channels.js');
    assert.ok(channelsRes.data.includes('_chTouchStart'), 'Must have _chTouchStart touch handler');
    assert.ok(channelsRes.data.includes('_chLastTap'), 'Must have _chLastTap double-tap detection');
    assert.ok(channelsRes.data.includes('this._joinVoice()'), 'Must call _joinVoice() on double action');
  });

  // 4. Fetch app-socket-channels.js
  console.log('📡 Fetching https://' + SERVER_HOST + '/js/modules/app-socket-channels.js?v=5.8.0 ...');
  const socketChRes = await fetchUrl('/js/modules/app-socket-channels.js?v=5.8.0');
  test('Auto-Select: app-socket-channels.js automatically switches to first public channel on load', () => {
    assert.strictEqual(socketChRes.statusCode, 200, 'Expected HTTP 200 from app-socket-channels.js');
    assert.ok(socketChRes.data.includes('autoCh'), 'Must contain autoCh channel auto-selection logic');
    assert.ok(socketChRes.data.includes('this.switchChannel(autoCh.code)'), 'Must switchChannel to autoCh.code');
  });

  // 5. Fetch BraidLayout.plugin.js
  console.log('📡 Fetching https://' + SERVER_HOST + '/plugins/BraidLayout.plugin.js ...');
  const braidRes = await fetchUrl('/plugins/BraidLayout.plugin.js');
  test('Braid: BraidLayout.plugin.js _buildMoreMenu is completely neutralized', () => {
    assert.strictEqual(braidRes.statusCode, 200, 'Expected HTTP 200 from BraidLayout.plugin.js');
    assert.ok(braidRes.data.includes('_buildMoreMenu()'), 'Contains _buildMoreMenu');
    assert.ok(braidRes.data.includes('.remove()'), 'Neutralized menu removes Braid clutter');
  });

  console.log(`\n========================================`);
  console.log(`Test Results: ${passed}/${total} PASSED`);
  console.log(`========================================\n`);

  if (passed !== total) {
    process.exit(1);
  }
}

runTests().catch(err => {
  console.error('Fatal Test Error:', err);
  process.exit(1);
});
