# Desktop readiness review

This sprint reviews source and automated checks; it does not claim visual or assistive-technology certification. No browser automation, installer, or production signing is involved.

## Native Linux and Windows

The window minimum is 1000×650. At this width the 230px sidebar and 60px content padding leave about 710px before the scrollbar. Foundation cells now use two columns at this size, and the hero can wrap. Main content scrolls independently; the sidebar can scroll when text scaling exceeds available height. A keyboard skip link targets the main region, which has a page heading label. Controls retain visible focus, reduced-motion handling, and forced-colors switch/selected-navigation indicators.

Remaining native checks: launch on Linux WebKitGTK and Windows WebView2, resize to 1000×650, inspect every page with default and compact spacing, use keyboard-only navigation and the skip link, reveal/hide paths, test 200% text scaling, and check Windows high contrast plus platform screen readers. These require a native observation session; CI compilation and source review cannot substitute for it. Do not mark them passed from a browser preview.

CI now runs frontend build and Rust formatting/check/Clippy/tests on both Linux and Windows without bundling. Existing storage tests exercise replacement, corrupt content preservation, oversized/non-regular rejection, and temporary-file collisions; Unix additionally tests symlink rejection. Windows ACL behavior and native dialogs are not certified by these checks.

## Corrupted settings recovery

Recovery is now implemented as an explicit two-step Settings action. Only readable regular files of at most 16 KiB with invalid JSON/schema are eligible. The backend revalidates on execution, flushes an exclusively created `settings.json.corrupt.bak`, then replaces the original with defaults using the existing same-directory temporary-write mechanism. Missing/healthy files and unsafe file types are refused. Existing backups and temporary files are preserved and block recovery; manual review is required to resolve these cases.

Tests cover exact malformed JSON, unknown-field and invalid UTF-8 backup preservation, default persistence and subsequent replacement, missing/healthy/changed/oversized/directory refusal, backup collisions, interrupted reset writes, and Unix symlink refusal/private file modes. Serialization tests reject positional arrays, duplicate fields, and wrong value types. Native Windows interaction, ACL behavior, real disk-full/flush failures, and power-loss behavior remain unverified. No test claim substitutes for those observations. Same-user races and concurrent application processes remain outside the protection boundary.

## Diagnostics privacy

The default diagnostics response contains null config/data locations. Only “Show local paths” requests those locations over local IPC. Hide removes paths from component state, and leaving the page unmounts that state. Reveal failures keep paths hidden. There is no export, logging, network sink, or environment enumeration. Revealed locations can identify the user in screenshots. A compromised authorized webview can still invoke the opt-in command; this UI choice is not an access-control boundary.
