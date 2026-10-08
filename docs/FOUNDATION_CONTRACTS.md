# Experimental foundation contracts

Status: pure Rust SDK metadata and deterministic previews implemented; persistent services and execution unavailable. Types are exported through `vsa_core_lib::sdk`. No stable SDK/API or binary ABI is promised. No dependencies were added.

All byte-based entry points require an object, bound input to 16 KiB, reject unknown/duplicate fields by direct deserialization and return fixed errors. Nested network rules also require objects. Display labels reject controls and directional formatting. These types have no filesystem/network/process access or permission-grant method.

| Area | Implemented foundation | Unavailable / owner review |
| --- | --- | --- |
| Profiles | Schema-v1 validated `profile-` IDs, bounded names, optional caller-supplied Unix creation time, default metadata, unique bounded selection and relative `profiles/<id>` layout | Actual profile creation/storage/switching, portable mode, import/export, encryption |
| Vault | Scoped profile/module/reference IDs, unavailable/locked/recovery states and metadata-provider interface | Secret input/output, unlocking, key management, recovery and production protection |
| Signed modules | Ed25519 / RFC 8785 canonicalization identifiers, publisher/key IDs, strict 64-byte lowercase-hex signature encoding, untrusted/revoked/malformed states | Verification, canonicalization, trust roots, revocation service, trusted status |
| Catalog | Module validation, numeric CORE API compatibility, permission preview and unsigned status; blocked install/update/remove/rollback plans preserve data | Real catalog sources, package dependency resolution, download, installation and execution |
| Updater | Version/channel/artifact-ID/size/SHA-256 metadata, downgrade/channel blocking, inert review/failure/recovery transitions, rollback preview | Downloads, integrity/signature checking, staged binary state, applying/rolling back binaries or data |
| Network | Owner/direction/TCP-or-UDP/port/decision rules, conflict/duplicate checks, bounded dry-run summary and unavailable backend | Application attribution, firewall rule translation, OS mutation and enforcement |
| Automation | Strict disabled plans, declared permission metadata, UTC trigger next-run previews, bounded retry/backoff and 100-entry skipped-run history preview | Persistence, timers, module action resolution, dispatch, side effects and headless engine |
| Support bundle | Allowlisted deterministic manifest embedded in explicit support preview; no attachments | Export, local log collection, arbitrary attachments, upload |

## Profiles and encrypted portable storage

Profile IDs are caller-provided persistent identities, never regenerated from names or timestamps. Fixed `profile-` prefixes and lowercase ASCII grammar prevent Windows device-name/case aliases and traversal. Names never become filesystem segments. Creation time is optional; default metadata has no fabricated time. Selection requires 1–64 unique admitted profiles and an existing active ID. This selection model does not switch today's global settings file.

Future layout: configuration root / `profiles/<validated-id>/`, containing versioned nonsecret settings and module preferences. Resolve through a native service beneath a fixed root, never frontend paths/current working directory. Switching must pause/finish writes and revalidate references; preserve the legacy app settings until an explicit backup-first migration is reviewed. Encrypted portable mode must use a separately reviewed provider, audited KDF/authenticated encryption, OS facilities where suitable and an explicit unlock lifecycle. No encrypted format or password handling is implemented. Removable-media disappearance, weak ACLs, offline recovery and keychain nonportability are review requirements.

## Vault threat model and boundary

Protect future persisted secret values from offline disk theft and accidental disclosure; model unlocked memory, same-user malicious processes, compromised modules/frontends, stolen portable media and lost keys separately. OS administrators and compromised OS/kernel are outside ordinary app protection. OS keychains may authorize the same logged-in user; they do not automatically isolate hostile modules. Avoid broad claims of protection from same-user malware.

Secret references contain identifiers only and are still potentially sensitive metadata: they are not included in diagnostics/support. References do not grant access. A future broker must check profile ownership, module identity, scoped declarations, explicit grants, current lock state and action purpose on every retrieval. Favor using a secret for a narrowly brokered operation over returning it to JS. Current provider reports Unavailable and offers no secret-byte or unlock API.

Review key custody, authenticated encryption, nonce generation, KDF parameters, OS keychain integration, plaintext lifetime/zeroization limits, cancellation, lock-on-sleep, backup encryption, key-loss recovery and versioned formats. Backups must not silently include key material. Recovery cannot promise retrieval without a viable reviewed key strategy. Owner security review and audited dependencies are prerequisites; no custom cryptography or plaintext secret persistence.

## Signing and catalog trust

Valid signature encoding is **not verification**. `SignatureMetadata::trust_state()` always returns VerificationUnavailable. There is deliberately no Verified/Trusted variant. A publisher ID is an unverified claim. Unsigned catalog metadata stays Unsigned.

Future signed payload must bind package/module identity, version, CORE API requirement, platform/architecture, declared/scoped capabilities, dependency identities/versions, publisher/key ID, integrity digest/size, update source/channel and schema version. Specify RFC 8785 canonical UTF-8 bytes, excluded signature field, domain separation, strict duplicate handling, supported numeric/string constraints and known-answer test vectors before signing. Current serde serialization is not a canonicalizer. Adopt audited Ed25519 and canonicalization implementations after GPL compatibility/security review. Verify bytes before installation and after staging; hashes alone do not establish authenticity. Define trusted root distribution, rotation/revocation, offline policy and rollback/freshness counters. No trust root or verifier exists.

Module manifest v1 admits optional validated `requiredCoreApi` and `publisherId`; neither authorizes installation. Version strings now accept canonical stable `major.minor.patch` with bounded u32 components; prerelease/build syntax and version ranges are intentionally unsupported pending a reviewed extension. Numeric comparison avoids lexical `1.10 < 1.9` errors. Catalog preview uses the admitted manifest API value, reports unspecified compatibility when absent and conservatively refuses different experimental 0.x minor API versions. Dependency solving is deferred until package semantics exist.

## Updater and rollback

Metadata contains no URL/path and limits artifacts to 2 GiB. Stable/Beta/Nightly are metadata channels; this version parser still supports stable numeric version syntax only. Preview rejects same/older versions and cross-channel changes. State transitions stop at ReviewRequired/Blocked or Failed/RecoveryRequired; they cannot claim downloaded, verified or applied status. RollbackPreview requires an older version and always preserves user data/requires owner review.

Future pipeline: reviewed source → bounded download → integrity and authenticated signature verification → separate staging → explicit apply → restart/health confirmation → history. Every trust-sensitive step must be fail-closed. Bind platform/architecture/source/channel and freshness to signatures, prevent silent downgrades, preserve old application artifacts and snapshot nonsecret configuration before any schema migration. Binary rollback must refuse incompatible data rather than resetting it. Update interruption/disk-full/cancellation require recoverable staging; signing keys and trust roots remain owner-controlled. No real updater is active.

## Network dry runs

Rules express metadata intent for an owner ID, direction, protocol and nonzero port. Duplicate rule IDs and repeated/conflicting exact scopes fail. No ranges, hostnames, wildcard endpoints, executable paths or command strings are accepted. Dry run reports default Deny, counts and backend unavailable. Logical denial does not block any OS network traffic. The unavailable adapter has no mutation method.

Windows/Linux backends need explicit owner review of process attribution, per-user/admin scope, OS-specific rule precedence, rollback receipts, coexistence with existing firewall rules and crash recovery. Never translate owner IDs directly into arbitrary executables or system commands. No firewall command is run by CORE or this sprint.

## CLI/headless design

No second executable is added: installer scope and GUI-only Tauri startup should not be complicated by a speculative CLI. Future read-only version/status/features/module-metadata/support commands can reuse pure domain services without a WebView. Manifest validation must bound explicit caller file reads and refuse links/special files; no frontend arbitrary-path IPC is needed. Sanitize errors and default to path-free output. CLI must not bypass action/secret permission brokers or expose a listener. Mutation/execution commands require separate review.
