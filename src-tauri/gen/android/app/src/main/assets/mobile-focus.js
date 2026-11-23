(function () {
  'use strict';

  // 1. Suppress push notification errors immediately
  window.__DISABLE_PUSH_PROMPT__ = true;
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
      if (dismissBtn) dismissBtn.click();
    }
    const betaModal = document.getElementById('android-beta-modal');
    if (betaModal) betaModal.style.setProperty('display', 'none', 'important');
    const promoModal = document.getElementById('desktop-promo-modal');
    if (promoModal) promoModal.style.setProperty('display', 'none', 'important');
  }

  // 2. Inject CSS styles for Discord Mobile Parity (Focus View)
  function injectStyles() {
    dismissModals();
    if (document.getElementById('haven-mobile-focus-styles')) return;

    const style = document.createElement('style');
    style.id = 'haven-mobile-focus-styles';
    style.textContent = `
      :root {
        --mobile-dock-height: 58px;
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
        height: 52px !important;
        padding: 0 12px !important;
        background: #17182b !important;
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
        font-size: 28px !important;
        font-weight: 700 !important;
        color: #b5bac1 !important;
        background: transparent !important;
        border: none !important;
        cursor: pointer !important;
        line-height: 1 !important;
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
        font-size: 16px !important;
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
        color: #b5bac1 !important;
        background: transparent !important;
        border: none !important;
      }

      /* Adjust message feed for bottom dock & composer */
      #chat, #messages-wrap, #messages, .messages {
        padding-bottom: calc(var(--mobile-dock-height) + var(--safe-bottom) + 72px) !important;
      }

      /* Composer Bar: Discord Mobile clean pill input */
      #message-input-area, .message-input-area {
        position: fixed !important;
        left: 8px !important;
        right: 8px !important;
        bottom: calc(var(--mobile-dock-height) + var(--safe-bottom) + 6px) !important;
        background: #1e1f35 !important;
        border-radius: 24px !important;
        border: 1px solid rgba(255, 255, 255, 0.08) !important;
        padding: 4px 8px !important;
        display: flex !important;
        align-items: center !important;
        gap: 6px !important;
        z-index: 9998 !important;
        box-shadow: 0 4px 16px rgba(0, 0, 0, 0.35) !important;
      }

      /* Message input box */
      #message-input, .message-input {
        flex: 1 !important;
        display: block !important;
        background: transparent !important;
        border: none !important;
        color: #f2f3f5 !important;
        font-size: 15px !important;
        padding: 8px 6px !important;
        outline: none !important;
        resize: none !important;
        max-height: 120px !important;
        line-height: 1.4 !important;
      }

      /* Attachment (+) button on the left */
      #upload-btn, .btn-upload, #composer-plus-btn, .btn-composer-plus {
        display: flex !important;
        align-items: center !important;
        justify-content: center !important;
        width: 34px !important;
        height: 34px !important;
        border-radius: 50% !important;
        background: #2b2d48 !important;
        color: #b5bac1 !important;
        border: none !important;
        cursor: pointer !important;
        flex-shrink: 0 !important;
        font-size: 20px !important;
        font-weight: 700 !important;
      }

      /* Emoji smile button on the right */
      #emoji-btn, .btn-emoji {
        display: flex !important;
        align-items: center !important;
        justify-content: center !important;
        width: 34px !important;
        height: 34px !important;
        background: transparent !important;
        color: #b5bac1 !important;
        border: none !important;
        cursor: pointer !important;
        flex-shrink: 0 !important;
      }

      /* Send button on the right */
      #send-btn, .btn-send {
        background: #7c5cfc !important;
        border-radius: 50% !important;
        width: 34px !important;
        height: 34px !important;
        display: flex !important;
        align-items: center !important;
        justify-content: center !important;
        border: none !important;
        flex-shrink: 0 !important;
        color: #fff !important;
      }

      /* Hide all web clutter buttons across composer */
      #gif-btn, #poll-btn, #time-btn, #code-btn, #draw-btn, #voice-rec-btn,
      #silent-btn, #burn-btn, #whisper-btn, #sound-effect-btn,
      .input-actions-divider, .btn-gif, .btn-poll, .btn-time,
      .pip-input-resizer,
      .input-actions-box > button:not(#upload-btn):not(#emoji-btn):not(#composer-plus-btn) {
        display: none !important;
      }

      .input-actions-box {
        display: contents !important;
      }

      /* Floating Voice Pill */
      #voice-bar {
        position: fixed !important;
        top: 56px !important;
        left: 12px !important;
        right: 12px !important;
        background: #2b2b46 !important;
        border: 1px solid #7c5cfc55 !important;
        border-radius: 20px !important;
        padding: 6px 14px !important;
        display: flex !important;
        align-items: center !important;
        gap: 8px !important;
        z-index: 9997 !important;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4) !important;
      }

      /* Slide-out Sidebar Drawer (Discord Mobile style) */
      .sidebar {
        background: #17182b !important;
        border-top-right-radius: 20px !important;
        border-bottom-right-radius: 20px !important;
        width: 84vw !important;
        max-width: 340px !important;
        padding-bottom: calc(var(--mobile-dock-height) + var(--safe-bottom) + 16px) !important;
        box-shadow: 4px 0 24px rgba(0, 0, 0, 0.5) !important;
      }

      .channel-item {
        border-radius: 12px !important;
        margin: 3px 10px !important;
        padding: 8px 12px !important;
        transition: background 0.15s ease !important;
      }

      .channel-item.active {
        background: #2b2d4a !important;
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
        background: #121324;
        border-top: 1px solid rgba(255, 255, 255, 0.08);
        display: flex;
        align-items: center;
        justify-content: space-around;
        z-index: 99999;
        user-select: none;
        box-shadow: 0 -2px 14px rgba(0, 0, 0, 0.5);
      }

      .focus-dock-tab {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        flex: 1;
        height: 100%;
        color: #949ba4;
        font-size: 11px;
        font-weight: 500;
        text-decoration: none;
        cursor: pointer;
        transition: color 0.15s ease, transform 0.15s ease;
        gap: 3px;
      }

      .focus-dock-tab svg {
        width: 22px;
        height: 22px;
        fill: currentColor;
        transition: transform 0.15s ease;
      }

      .focus-dock-tab.active {
        color: #7c5cfc;
        font-weight: 700;
      }

      .focus-dock-tab.active svg {
        transform: scale(1.12);
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

  // 3. Inject Bottom Dock HTML and Event Listeners
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
