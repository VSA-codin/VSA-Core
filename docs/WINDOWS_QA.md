# Native Windows QA

## Owner-reported baseline (2026-10-08)

These facts were reported by the owner. They are manual observations, not automated verification or a retest of this sprint's changes. Exact artifact hashes, Windows version and checkbox state were not supplied.

| Area | Owner report |
| --- | --- |
| Build | Native Windows x86_64 MSVC release build succeeded; `pnpm tauri build` produced MSI and NSIS installers. |
| Runtime | Native app reports `windows / x86_64` and `release`. |
| Titlebar | Frameless titlebar works; dragging, double-click maximize/restore, minimize, maximize/restore button and close all work on real Windows. |
| Settings | Save works; compact layout persists after restart; settings status changes from Missing/defaults to Loaded. |
| Privacy | Trust Center reveal shows only expected VSA CORE AppData paths. Support Report stays sanitized after reveal and excludes username/home directory/settings contents. |
| NSIS | Per-user installation succeeded. Uninstall removed Start Menu entry and install directory, **also removed VSA CORE user-data directory (undesirable; unresolved)**. |
| MSI | System install succeeded under Program Files; settings persisted. Uninstall removed application files/Start Menu entry and preserved VSA CORE user-data directory. |

## Test record

For each run record commit, package version, artifact SHA-256, architecture, Windows build, WebView2 version, installer type/scope, standard/admin user, language, scale, monitors and outcome. Keep usernames, absolute paths, screenshots containing paths and backup contents private. Record whether each item is pass, fail or not tested; do not replace missing observations with expectations. Use a disposable VM/test account and nonsecret settings. Back up test data outside application directories before uninstall/recovery tests.

## Clean installation and runtime

- [ ] NSIS clean per-user install as standard user: verify proposed location, VSA CORE Start Menu name, uninstall entry, publisher/version/icon, no unexpected app elevation.
- [ ] MSI clean system install: observe UAC and Program Files location, installation ACLs and Start Menu visibility for intended users.
- [ ] Record unknown-publisher/SmartScreen behavior; no Authenticode claim. Cancel/block must leave a coherent machine state.
- [ ] Exercise existing/missing WebView2 runtime and installer cancellation/network-unavailable behavior. Fully offline provisioning is not promised.
- [ ] Launch from Start Menu and executable; verify platform/architecture/release/version and all eight IPC operations through their UI flows.
- [ ] First launch with no config directory/file uses defaults without creating settings until Save.
- [ ] Save compact layout, observe success, quit normally and restart. Confirm persistence and Loaded health.
- [ ] For MSI, confirm settings go to the user's AppData rather than Program Files; run as another standard user to check separation.

## Priority NSIS persistence retest

- [ ] Build a fresh locked artifact from this branch. Save compact layout and copy/hash settings, backups and a harmless sentinel file in the app-data directories privately.
- [ ] Close CORE. Invoke interactive uninstall from Windows installed-apps UI; record **Delete app data's initial state**. Leave it **unchecked**. Capture whether it stays unchecked.
- [ ] Verify Start Menu/application removal and byte-for-byte preservation of settings, backups and sentinel data in both Roaming and Local bundle directories.
- [ ] Reinstall the same artifact and confirm Loaded/compact layout; no reset or automatic migration.
- [ ] Repeat via uninstall.exe, silent `/S` and passive `/P` in a disposable test account. Record commands locally without usernames. Confirm data preservation.
- [ ] Separately test the checked deletion option only with disposable backed-up data. Record its explicitly destructive behavior; do not use on owner data.
- [ ] If unchecked uninstall deletes data, stop acceptance: retain generated script, artifact hash, checkbox evidence and before/after layout privately. Investigate upstream; no fix is claimed by this sprint.

## MSI, reinstall and upgrades

- [ ] MSI uninstall removes installed app files/Start Menu/uninstall registration and retains settings/backups/sentinels. Reinstall restores settings.
- [ ] Same-version NSIS/MSI repair/reinstall preserves settings and avoids duplicate shortcuts or stale executables.
- [ ] Test upgrade between two distinguishable versions, with app closed and running; cancellation retains recoverable app/data state.
- [ ] Test blocked downgrade for NSIS and MSI; no user-data mutation. Older apps must refuse unsupported settings schema versions.
- [ ] Test NSIS → MSI and MSI → NSIS deliberately in a disposable VM; publisher identity is now explicit. Record previous-install detection, UAC, old artifact cleanup, shortcuts and persistence. Cross-format seamless migration is not certified.
- [ ] Test installed app with read-only/missing configuration, disk-full/access-denied, interrupted write, existing temp/backup and explicit corruption recovery. Never delete real settings to manufacture a test.
- [ ] Kill a disposable instance during save/recovery. Verify originals/backups and clear recovery guidance. Power-loss durability is not promised.

## Desktop, accessibility and privacy

- [ ] Drag, double-click maximize/restore, minimize, maximize/restore and close; repeat with keyboard and maximized window.
- [ ] Windows 10/11, 100/125/150/200% DPI, minimum 1000×650, resize, 200% text zoom and compact layout.
- [ ] Mixed-DPI multiple monitors: drag across, maximize on each, unplug/reconnect monitor and resume from sleep. Check reachable titlebar controls.
- [ ] OS light/dark, high contrast/forced colors and reduced motion: intentional dark design remains legible with visible focus.
- [ ] Keyboard-only navigation, skip link, form labels, switch state, module dropdown arrows/Home/End/type-ahead/Enter/Space/Escape/Tab, disclosure and search.
- [ ] Recovery Cancel/Escape restores focus to review button; successful save/recovery focuses its status. Error/retry states remain reachable.
- [ ] NVDA/JAWS native smoke test with landmarks/headings/button names/selection/status announcements; record actual reader/version. No screen-reader certification is claimed.
- [ ] Support text and embedded manifest are deterministic for unchanged facts, selectable, path-free after reveal/hide, and contain no settings/module descriptions/backup contents. No automatic clipboard/export/upload.

See [Accessibility QA](ACCESSIBILITY_QA.md) and [Windows packaging](WINDOWS_PACKAGING.md). All unchecked items remain manual requirements.
