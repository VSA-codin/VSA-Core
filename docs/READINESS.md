# Desktop readiness review

This sprint reviews source and automated checks; it does not claim visual or assistive-technology certification. No browser automation, installer, or production signing is involved.

## Native Linux and Windows

The window minimum is 1000×650. At this width the 230px sidebar and 60px content padding leave about 710px before the scrollbar. Foundation cells now use two columns at this size, and the hero can wrap. Main content scrolls independently; the sidebar can scroll when text scaling exceeds available height. A keyboard skip link targets the main region, which has a page heading label. Controls retain visible focus, reduced-motion handling, and forced-colors switch/selected-navigation indicators.

Remaining native checks: launch on Linux WebKitGTK and Windows WebView2, resize to 1000×650, inspect every page with default and compact spacing, use keyboard-only navigation and the skip link, reveal/hide paths, test 200% text scaling, and check Windows high contrast plus platform screen readers. These require a native observation session; CI compilation and source review cannot substitute for it. Do not mark them passed from a browser preview.

CI now runs frontend build and Rust formatting/check/Clippy/tests on both Linux and Windows without bundling. Existing storage tests exercise replacement, corrupt content preservation, oversized/non-regular rejection, and temporary-file collisions; Unix additionally tests symlink rejection. Windows ACL behavior and native dialogs are not certified by these checks.

## Corrupted settings recovery design (implementation deferred)

Current behavior remains read-only retry and manual repair, with no automatic reset. A failed load is not proof of corruption: access failures, unsafe file types, oversized input, and invalid JSON must be distinguished before enabling recovery.

A future explicit “Back up corrupted settings and reset” action must:

1. Enable only for bounded, readable, regular settings files with invalid JSON/schema. Refuse valid/missing files, symlinks, directories, inaccessible files, and oversized files.
2. Explain that the original bytes will remain in the private config directory and the compact preference will return to default. Require a separate user confirmation; never repair on startup or retry.
3. Serialize through the settings mutex, revalidate at execution time, and create a uniquely named backup exclusively with restrictive Unix mode/inherited Windows ACLs. Never overwrite an existing backup. Write and flush all original bytes before changing settings.
4. Commit defaults with a same-directory exclusive temporary file. On any backup or replacement failure preserve the original and return a generic error without paths or content. Keep any completed backup for manual review. Do not automatically delete backups or interrupted writes.
5. Test healthy/missing refusal, exact byte preservation, unknown fields, malformed UTF-8, backup collisions, backup write/flush failures, reset write/replacement failures, changed input, and platform file types/permissions on Linux and Windows. State the existing single-process and same-user race limitations.

Implementation is deferred until the failure paths and native Windows behavior can be verified. No recovery command or destructive UI is exposed in this sprint.

## Diagnostics privacy

The default diagnostics response contains null config/data locations. Only “Show local paths” requests those locations over local IPC. Hide removes paths from component state, and leaving the page unmounts that state. Reveal failures keep paths hidden. There is no export, logging, network sink, or environment enumeration. Revealed locations can identify the user in screenshots. A compromised authorized webview can still invoke the opt-in command; this UI choice is not an access-control boundary.
