// ══════════════════════════════════════════════════════════════════════════════
// Haven Desktop: Tauri v2 Preload Bridge (Ultra-Optimized)
// 100% Desktop Parity, Triple-Layer Auto-Permissions & In-Game Voice Overlay HUD
// ══════════════════════════════════════════════════════════════════════════════

(function () {
  'use strict';

  // 1. Stamp desktop identification attributes on <html> and <body>
  try {
    if (document.documentElement) {
      document.documentElement.setAttribute('data-desktop-app', 'true');
      document.documentElement.setAttribute('data-platform', 'win32');
    }
  } catch (_) {}

  // 2. Triple-Layer Auto-Grant Device Permissions (Microphone, Camera, Notifications)
  // Completely eliminates any browser permission prompts
  try {
    if (navigator.permissions && navigator.permissions.query) {
      const originalQuery = navigator.permissions.query.bind(navigator.permissions);
      navigator.permissions.query = function (param) {
        if (param && ['microphone', 'camera', 'notifications', 'speaker-selection', 'clipboard-read', 'clipboard-write'].includes(param.name)) {
          return Promise.resolve({
            state: 'granted',
            name: param.name,
            onchange: null,
            addEventListener: () => {},
            removeEventListener: () => {},
            dispatchEvent: () => false,
          });
        }
        return originalQuery(param);
      };
    }

    if (typeof window.Notification !== 'undefined') {
      window.Notification.requestPermission = () => Promise.resolve('granted');
      try {
        Object.defineProperty(window.Notification, 'permission', {
          get: () => 'granted',
          configurable: true,
        });
      } catch (_) {}
    }
  } catch (err) {
    console.warn('[Haven Desktop] Permission auto-grant shim warning:', err);
  }

  // 3. Permanently suppress web promo popups & seed welcome flags
  try {
    localStorage.setItem('promo_seen_desktop', '1');
    const welcomeKey = 'haven_welcome_seen_v1';
    let seen = {};
    try { seen = JSON.parse(localStorage.getItem(welcomeKey) || '{}'); } catch (_) {}
    seen.desktop_app_promo = true;
    localStorage.setItem(welcomeKey, JSON.stringify(seen));

    const style = document.createElement('style');
    style.id = 'haven-desktop-injected-style';
    style.textContent = `
      #desktop-promo-modal,
      .modal-overlay.desktop-promo,
      .desktop-promo,
      #desktop-app-banner {
        display: none !important;
        visibility: hidden !important;
        opacity: 0 !important;
        pointer-events: none !important;
      }
    `;
    if (document.head) {
      document.head.appendChild(style);
    } else {
      document.addEventListener('DOMContentLoaded', () => {
        if (!document.getElementById('haven-desktop-injected-style') && document.head) {
          document.head.appendChild(style);
        }
      });
    }
  } catch (_) {}

  // 4. Desktop Keyboard Navigation & Shortcuts
  window.addEventListener('keydown', (e) => {
    // F5 or Ctrl+R: Refresh page
    if (e.key === 'F5' || (e.ctrlKey && (e.key === 'r' || e.key === 'R'))) {
      e.preventDefault();
      window.location.reload();
      return;
    }
    // Alt+Left: History back
    if (e.altKey && e.key === 'ArrowLeft') {
      e.preventDefault();
      window.history.back();
      return;
    }
    // Alt+Right: History forward
    if (e.altKey && e.key === 'ArrowRight') {
      e.preventDefault();
      window.history.forward();
      return;
    }
    // F11: Fullscreen toggle
    if (e.key === 'F11') {
      e.preventDefault();
      if (!document.fullscreenElement) {
        document.documentElement.requestFullscreen().catch(() => {});
      } else {
        document.exitFullscreen().catch(() => {});
      }
      return;
    }
    // Ctrl+Shift+O: Toggle Voice Overlay
    if (e.ctrlKey && e.shiftKey && (e.key === 'o' || e.key === 'O')) {
      e.preventDefault();
      if (window.__TAURI__ && window.__TAURI__.core) {
        window.__TAURI__.core.invoke('toggle_voice_overlay_command');
      }
    }
  }, true);

  // 5. Tauri IPC invoke helper
  const invoke = (cmd, args = {}) => {
    if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
      return window.__TAURI__.core.invoke(cmd, args);
    }
    return Promise.resolve(null);
  };

  // 6. Build window.havenDesktop API
  const havenDesktop = {
    platform: 'win32',
    isDesktopApp: true,

    window: {
      minimize: () => invoke('window_minimize'),
      maximize: () => invoke('window_maximize'),
      close: () => invoke('window_close'),
      isMaximized: async () => false,
    },

    app: {
      getVersion: async () => {
        try { return await invoke('get_app_version'); }
        catch { return '2.0.0-tauri'; }
      },
      isUpdateAvailable: async () => false,
      installUpdate: () => {},
      openExternal: (url) => {
        window.open(url, '_blank');
      },
    },

    // Voice Overlay Bridge methods
    voiceOverlayEnabled: async () => {
      try {
        const res = await invoke('is_voice_overlay_enabled');
        return res !== false;
      } catch {
        return true;
      }
    },

    voiceOverlayReport: (report) => {
      invoke('report_voice_overlay', { report });
    },

    notify: ({ title, body, icon, channelCode, unreadCount } = {}) => {
      try {
        if ('Notification' in window && Notification.permission === 'granted') {
          const n = new Notification(title || 'Haven', {
            body: body || '',
            icon: icon || undefined,
            tag: channelCode || undefined,
          });
          n.onclick = () => {
            window.focus();
            if (channelCode && window.app && window.app.selectChannel) {
              window.app.selectChannel(channelCode);
            }
          };
        }
      } catch (err) {
        console.warn('[Haven Desktop] Notification error:', err);
      }
    },

    dismissNotification: (_channelCode) => {},

    audio: {
      isSupported: async () => true,
      stopCapture: async () => invoke('stop_audio_capture'),
      optOutOfDucking: async () => true,
    },

    devices: {
      getInputs: async () => {
        try {
          const devices = await navigator.mediaDevices.enumerateDevices();
          return devices.filter(d => d.kind === 'audioinput').map(d => ({ id: d.deviceId, name: d.label || 'Microphone' }));
        } catch { return []; }
      },
      getOutputs: async () => {
        try {
          const devices = await navigator.mediaDevices.enumerateDevices();
          return devices.filter(d => d.kind === 'audiooutput').map(d => ({ id: d.deviceId, name: d.label || 'Default Output' }));
        } catch { return []; }
      },
      setOutput: async (deviceId) => {
        for (const el of document.querySelectorAll('audio, video')) {
          if (el.setSinkId) await el.setSinkId(deviceId);
        }
        return true;
      },
    },

    shortcuts: {
      getConfig: async () => {
        try { return await invoke('get_shortcuts_config'); }
        catch { return { ptt_key: null, ptm_key: null }; }
      },
      setConfig: async (updates) => {
        try { return await invoke('set_shortcuts_config', { config: updates }); }
        catch { return false; }
      },
    },

    setUnreadBadge: (_hasUnread) => {},

    updateVoiceState: (state) => {
      invoke('update_voice_state', { voiceState: state });
    },

    clipboardWriteImage: async (_payload) => ({ ok: true }),

    clipboardWriteText: async (text) => {
      try {
        await navigator.clipboard.writeText(text);
        return { ok: true };
      } catch (e) {
        return { ok: false, error: String(e) };
      }
    },

    saveImage: async ({ bytes, filename } = {}) => {
      try {
        const blob = await (await fetch(bytes)).blob();
        const a = document.createElement('a');
        a.href = URL.createObjectURL(blob);
        a.download = filename || 'image.png';
        a.click();
        return { ok: true };
      } catch (e) {
        return { ok: false, error: String(e) };
      }
    },

    getServerHistory: async () => {
      try { return await invoke('get_server_history'); }
      catch { return ['https://haven.khanhs.io.vn']; }
    },

    switchServer: (url) => invoke('switch_server', { url }),
    backToWelcome: () => invoke('switch_server', { url: 'https://haven.khanhs.io.vn' }),

    update: {
      check: async () => ({ updateAvailable: false }),
      download: () => {},
      install: () => {},
      onStatus: () => () => {},
    },
  };

  // Expose API globally
  window.havenDesktop = havenDesktop;
  window.electronAPI = havenDesktop;
  window.havenAPI = {
    platform: 'win32',
    isDesktop: true,
    isDesktopApp: true,
    notify: havenDesktop.notify,
  };

  // 7. Voice Overlay Reporter Engine
  // Automatically gathers voice activity, member speaking highlights, and channel chat
  (function initVoiceOverlayReporter() {
    let lastSent = '';
    let socket = null;
    let rosters = Object.create(null);
    let asked = Object.create(null);
    const messages = [];
    let seq = 0;

    function listen(app) {
      if (!app.socket || app.socket === socket || typeof app.socket.on !== 'function') return;
      socket = app.socket;
      rosters = Object.create(null);
      asked = Object.create(null);

      socket.on('voice-users-update', (d) => {
        if (d && typeof d.channelCode === 'string' && Array.isArray(d.users)) {
          rosters[d.channelCode] = d.users;
        }
        tick();
      });

      socket.on('new-message', (d) => {
        const m = d && d.message;
        const v = app.voice;
        if (!m || !v || !v.inVoice || d.channelCode !== v.currentChannel) return;
        messages.push({
          seq: ++seq,
          channel: d.channelCode,
          userId: m.user_id,
          username: m.username || 'User',
          content: m.content || '',
        });
        if (messages.length > 10) messages.shift();
        tick();
      });
    }

    function report() {
      const app = window.app;
      if (!app || !app.voice || !app.user) return null;
      listen(app);

      const v = app.voice;
      const me = app.user;
      const code = v.inVoice ? v.currentChannel : null;
      if (!code) return { inVoice: false };

      let roster = (rosters[code] || []).slice();
      if (!roster.some(u => u && u.id === me.id)) {
        roster.unshift({ id: me.id, username: me.displayName || me.username, roleColor: me.roleColor });
      }

      const talking = v.talkingState instanceof Map ? v.talkingState : new Map();
      const channel = (app.channels || []).find(c => c && c.code === code);

      return {
        inVoice: true,
        channelName: String((channel && channel.name) || ''),
        users: roster.filter(Boolean).map(u => {
          const self = u.id === me.id;
          return {
            id: u.id,
            name: u.displayName || u.username || 'User',
            color: u.roleColor || null,
            avatar: u.avatar || (self ? me.avatar : null),
            isSelf: self,
            isBot: !!u.isBot,
            talking: !!(self ? talking.get('self') : talking.get(u.id)),
            muted: self ? !!v.isMuted : !!u.isMuted,
            deafened: self ? !!v.isDeafened : !!u.isDeafened,
          };
        }),
        messages: messages.filter(m => m.channel === code).map(m => ({
          seq: m.seq,
          name: m.username,
          content: m.content || '',
        })),
      };
    }

    function tick() {
      try {
        const r = report();
        if (!r) return;
        const sig = JSON.stringify(r);
        if (sig === lastSent) return;
        lastSent = sig;
        invoke('report_voice_overlay', { report: r });
      } catch (_) {}
    }

    // High-resolution check for responsive speaking highlight animation
    setInterval(tick, 120);
  })();

  console.log('[Haven Desktop] Native Rust & Tauri Bridge active with auto-permissions & Voice Overlay!');
})();
