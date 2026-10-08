# Local data and future operational boundaries

This document records design decisions and review gates. Current persistence is one small versioned settings JSON file and explicit corruption backups. The [foundation contracts](FOUNDATION_CONTRACTS.md) and [automation contract](AUTOMATION.md) remain inert and nonpersistent. There is no database, profile manager, backup archive, Vault, updater, scheduler runtime, network listener, or local audit log.

## Automation and scheduling

The experimental SDK implements strict disabled automation plans, manual/one-time/interval/daily UTC/weekly UTC triggers, deterministic next-run previews, bounded retry metadata and skipped-run history previews. No timer, persistence, dispatch or execution exists. See [Automation](AUTOMATION.md) for exact schema/time semantics and owner review gates. Plans declare permissions but never grant them; module/action resolution and resource-scoped consent remain prerequisites.

## Settings evolution and profiles

Settings now serialize schema version 1. Legacy unversioned objects read as v1 without rewrites; unsupported/malformed/repeated version markers refuse Save and recovery. No automatic migration exists. See [Settings storage](SETTINGS_STORAGE.md). Recovery is an explicit backed-up reset, never an implicit migration.

Future profiles should isolate configuration, module preferences, and data beneath validated opaque profile identities resolved by a service. Do not allow arbitrary paths or derive filesystem names from display names. Test case collisions, Windows reserved names, Unicode, symlinks/reparse points, concurrent processes, read-only locations, and profile switching during writes. Vault keys and account references need separate isolation and cleanup semantics.

Portable mode requires an explicit storage-location choice and a separate threat model. Relative locations must resolve against a fixed reviewed base, never the current working directory. Removable media can lose writes, inherit weak permissions, or disappear. OS keychain-bound secrets may not be portable; encrypted portable profiles need reviewed key management. No encrypted/portable profile capability exists today.

## Backup and restore

Current explicit corruption recovery preserves exact invalid settings bytes before resetting defaults. It rereads the original after backup and refuses changed or missing targets; this is a consistency check with a remaining filesystem race. It is not a general backup/restore feature. Corrupt backups may contain unexpected user-provided data; do not automatically include them in support reports.

A future nonsecret export should preview an allowlist of settings/module preferences, use an explicit user action, identify schema versions, and exclude Vault contents, credentials, paths, and account data unless separately reviewed. Restore should validate sizes and schema before mutation, preserve the prior configuration, show a diff, and require explicit confirmation. Never overwrite newer incompatible data.

Archive support would require traversal/absolute-path rejection, symlink/hardlink refusal, bounded entries and decompression, duplicate-name/case-collision handling, Windows reserved-name checks, staging, and failure recovery. Prefer a bounded single JSON document while data is small. No cloud backup or archive importer is implemented.

## Diagnostics, support, and event history

The implemented support preview is explicit, local, deterministic, and path-free. Manual text selection gives the owner control over sharing. File export or support bundles should retain that preview/allowlist boundary; never enumerate environment variables, browser data, clipboard, unrelated files, or corrupt backups. No automatic upload or telemetry is permitted.

A persistent audit log is deferred: current operations already show success/failure, and event retention would introduce additional privacy, integrity, storage, and deletion obligations. If justified later, use bounded local records with explicit clear/delete controls, no secret values or identifying paths, and defined behavior when logging fails. Do not claim tamper resistance against the same user.

Filesystem JSON remains sufficient for current bounded settings. A database becomes useful only with demonstrated transactional/query needs such as substantial local history or multiple linked entities. Do not add SQLite for speculative future modules.

## Vault and updates

Vault implementation remains blocked on owner security review of OS keychain availability, an audited encryption strategy where required, key custody, lock/unlock state, plaintext lifetime/zeroization limits, migration, backups, loss recovery, and same-user threats. No plaintext secret storage or home-grown encryption is acceptable.

Updates require reviewed trusted sources, established signature verification, secure signing-key custody, release channels (stable/beta/nightly), explicit user confirmation, offline behavior, rollback/failure recovery, and rollback protection. Authenticity does not prove freshness. Channel/integrity metadata and inert review/rollback previews now exist; no download or installation is enabled. See [Foundation contracts](FOUNDATION_CONTRACTS.md).

## CLI and platforms

Pure domain contracts and the settings service are separated from Tauri command wrappers. A future local CLI can reuse domain/services with explicit storage resolution; it must not create a public listener or bypass GUI permission policy. Do not expose arbitrary filesystem paths through IPC merely for CLI convenience.

Tauri resolves native application locations. Linux runtime requires the native GTK/WebKit stack; Windows runtime uses WebView2 and does not require WSL. No platform abstraction is added without actual duplication. OS keychain, notifications, startup registration, and service adapters require concrete needs and separate least-privilege review.
