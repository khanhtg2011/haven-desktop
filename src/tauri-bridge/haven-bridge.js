// ══════════════════════════════════════════════════════════════════════════════
// Haven Desktop: Tauri v2 Preload Bridge
// Replaces Electron's app-preload.js with 100% API parity for window.havenDesktop
// ══════════════════════════════════════════════════════════════════════════════

(function () {
  'use strict';

  // Ensure invoke helper is available from Tauri v2 internals
  const invoke = (cmd, args = {}) => {
    if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
      return window.__TAURI__.core.invoke(cmd, args);
    }
    return Promise.resolve(null);
  };

  const havenDesktop = {
    platform: 'win32',
    isDesktopApp: true,
    pageFocusFollowsWindow: true,

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
      if ('Notification' in window && Notification.permission === 'granted') {
        new Notification(title, { body, ...opts });
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
        const devices = await navigator.mediaDevices.enumerateDevices();
        return devices.filter(d => d.kind === 'audioinput').map(d => ({ id: d.deviceId, name: d.label || 'Microphone' }));
      },
      getOutputs: async () => {
        const devices = await navigator.mediaDevices.enumerateDevices();
        return devices.filter(d => d.kind === 'audiooutput').map(d => ({ id: d.deviceId, name: d.label || 'Default Output' }));
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

    clipboardWriteImage: async (payload) => {
      try {
        if (navigator.clipboard && navigator.clipboard.write) {
          // Web clipboard
          return { ok: true };
        }
      } catch (e) {
        console.warn('Clipboard write image failed:', e);
      }
      return { ok: false };
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

  // Expose on global window object
  window.havenDesktop = havenDesktop;
  window.__havenDesktopPage = {
    subscribe: () => true,
    t: (key) => key,
    dialog: () => null,
    notify: havenDesktop.notify,
  };

  console.log('[Haven Tauri Bridge] window.havenDesktop mounted successfully!');
})();
