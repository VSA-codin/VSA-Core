# Local data and future operational boundaries

This document records design decisions and review gates. Current persistence is one small settings JSON file and explicit corruption backups. There is no database, profile manager, backup archive, Vault, updater, scheduler runtime, network listener, or local audit log.

## Automation and scheduling

The experimental Rust SDK provides `AutomationPlan` and `AutomationTrigger`. A plan contains bounded identity/name metadata, a module ID, a module-defined action ID, and a Manual or Interval trigger. IDs use the same restricted identity grammar as modules. Intervals are integer minutes from 1 through 10,080 (7 days). Trigger JSON rejects unknown/duplicate fields and variants. Plans can only be constructed through validation and always serialize `enabled: false`.

Plans contain no command string, executable path, arbitrary arguments, URL, or secret. They are not persisted, exposed through IPC, or consumed by a timer/dispatcher. They do not prove that a module/action exists. They have no last-run result because nothing runs. The Automation view remains a factual placeholder.

A future action registry must verify module identity, compatibility, installed state, action availability, declarations, explicit consent, and resource scopes before dispatch. A plan must never grant permissions. Review cancellation, concurrency, stale references, failure reporting, and bounded history. Default all new workflows to disabled.

The interval contract is relative duration metadata, not calendar time. Before runtime scheduling, define monotonic timing while running, sleep/restart behavior, missed-run policy, overlap handling, timezone/DST rules for any calendar triggers, and explicit next-run semantics. No privileged service or OS scheduled task is needed for this foundation. Do not launch shell commands or add remote administration.

## Settings evolution and profiles

One boolean does not justify a migration framework or settings schema version yet. The strict loader preserves unknown/newer fields instead of overwriting them. Add an explicit schema version before incompatible settings changes; define supported versions, backup-first migration, safe downgrade refusal, and tests. Recovery is a user-approved reset, never an implicit migration.

Future profiles should isolate configuration, module preferences, and data beneath validated opaque profile identities resolved by a service. Do not allow arbitrary paths or derive filesystem names from display names. Test case collisions, Windows reserved names, Unicode, symlinks/reparse points, concurrent processes, read-only locations, and profile switching during writes. Vault keys and account references need separate isolation and cleanup semantics.

Portable mode requires an explicit storage-location choice and a separate threat model. Relative locations must resolve against a fixed reviewed base, never the current working directory. Removable media can lose writes, inherit weak permissions, or disappear. OS keychain-bound secrets may not be portable; encrypted portable profiles need reviewed key management. No encrypted/portable profile capability exists today.

## Backup and restore

Current explicit corruption recovery preserves exact invalid settings bytes before resetting defaults. It is not a general backup/restore feature. Corrupt backups may contain unexpected user-provided data; do not automatically include them in support reports.

A future nonsecret export should preview an allowlist of settings/module preferences, use an explicit user action, identify schema versions, and exclude Vault contents, credentials, paths, and account data unless separately reviewed. Restore should validate sizes and schema before mutation, preserve the prior configuration, show a diff, and require explicit confirmation. Never overwrite newer incompatible data.

Archive support would require traversal/absolute-path rejection, symlink/hardlink refusal, bounded entries and decompression, duplicate-name/case-collision handling, Windows reserved-name checks, staging, and failure recovery. Prefer a bounded single JSON document while data is small. No cloud backup or archive importer is implemented.

## Diagnostics, support, and event history

The implemented support preview is explicit, local, deterministic, and path-free. Manual text selection gives the owner control over sharing. File export or support bundles should retain that preview/allowlist boundary; never enumerate environment variables, browser data, clipboard, unrelated files, or corrupt backups. No automatic upload or telemetry is permitted.

A persistent audit log is deferred: current operations already show success/failure, and event retention would introduce additional privacy, integrity, storage, and deletion obligations. If justified later, use bounded local records with explicit clear/delete controls, no secret values or identifying paths, and defined behavior when logging fails. Do not claim tamper resistance against the same user.

Filesystem JSON remains sufficient for current bounded settings. A database becomes useful only with demonstrated transactional/query needs such as substantial local history or multiple linked entities. Do not add SQLite for speculative future modules.

## Vault and updates

Vault implementation remains blocked on owner security review of OS keychain availability, an audited encryption strategy where required, key custody, lock/unlock state, plaintext lifetime/zeroization limits, migration, backups, loss recovery, and same-user threats. No plaintext secret storage or home-grown encryption is acceptable.

Updates require reviewed trusted sources, established signature verification, secure signing-key custody, release channels (stable/beta/nightly), explicit user confirmation, offline behavior, rollback/failure recovery, and rollback protection. Authenticity does not prove freshness. Channel metadata is deferred until a real consumer exists; no download or installation is enabled.

## CLI and platforms

Pure domain contracts and the settings service are separated from Tauri command wrappers. A future local CLI can reuse domain/services with explicit storage resolution; it must not create a public listener or bypass GUI permission policy. Do not expose arbitrary filesystem paths through IPC merely for CLI convenience.

Tauri resolves native application locations. Linux runtime requires the native GTK/WebKit stack; Windows runtime uses WebView2 and does not require WSL. No platform abstraction is added without actual duplication. OS keychain, notifications, startup registration, and service adapters require concrete needs and separate least-privilege review.
