# Desktop readiness review

This sprint reviews source and automated checks; it does not claim visual or assistive-technology certification. No browser automation, installer, or production signing is involved.

## Native Linux and Windows

The window minimum is 1000×650. At this width the 216px sidebar and 56px content padding leave about 728px before the scrollbar. Foundation cells now use two columns at this size, and the hero can wrap. Main content scrolls independently; the sidebar can scroll when text scaling exceeds available height. A keyboard skip link targets the main region, which has a page heading label. Controls retain visible focus, reduced-motion handling, and forced-colors switch/selected-navigation indicators.

Remaining native checks: launch on Linux WebKitGTK and Windows WebView2, resize to 1000×650, inspect every page with default and compact spacing, use keyboard-only navigation and the skip link, reveal/hide paths, test 200% text scaling, and check Windows high contrast plus platform screen readers. These require a native observation session; CI compilation and source review cannot substitute for it. Do not mark them passed from a browser preview.

CI now runs frontend build and Rust formatting/check/Clippy/tests on both Linux and Windows without bundling. Existing storage tests exercise replacement, corrupt content preservation, oversized/non-regular rejection, missing-directory defaults, non-directory config rejection, and temporary-file collisions; Unix additionally tests symlink rejection. Windows ACL behavior and native dialogs are not certified by these checks.

## Corrupted settings recovery

Recovery is now implemented as an explicit two-step Settings action. Only readable regular files of at most 16 KiB with invalid JSON/schema are eligible. The backend revalidates on execution, flushes an exclusively created `settings.json.corrupt.bak`, then rereads and compares the target bytes before replacing the original with defaults using the existing same-directory temporary-write mechanism. Missing/healthy files and unsafe file types are refused. Existing backups and temporary files are preserved and block recovery; manual review is required to resolve these cases.

Tests cover exact malformed JSON, unknown-field and invalid UTF-8 backup preservation, default persistence and subsequent replacement, missing/healthy/changed/oversized/directory refusal, backup collisions, interrupted reset writes, and Unix symlink refusal/private file modes. Serialization tests reject positional arrays, duplicate fields, and wrong value types. Native Windows interaction, ACL behavior, real disk-full/flush failures, and power-loss behavior remain unverified. No test claim substitutes for those observations. Direct directory symlinks and Windows reparse metadata are now refused. Additional tests cover post-backup changed/missing targets, Unicode config directories, escaped duplicate keys, Unix read-only write failures and private new directories. Windows reparse-point behavior still requires native observation. Same-user races and concurrent application processes remain outside the protection boundary.

## Diagnostics privacy

The default diagnostics response contains null config/data locations. Only “Show local paths” requests those locations over local IPC. Hide removes paths from component state, and leaving the page unmounts that state. Reveal failures keep paths hidden. A user-requested sanitized support preview now provides selectable text without paths or file contents, even after paths are revealed. There is no file export, logging, network sink, or environment enumeration. Revealed locations can identify the user in screenshots. A compromised authorized webview can still invoke the opt-in command; this UI choice is not an access-control boundary.

## Async and failure states

Read-only views share stale-request guards and safe Retry actions. Settings reads, writes, recovery, path reveal, and support preview ignore results after their page unmounts. Compact-layout edits update the form immediately; Save applies and persists them. Unsaved changes are labeled and discarded when leaving Settings. Navigation is temporarily disabled during Save/recovery so completion cannot leave the shell with a stale preference. Native keyboard focus, announcement timing, and layout at high text zoom still require observation. Development Strict Mode can dispatch two mount reads; stale results are discarded, and no effect performs writes.

Module search uses a labeled native search field. Lifecycle uses a locally styled button and listbox with active-descendant focus, selected-state semantics, ArrowUp/ArrowDown, Home/End, first-letter navigation, Enter/Space selection, Escape dismissal with focus return, native Tab movement, blur dismissal, and outside pointer dismissal. Clear filters and a polite result count complete the filter row. Validate these interactions in both native webviews and screen readers; source review is not interactive certification.

The desktop polish pass adds decorative local SVG navigation icons, a compact selected navigation treatment, consistent controls and focus rings, shorter module rows, labeled details, and a dedicated Support Report page. Trust Center separates implemented facts, current diagnostics, absent systems, and known limitations. Status text does not depend on color. Default and compact layouts both reduce module spacing; 200% text scaling and forced-colors behavior remain native review items.


## Review of minimum privileges

The frontend uses explicit application IPC and four least-privilege window commands for the custom titlebar. The capability grants `allow-foundation`, `core:window:allow-close`, `core:window:allow-minimize`, `core:window:allow-toggle-maximize` and `core:window:allow-start-dragging`. It does not grant `core:window:default`, shell/opener, generic filesystem/network or remote origins. Native owner-reported Windows titlebar success is recorded in [Windows QA](WINDOWS_QA.md); remaining native/accessibility/installer tests are separate from compilation.
