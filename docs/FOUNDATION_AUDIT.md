# Foundation audit · 2026-10-07

This is a source and automated-check review of the metadata foundation, not a production release or native UX certification. The four roadmap products remain Planned. No executable module, credentials, Vault, scheduler runtime, updater, installer or account integration was added.

## Application network and webview inventory

| Surface | Current behavior | Boundary |
| --- | --- | --- |
| React application | Eight typed local Tauri commands in `src/services/api.ts` | No fetch, XMLHttpRequest, WebSocket, external scripts, CDN, remote image/font or clipboard API in app code |
| Rust application | Registry metadata, bounded local settings and diagnostics | No HTTP client calls, sockets, listener, process launch, download or upload |
| Release CSP | Local scripts/styles/assets and Tauri IPC connections | No arbitrary HTTP connection origin, frames, objects or base URL; CSP is defense in depth |
| Tauri capabilities | `allow-foundation` application permission for local `main` | Eight named commands; no built-in plugin grants or remote capability. Compromised authorized content can still use allowed commands |
| Vite development | Loopback dev URL and WebSocket HMR | Development-only tooling; not telemetry. `TAURI_DEV_HOST` changes development hosting when explicitly configured |
| Cargo lockfile | Includes `reqwest`, `hyper` and `tokio` transitively | `reqwest`/`hyper` are absent from the active Linux tree. Cached Tauri source declares reqwest for Android/iOS; its dev proxy is mobile-only. A lockfile entry alone is not an app networking path |
| Native webview | WebKitGTK on Linux, WebView2 on Windows | Network-capable platform runtime; source audit does not certify OS/runtime traffic or replace packet observation |

`dangerouslySetInnerHTML`, eval, Function construction, untrusted URLs and external-link launch are absent from application source. JSX renders metadata as text. Dependencies and the native runtime remain trusted components; this audit is not a claim that third-party internals never evaluate JavaScript or have networking support.

No telemetry, analytics, tracking, advertising, automatic report upload, environment enumeration, browser-data import or unrelated-file scan exists in application code. Diagnostics paths are null unless explicitly requested. Support Report uses an independent fixed-field allowlist, excludes paths even after reveal, and excludes module names/descriptions and settings bytes. Manual selection/sharing is controlled by the user.

## Data integrity and hostile-input review

Registry admission preserves registration order, rejects duplicate/invalid identities before mutation and uses one lifecycle enum. Planned entries have no version; other states require nonblank bounded version metadata. Source manifests accept only Planned/Available and cannot claim local installation, activation or grants. Bounds are UTF-8 bytes, not display width. Control/directional formatting characters are rejected; ordinary international text is accepted. This does not prevent all lookalike Unicode labels.

Logical permissions require both declaration and explicit Allow. Missing/undeclared/unknown permissions do not create authority. Policy deserialization rejects repeated keys, including escaped spellings, and serialization order is deterministic. Permissions remain metadata; there is no OS sandbox or module execution.

Settings accepts a strict JSON object containing only an optional boolean compact-layout preference. Unknown/duplicate fields, escaped duplicate keys, arrays, wrong types, invalid UTF-8, oversized files and unsafe target types fail without ordinary-save repair. Missing locations use defaults without writing. Reads remain bounded even if a file grows.

Writes use exclusive same-directory temporary files, file flush/close and rename replacement. Existing temp files are preserved. Explicit recovery flushes an exclusive exact-byte backup, rereads the original and refuses changed/missing targets before reset. Existing backups are never overwritten. New Unix directories are 0700 and files are 0600; existing directory modes are retained and Windows inherits ACLs.

Direct configured-directory links, direct target links and Windows reparse metadata are refused. Ancestor links, hard links, races after checks, same-user interference, cross-process coordination, ACL correctness, unusual filesystems and crash/power-loss durability remain limitations. The parent directory is not synced. File readability is not a write-access or durability claim.

Production Rust source contains no unwrap/expect, unsafe block, dbg output or broad allow attribute. Test unwraps are retained. The settings mutex intentionally spans bounded I/O to serialize one process's read/validate/write/recovery operations; no nested locking or network work occurs under it. IPC errors and fatal runtime errors use fixed messages without OS details or paths.

## Tests and CI boundaries

Rust coverage grew from 37 to 46 tests. Added boundaries include post-backup changed/missing targets, direct Unix directory links, Unicode settings paths, escaped duplicate fields, read-only write refusal, private new directories with existing-mode preservation, directional metadata, deterministic/revoked permission policy and support-report metadata exclusion. Existing tests continue covering schema/size limits, corrupt originals, collisions, lifecycle/counts, manifest authority, inert plans, diagnostics and version consistency.

Linux permission tests account for privileged runners that can bypass mode bits. Windows-only ACL/reparse behavior and native controls still need manual observation. A Windows dependency-tree command was attempted offline and lacked a cached transitive crate; no package update was performed. Windows compilation/testing is delegated to the existing Windows CI job.

CI gates remain unchanged: frozen frontend install/build, formatting, cargo check, Clippy with warnings denied and cargo test on Linux and Windows. CodeQL Actions, JavaScript/TypeScript and Rust remain enabled. No dependency was added, removed or updated. The moderate glib 0.18.5 stack advisory remains open and unsuppressed; a supported upstream-compatible solution is required.

## Second and simplification passes

Trust Center keeps implemented facts and limitations visible when diagnostics fails. Support Report restores focus when its preview is hidden. Lifecycle focus loss dismisses the popup, including backward Tab. CSS was consolidated into a small token foundation with shared controls and focus styling, removing duplicate desktop/responsive rules and decorative gradients/glow. Navigation and metadata use local assets and native disclosures. No state library, router or UI framework was introduced.

Review the native checklist in [Desktop readiness](READINESS.md) before release. Future Vault, executable modules, authentication, updates, release signing and isolation remain governed by [Future boundaries](FUTURE_BOUNDARIES.md). Existing ecosystem and local-data architecture documents already describe these absent systems; no speculative runtime scaffolding was added.
