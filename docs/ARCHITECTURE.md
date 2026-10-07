# VSA CORE architecture

VSA CORE is a GPL-3.0-only local desktop foundation using Tauri 2, Rust, React, and TypeScript. The current implementation contains no executable module system, account requirement, telemetry collection, or network service.

## Frontend and IPC

`src/App.tsx` owns simple in-memory navigation and the dashboard. `src/pages` contains Modules, Settings, and Trust Center. `src/services/api.ts` centralizes typed Tauri invoke commands; React has no direct filesystem access. Automation and Vault are honest placeholders. Compact layout is the only saved preference and changes actual layout spacing. Each backend view has loading and error states. Page changes reset the content scroll position; the sidebar stays fixed and the content header is sticky. Module rows expose lifecycle and permission details through keyboard-accessible disclosures. Browser-only previews cannot reach Rust and show unavailable states.

Commands are `get_core_status`, `get_modules`, `get_settings`, `update_settings`, `settings_recovery_available`, `recover_settings`, and `get_diagnostics`. The Rust command layer maps lock and storage failures to safe messages without underlying OS errors or paths. Diagnostics omits application locations from IPC responses by default. Explicit local reveal requests paths; hiding clears them from component state. The managed registry is immutable after startup and needs no mutex. Settings operations are serialized through one service mutex. Poisoned settings locks return errors; they are not silently recovered. Built-in registry validation errors propagate through startup setup.

## Registry and lifecycle

`core/module_registry.rs` stores descriptors in registration order, rejects blank, malformed, and duplicate IDs, blank names, and repeated permission declarations, and counts only Enabled entries. Lifecycle is a serialized enum: Planned, Available, Installed, Enabled. Enabled implies installed by its meaning; independent booleans cannot contradict one another. There is no transition or installation API yet. IDs use lowercase ASCII letters, digits, and hyphens. Planned entries must have no version; other lifecycle states require a nonblank version. Version strings are metadata, not validated release artifacts.

The four built-in descriptors are roadmap metadata for Steam Power Suite, VSA ASF, VSA StreamDropCollector, and VSA R4R + SDA. All are Planned. No product features are implemented. Registry mutation is internal only; there is no registration IPC command.

## Settings and storage

`core/settings.rs` defines a strict serde settings schema, currently one boolean with a false default. Only JSON objects are accepted; unknown and duplicate fields and nonboolean preference values are rejected. No passwords, credentials, cookies, keys, or telemetry preferences are stored.

`commands/settings.rs` delegates to `services::SettingsService`, which delegates to `storage::SettingsStore`. Tauri resolves the application config directory at startup, without hardcoded user paths. Missing settings yield defaults without writing. Invalid JSON, oversized files, non-regular files, and symlinks are rejected and preserved. The frontend may use default layout if startup reading fails; Settings and Trust Center expose the storage error.

Writes are serialized by the service mutex, encode to a same-directory temporary file opened with create_new, sync the file, close it, then rename over settings.json. Unix temporary files use mode 0600. Existing temporary files are never overwritten; an interrupted write requires manual review and recovery. Failed writes leave the prior target intact under normal filesystem semantics. No automatic corruption repair is attempted. Settings offers retry, a preference reset that requires Save, and explicit confirmed corruption recovery. Recovery is eligible only for bounded readable regular files with invalid JSON/schema, revalidates under the service lock, exclusively creates `settings.json.corrupt.bak`, writes and syncs the exact original bytes, then uses the ordinary temporary-write replacement to persist defaults. Existing backups and interrupted writes block recovery; neither is overwritten or automatically deleted. A failed backup or replacement preserves the original under normal filesystem semantics; a partial or complete backup is retained for manual review. Backup creation uses mode 0600 on Unix and inherited ACLs on Windows.

This is a single-process persistence foundation. Cross-process coordination and hostile same-user filesystem races are not solved. Atomic rename is filesystem-dependent; parent-directory power-loss durability is not guaranteed. Windows ACLs are inherited from the application directory. Application data location is reported separately and need not exist yet.

## Permission policy

`security` provides typed permission categories and Allow/Deny decisions. Policies are serialized objects with unique typed keys; unknown categories, invalid decisions, and duplicate keys are rejected. Evaluation denies undeclared permissions and missing decisions, even if an undeclared permission has an Allow entry. Planned descriptors declare no permissions because their requirements have not been reviewed. There are no permission grants exposed in UI or IPC. This is logical metadata, not an operating-system sandbox or enforced execution boundary.

## Diagnostics

`SettingsService` reads storage availability and constructs the `core/diagnostics.rs` snapshot without putting filesystem access in the domain model. The narrow snapshot contains: version, OS, architecture, runtime, build mode, local-first/account/telemetry facts, application locations, settings load state (missing, loaded, invalid, unavailable), storage readability, registry status/counts, and allowed declared permission count. No environment dump, personal-file enumeration, logger, export, network sink, or support bundle exists. Storage readability does not claim successful writes. Paths are absent from default IPC responses, can be explicitly requested, and should be reviewed before sharing screenshots.

## Security configuration and unfinished systems

The webview CSP limits scripts and content to local sources and connections to Tauri IPC. A separate development CSP permits inline Vite refresh/style injection and loopback HMR on port 1420; release policy does not. Remote TAURI_DEV_HOST development requires an explicit reviewed dev CSP adjustment. The main window capability has core defaults and no opener permission. The unused opener plugin and its frontend/backend dependencies have been removed.

Module download/execution, OS sandboxing, Vault, encryption, authentication, scheduler, automation, updater, code signing for production, installer releases, and cloud services are not implemented. Unit tests cover registry, lifecycle, settings/storage, policy, and diagnostics. CI builds the frontend and checks Rust formatting, compilation, Clippy, and tests on Linux and Windows runners. These checks do not establish native webview UX readiness; interactive native verification remains necessary.
