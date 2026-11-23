(function () {
  'use strict';

  // 1. Suppress push notification errors immediately
  window.__DISABLE_PUSH_PROMPT__ = true;
  try {
    localStorage.setItem('haven_push_error_dismissed', 'true');
  } catch(e){}

  if (typeof Notification !== 'undefined') {
    try {
      Notification.requestPermission = () => Promise.resolve('granted');
    } catch (_) {}
  }

  function dismissModals() {
    const pushModal = document.getElementById('push-error-modal');
    if (pushModal) {
      pushModal.style.setProperty('display', 'none', 'important');
      pushModal.style.setProperty('opacity', '0', 'important');
      pushModal.style.setProperty('visibility', 'hidden', 'important');
      const dismissBtn = document.getElementById('push-error-dismiss-btn');
      if (dismissBtn) {
        try { dismissBtn.click(); } catch(e){}
      }
      pushModal.remove();
    }
    const betaModal = document.getElementById('android-beta-modal');
    if (betaModal) {
      betaModal.style.setProperty('display', 'none', 'important');
      betaModal.remove();
    }
    const promoModal = document.getElementById('desktop-promo-modal');
    if (promoModal) {
      promoModal.style.setProperty('display', 'none', 'important');
      promoModal.remove();
    }

    if (window.app && typeof window.app._showPushError === 'function') {
      window.app._showPushError = function() {};
    }
  }

  // 2. Inject CSS styles for Discord Mobile Parity (Focus View)
  function injectStyles() {
    dismissModals();
    if (document.getElementById('haven-mobile-focus-styles')) return;

    const style = document.createElement('style');
    style.id = 'haven-mobile-focus-styles';
    style.textContent = `
      :root {
        --mobile-dock-height: 54px;
        --safe-bottom: env(safe-area-inset-bottom, 0px);
        --safe-top: env(safe-area-inset-top, 0px);
      }

      /* Suppress annoying modals */
      #push-error-modal, #android-beta-modal, #desktop-promo-modal,
      .android-beta-promo, .desktop-promo {
        display: none !important;
        opacity: 0 !important;
        pointer-events: none !important;
        visibility: hidden !important;
      }

      /* Hide desktop elements */
      #titlebar, .window-controls, .desktop-decorations, .sidebar-resize-handle {
        display: none !important;
      }

      /* Top Channel Header: Discord Mobile style */
      .channel-header, #chat-header, .chat-header {
        height: 48px !important;
        min-height: 48px !important;
        padding: 0 10px !important;
        background: #2b2d31 !important;
        border-bottom: 1px solid rgba(255, 255, 255, 0.08) !important;
        display: flex !important;
        align-items: center !important;
        justify-content: space-between !important;
      }

      /* Back button '<' replacing hamburger */
      #mobile-menu-btn {
        display: flex !important;
        align-items: center !important;
        justify-content: center !important;
        width: 36px !important;
        height: 36px !important;
        font-size: 26px !important;
        font-weight: 700 !important;
        color: #949ba4 !important;
        background: transparent !important;
        border: none !important;
        cursor: pointer !important;
        line-height: 1 !important;
        padding: 0 !important;
        margin-right: 4px !important;
      }

      #mobile-menu-btn svg {
        display: none !important;
      }

      /* Hide mobile server hexagon button in chat header to match Discord */
      .channel-header #mobile-server-dropdown,
      .channel-header #mobile-server-btn {
        display: none !important;
      }

      #channel-header-name {
        font-size: 15.5px !important;
        font-weight: 700 !important;
        color: #f2f3f5 !important;
      }

      /* Members button on top right */
      #mobile-users-btn {
        display: flex !important;
        align-items: center !important;
        justify-content: center !important;
        width: 36px !important;
        height: 36px !important;
        color: #949ba4 !important;
        background: transparent !important;
        border: none !important;
      }

      /* Message Area: padding for floating composer & dock */
      #chat, #messages-wrap, #messages, .messages {
        padding-bottom: calc(var(--mobile-dock-height) + var(--safe-bottom) + 70px) !important;
      }

      /* Discord Mobile Floating Pill Composer */
      #message-input-area, .message-input-area {
        position: fixed !important;
        left: 8px !important;
        right: 8px !important;
        bottom: calc(var(--mobile-dock-height) + var(--safe-bottom) + 6px) !important;
        background: #383a40 !important;
        border-radius: 22px !important;
        border: 1px solid rgba(255, 255, 255, 0.08) !important;
        padding: 3px 6px !important;
        display: flex !important;
        flex-direction: row !important;
        align-items: center !important;
        gap: 4px !important;
        z-index: 99995 !important;
        box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4) !important;
        box-sizing: border-box !important;
      }

      /* Single Attachment (+) Pill Button */
      #upload-btn, .btn-upload {
        order: 1 !important;
        display: flex !important;
        align-items: center !important;
        justify-content: center !important;
        width: 32px !important;
        height: 32px !important;
        min-width: 32px !important;
        border-radius: 50% !important;
        background: #4e5058 !important;
        color: #dbdee1 !important;
        border: none !important;
        cursor: pointer !important;
        flex-shrink: 0 !important;
        padding: 0 !important;
        margin: 0 !important;
      }

      #upload-btn svg, #upload-btn .upload-icon-clippy {
        display: none !important;
      }

      #upload-btn::before {
        content: '+' !important;
        display: block !important;
        font-size: 22px !important;
        font-weight: 600 !important;
        line-height: 1 !important;
        color: #dbdee1 !important;
        margin-top: -2px !important;
      }

      /* Text Input */
      #message-input, .message-input {
        order: 2 !important;
        flex: 1 1 auto !important;
        min-width: 0 !important;
        display: block !important;
        background: transparent !important;
        border: none !important;
        color: #f2f3f5 !important;
        font-size: 14.5px !important;
        padding: 6px 6px !important;
        outline: none !important;
        resize: none !important;
        max-height: 100px !important;
        line-height: 1.35 !important;
        margin: 0 !important;
      }

      /* Emoji Button */
      #emoji-btn, .btn-emoji {
        order: 3 !important;
        display: flex !important;
        align-items: center !important;
        justify-content: center !important;
        width: 32px !important;
        height: 32px !important;
        min-width: 32px !important;
        background: transparent !important;
        color: #b5bac1 !important;
        border: none !important;
        cursor: pointer !important;
        flex-shrink: 0 !important;
        padding: 0 !important;
        margin: 0 !important;
      }

      #emoji-btn svg {
        width: 22px !important;
        height: 22px !important;
      }

      /* Send Button */
      #send-btn, .btn-send {
        order: 4 !important;
        background: #5865F2 !important;
        border-radius: 50% !important;
        width: 32px !important;
        height: 32px !important;
        min-width: 32px !important;
        display: flex !important;
        align-items: center !important;
        justify-content: center !important;
        border: none !important;
        flex-shrink: 0 !important;
        color: #ffffff !important;
        cursor: pointer !important;
        padding: 0 !important;
        margin: 0 0 0 2px !important;
      }

      #send-btn svg {
        width: 16px !important;
        height: 16px !important;
        margin-left: 2px !important;
      }

      /* Hide ALL clutter buttons & duplicate plus */
      #composer-plus-btn,
      .btn-composer-plus,
      #gif-btn,
      .btn-gif,
      #poll-btn,
      .btn-poll,
      #time-btn,
      .btn-time,
      #format-btn,
      .btn-format,
      #voice-btn,
      .btn-voice-msg,
      #self-destruct-btn,
      .btn-self-destruct,
      #silent-btn,
      .btn-silent,
      #highlight-btn,
      .btn-highlight,
      #burn-btn,
      .btn-burn,
      #sb-sidebar-toggle-btn,
      .sb-sidebar-toggle-btn,
      .pip-input-resizer,
      .input-actions-divider,
      .braid-more-wrap,
      #braid-more-btn,
      #braid-theme-btn,
      #braid-classic-btn,
      .braid-bar-btn,
      .sidebar-bottom-bar,
      #braid-return-pill {
        display: none !important;
        visibility: hidden !important;
        width: 0 !important;
        height: 0 !important;
        opacity: 0 !important;
        pointer-events: none !important;
      }

      .input-actions-box {
        display: contents !important;
      }

      /* User Bar at the Bottom */
      .sidebar-bottom {
        margin-top: auto !important;
        border-top: none !important;
      }

      .user-bar {
        display: flex !important;
        align-items: center !important;
        background: #232428 !important;
        padding: 6px 10px !important;
        margin: 0 !important;
        border-top: 1px solid rgba(255, 255, 255, 0.06) !important;
        height: 52px !important;
        box-sizing: border-box !important;
        gap: 6px !important;
      }

      /* Slide-out Sidebar Drawer */
      .sidebar {
        background: #2b2d31 !important;
        border-top-right-radius: 16px !important;
        border-bottom-right-radius: 16px !important;
        width: 82vw !important;
        max-width: 320px !important;
        padding-bottom: calc(var(--mobile-dock-height) + var(--safe-bottom) + 16px) !important;
        box-shadow: 4px 0 24px rgba(0, 0, 0, 0.5) !important;
      }

      .channel-item {
        border-radius: 8px !important;
        margin: 2px 8px !important;
        padding: 7px 10px !important;
        cursor: pointer !important;
      }

      .channel-item.active {
        background: rgba(255, 255, 255, 0.12) !important;
        color: #ffffff !important;
        font-weight: 600 !important;
      }

      /* Focus View Bottom Navigation Dock */
      #haven-mobile-focus-dock {
        position: fixed;
        bottom: 0;
        left: 0;
        right: 0;
        height: calc(var(--mobile-dock-height) + var(--safe-bottom));
        padding-bottom: var(--safe-bottom);
        background: #1e1f22;
        border-top: 1px solid rgba(255, 255, 255, 0.08);
        display: flex;
        align-items: center;
        justify-content: space-around;
        z-index: 99999;
        user-select: none;
        box-shadow: 0 -2px 12px rgba(0, 0, 0, 0.4);
      }

      .focus-dock-tab {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        flex: 1;
        height: 100%;
        color: #949ba4;
        font-size: 10.5px;
        font-weight: 500;
        text-decoration: none;
        cursor: pointer;
        transition: color 0.15s ease, transform 0.15s ease;
        gap: 3px;
        -webkit-tap-highlight-color: transparent;
      }

      .focus-dock-tab svg {
        width: 22px;
        height: 22px;
        fill: currentColor;
        transition: transform 0.15s ease;
      }

      .focus-dock-tab.active {
        color: #5865F2;
        font-weight: 700;
      }

      .focus-dock-tab.active svg {
        transform: scale(1.1);
      }

      .focus-dock-avatar {
        width: 24px;
        height: 24px;
        border-radius: 50%;
        object-fit: cover;
        border: 1.5px solid currentColor;
      }
    `;
    (document.head || document.documentElement).appendChild(style);
    dismissModals();
  }

  // 3. Double click on channel to directly join voice!
  function triggerVoiceJoin(code) {
    if (!code) return;
    if (window.app && typeof window.app.switchChannel === 'function') {
      if (window.app.currentChannel === code) {
        if (typeof window.app._joinVoice === 'function') window.app._joinVoice();
        else document.getElementById('voice-join-btn')?.click();
      } else {
        const p = window.app.switchChannel(code);
        const join = () => {
          if (typeof window.app._joinVoice === 'function') window.app._joinVoice();
          else document.getElementById('voice-join-btn')?.click();
        };
        if (p && typeof p.then === 'function') {
          p.then(() => setTimeout(join, 150));
        } else {
          setTimeout(join, 150);
        }
      }
    } else {
      document.getElementById('voice-join-btn')?.click();
    }
  }

  // Double click listener (Desktop & Mouse)
  document.addEventListener('dblclick', (e) => {
    const item = e.target.closest('.channel-item');
    if (!item) return;
    if (e.target.closest('.channel-more-btn') || e.target.closest('.channel-collapse-arrow')) return;

    const code = item.dataset.code;
    if (code) {
      e.preventDefault();
      e.stopPropagation();
      triggerVoiceJoin(code);
    }
  }, true);

  // Rapid double-tap tracking (< 450ms) for Touch / Mobile
  let lastTapTime = 0;
  let lastTapCode = null;

  function checkDoubleTap(item, e) {
    if (!item) return;
    if (e.target.closest('.channel-more-btn') || e.target.closest('.channel-collapse-arrow')) return;

    const code = item.dataset.code;
    if (!code) return;

    const now = Date.now();
    if (lastTapCode === code && (now - lastTapTime) < 450) {
      e.preventDefault();
      e.stopPropagation();
      triggerVoiceJoin(code);
      lastTapTime = 0;
      lastTapCode = null;
      return true;
    }

    lastTapTime = now;
    lastTapCode = code;
    return false;
  }

  document.addEventListener('click', (e) => {
    const item = e.target.closest('.channel-item');
    if (item) checkDoubleTap(item, e);
  }, true);

  document.addEventListener('touchend', (e) => {
    const item = e.target.closest('.channel-item');
    if (item) checkDoubleTap(item, e);
  }, { passive: false });

  // 4. Inject Bottom Dock HTML and Event Listeners
  function injectDock() {
    dismissModals();
    if (document.getElementById('haven-mobile-focus-dock')) return;
    if (!document.body) return;

    const dock = document.createElement('nav');
    dock.id = 'haven-mobile-focus-dock';
    dock.innerHTML = `
      <div class="focus-dock-tab active" data-tab="server">
        <svg viewBox="0 0 24 24"><path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z"/></svg>
        <span>Server</span>
      </div>
      <div class="focus-dock-tab" data-tab="dms">
        <svg viewBox="0 0 24 24"><path d="M20 2H4c-1.1 0-2 .9-2 2v18l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-2 12H6v-2h12v2zm0-3H6V9h12v2zm0-3H6V6h12v2z"/></svg>
        <span>DMs</span>
      </div>
      <div class="focus-dock-tab" data-tab="activity">
        <svg viewBox="0 0 24 24"><path d="M12 22c1.1 0 2-.9 2-2h-4c0 1.1.9 2 2 2zm6-6v-5c0-3.07-1.63-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.64 5.36 6 7.92 6 11v5l-2 2v1h16v-1l-2-2zm-2 1H8v-6c0-2.48 1.51-4.5 4-4.5s4 2.02 4 4.5v6z"/></svg>
        <span>Activity</span>
      </div>
      <div class="focus-dock-tab" data-tab="you">
        <svg viewBox="0 0 24 24"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/></svg>
        <span>You</span>
      </div>
    `;
    document.body.appendChild(dock);

    // Switch Tabs
    const tabs = dock.querySelectorAll('.focus-dock-tab');
    tabs.forEach(tab => {
      tab.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        tabs.forEach(t => t.classList.remove('active'));
        tab.classList.add('active');

        const tabName = tab.dataset.tab;
        if (tabName === 'server') {
          const menuBtn = document.getElementById('mobile-menu-btn');
          if (menuBtn) menuBtn.click();
        } else if (tabName === 'dms') {
          const homeServer = document.getElementById('home-server');
          if (homeServer) homeServer.click();
          const dmToggle = document.getElementById('dm-toggle-header');
          if (dmToggle) dmToggle.click();
        } else if (tabName === 'activity') {
          const mentionsBtn = document.getElementById('mentions-btn');
          if (mentionsBtn) mentionsBtn.click();
        } else if (tabName === 'you') {
          const settingsBtn = document.getElementById('mobile-settings-btn') || document.getElementById('open-settings-btn');
          if (settingsBtn) settingsBtn.click();
          else {
            const userAvatar = document.getElementById('user-bar-avatar');
            if (userAvatar) userAvatar.click();
          }
        }
      });
    });

    // Relocate user bar to bottom of drawer & clean composer
    document.querySelectorAll(
      '.braid-more-wrap, #braid-more-btn, #braid-theme-btn, #braid-classic-btn, .sidebar-bottom-bar, #composer-plus-btn, #gif-btn, #poll-btn, #time-btn, #format-btn, #voice-btn, #self-destruct-btn, #silent-btn, #highlight-btn, #burn-btn, #sb-sidebar-toggle-btn'
    ).forEach(el => {
      el.style.setProperty('display', 'none', 'important');
      el.remove();
    });

    const sidebar = document.querySelector('.sidebar');
    const userBar = document.querySelector('.user-bar');
    const sidebarBottom = document.querySelector('.sidebar-bottom');
    if (sidebar && userBar && sidebarBottom && userBar.parentElement !== sidebarBottom) {
      sidebarBottom.prepend(userBar);
      const muteBtn = document.getElementById('voice-mute-btn');
      const deafenBtn = document.getElementById('voice-deafen-btn');
      const settingsBtn = document.getElementById('mobile-settings-btn') || document.getElementById('open-settings-btn');
      if (muteBtn && muteBtn.parentElement !== userBar) { muteBtn.style.display = 'flex'; userBar.appendChild(muteBtn); }
      if (deafenBtn && deafenBtn.parentElement !== userBar) { deafenBtn.style.display = 'flex'; userBar.appendChild(deafenBtn); }
      if (settingsBtn && settingsBtn.parentElement !== userBar) { settingsBtn.style.display = 'flex'; userBar.appendChild(settingsBtn); }
    }

    // Replace hamburger menu icon text with back arrow '<' in chat view
    const menuBtn = document.getElementById('mobile-menu-btn');
    if (menuBtn) {
      menuBtn.innerHTML = '‹';
    }

    // Set Vietnamese placeholder for message input
    const msgInput = document.getElementById('message-input');
    if (msgInput) {
      msgInput.placeholder = 'Nhập tin nhắn...';
    }

    // Update avatar in "You" tab
    const checkAvatar = setInterval(() => {
      const userImg = document.querySelector('#user-bar-avatar img, #user-bar-face');
      if (userImg && userImg.src) {
        const youTab = dock.querySelector('[data-tab="you"]');
        if (youTab) {
          youTab.innerHTML = `
            <img src="${userImg.src}" class="focus-dock-avatar" />
            <span>You</span>
          `;
        }
        clearInterval(checkAvatar);
      }
    }, 1000);
    setTimeout(() => clearInterval(checkAvatar), 15000);
  }

  // Run immediately and also hook to DOM changes
  injectStyles();
  injectDock();

  // Re-run whenever DOM updates (SPA page navigation)
  const observer = new MutationObserver(() => {
    injectStyles();
    injectDock();
  });
  if (document.body) {
    observer.observe(document.body, { childList: true, subtree: true });
  } else {
    document.addEventListener('DOMContentLoaded', () => {
      injectStyles();
      injectDock();
      observer.observe(document.body, { childList: true, subtree: true });
    });
  }
})();
