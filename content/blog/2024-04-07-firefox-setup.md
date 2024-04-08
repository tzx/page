+++
title = "Firefox Setup"
+++

I borked my old laptop the other day ):. I currently have dotfiles that reproduced my setup, but I have nothing for Firefox (And I don't want to use Nix to manage this). Instead I am going to document it here. Honestly I should also document other stuff I ran into (missing polkit, AMD power profiles, backlight, btrfs, xdg-desktop-portal)

### about:config

```
KEY                                                          VALUE                      COMMENT
browser.cache.disk.parent_directory                          /run/user/1000/firefox     disk cache on RAM, 1000 is my UID
extensions.pocket.enabled                                    false
media.ffmpeg.vaapi.enabled                                   true                       Hardware Acceleration
privacy.trackingprotection.enabled                           true                       Enhanced Tracking Protection
privacy.firstparty.isolate                                   true                       Don't track across multiple sites
toolkit.legacyUserProfileCustomizations.stylesheets          true                       I want css, mainly for Sidebery

```

### about:preferences

#### Tabs

- [x] Ctrl+Tab cycles through tabs in recently used order
- [x] Open links in tabs instead of new windows
- [] When you open a link, image or media in a new tab, switch to it immediately
- [x] Confirm before closing multiple tabs
- [x] Confirm before quitting with Ctrl+Q

### Extensions

- [Sidebery](https://addons.mozilla.org/en-US/firefox/addon/sidebery/)
- [uBlock Origin](https://addons.mozilla.org/en-US/firefox/addon/ublock-origin/)
- [I still don't care about cookies](https://addons.mozilla.org/en-US/firefox/addon/istilldontcareaboutcookies/)
- [ClearURLS](https://addons.mozilla.org/en-US/firefox/addon/clearurls/)
- [Bitwarden](https://addons.mozilla.org/en-US/firefox/addon/bitwarden-password-manager/)

### CSS

I only use CSS for Sidebery currently. It just removes the tab bar and makes
sidebery autohide. Steps to create it are
[here](https://www.userchrome.org/how-create-userchrome-css.html).

```css
#TabsToolbar
{
    visibility: collapse;
}
#sidebar-box #sidebar-header {
  display: none !important;
}

/* show sidebar when hover */
#sidebar-box {
    --uc-sidebar-width: 48px;
    --uc-sidebar-hover-width: 300px;
    --uc-autohide-sidebar-delay: 1s;
    width: var(--uc-sidebar-width) !important;
    min-width: var(--uc-sidebar-width) !important;
    max-width: var(--uc-sidebar-width) !important;
    position: relative;
    z-index: 1;
}

#sidebar {
    /* start 1s after hover but immediately after removing hover */
    transition: min-width 0.2s ease-in-out 0s !important;
    min-width: var(--uc-sidebar-width) !important;
}

#sidebar:hover {
    transition: min-width 0.02s ease-in-out --var(--uc-autohide-sidebar-delay) !important;
}

#sidebar-box:hover > #sidebar {
    min-width: var(--uc-sidebar-hover-width) !important;
    transition-delay: 0.02s !important;
    box-shadow: 0px 6px 12px 0 rgba(0, 0, 0, 0.5);
}

#sidebar {
    border-inline: 1px solid var(--toolbar-field-background-color);
    border-inline-width: 0px 1px;
}

#sidebar-box[positionend] > * {
    border-inline-width: 1px 0px;
}

#main-window[sizemode="fullscreen"] #sidebar-box {
    --uc-sidebar-width: 1px;
    transition: 0.8s margin-left ease-out !important;
}

#sidebar-splitter {
    display: none !important;
}
```
