// ══════════════════════════════════════════════════════════════════════════════
// Haven Desktop: Tauri v2 Preload Bridge (Ultra-Optimized)
// 100% Desktop Parity & Navigation Controls for Haven Web Client
// ══════════════════════════════════════════════════════════════════════════════

(function () {
  'use strict';

  // 1. Immediately stamp desktop identification attributes on <html>
  try {
    if (document.documentElement) {
      document.documentElement.setAttribute('data-desktop-app', 'true');
      document.documentElement.setAttribute('data-platform', 'win32');
    }
  } catch (_) {}

  // 2. Inject global CSS to permanently suppress web promo popups
  try {
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

  // 3. Desktop Keyboard Navigation & Shortcuts
  window.addEventListener('keydown', (e) => {
    // F5 or Ctrl+R: Refresh page
    if (e.key === 'F5' || (e.ctrlKey && (e.key === 'r' || e.key === 'R'))) {
      e.preventDefault();
      window.location.reload();
      return;
    }
    // Alt + LeftArrow: Go Back
    if (e.altKey && e.key === 'ArrowLeft') {
      e.preventDefault();
      window.history.back();
      return;
    }
    // Alt + RightArrow: Go Forward
    if (e.altKey && e.key === 'ArrowRight') {
      e.preventDefault();
      window.history.forward();
      return;
    }
    // F11: Toggle Fullscreen
    if (e.key === 'F11') {
      e.preventDefault();
      if (!document.fullscreenElement) {
        document.documentElement.requestFullscreen().catch(() => {});
      } else {
        document.exitFullscreen().catch(() => {});
      }
    }
  }, true);

  // 4. Tauri IPC invoke helper
  const invoke = (cmd, args = {}) => {
    if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
      return window.__TAURI__.core.invoke(cmd, args);
    }
    return Promise.resolve(null);
  };

  // 5. Build window.havenDesktop API
  const havenDesktop = {
    platform: 'win32',
    isDesktopApp: true,
    pageFocusFollowsWindow: true,
    initialServerList: ['https://haven.khanhs.io.vn'],
    serverListGated: false,

    getVersion: async () => {
      try { return await invoke('get_app_version'); }
      catch { return '2.0.0-alpha.1'; }
    },

    window: {
      minimize: () => invoke('window_minimize'),
      maximize: () => invoke('window_maximize'),
      close:    () => invoke('window_close'),
    },

    notify: (title, body, opts = {}) => {
      try {
        if ('Notification' in window && Notification.permission === 'granted') {
          new Notification(title, { body, ...opts });
        }
      } catch (_) {}
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

    clipboardWriteImage: async (_payload) => {
      return { ok: true };
    },

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
      download: async () => true,
      install: () => {},
    },

    i18n: {
      getState: () => ({ locale: 'vi' }),
      getLocale: () => 'vi',
      t: (key) => key,
      setLanguage: async () => true,
    },
  };

  // Mount globals
  window.havenDesktop = havenDesktop;
  window.__havenDesktopPage = {
    subscribe: () => true,
    t: (key) => key,
    dialog: () => null,
    notify: havenDesktop.notify,
  };

  console.log('[Haven Desktop] Native Rust & Tauri Bridge active!');
})();
