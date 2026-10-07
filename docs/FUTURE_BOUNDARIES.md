# Future security boundaries

These are review prerequisites, not implemented capabilities or a commitment to a particular design. The current application has no module execution, OS sandbox, Vault, updater, or local profiles.

## Module execution and permissions

Current permission policy is logical metadata. It **does not provide OS-level sandbox enforcement**. Declaring or allowing a permission does not isolate code, restrict the operating system, or authorize execution today.

The bounded non-executable manifest v1 is implemented; see [Module contract](MODULE_CONTRACT.md). Before execution is introduced, review package manifest evolution, stable identity, publisher provenance, signed packages, hash verification, trust and revocation policy, installation integrity, updates, rollback/recovery, and platform-specific isolation. Hashes alone do not establish publisher authenticity. The current manifest admits metadata only; it is not an executable package manifest.

Future enforcement points could include package admission, process creation, and a narrowly scoped broker for network, filesystem, secrets, clipboard, and notifications. Each operation would need to check both declarations and user grants, scope resources, deny missing or unknown permissions, and avoid ambient authority. In-process checks alone cannot constrain hostile native code. Windows and Linux isolation feasibility must be established before promising a sandbox or enabling execution.

## Settings and local profiles

The current schema is one boolean; no schema version or migration engine is needed yet. Missing fields use intentional defaults. Unknown fields fail closed and are preserved, including files from a future incompatible version. Recovery is explicit and backed up; it must not be described as automatic migration.

Before adding incompatible fields, define schema versions, supported upgrade/downgrade behavior, and backup-preserving migrations. An older application must not silently overwrite newer data. Future local profiles should have separate validated identities and explicit selection; do not turn user input into arbitrary filesystem paths. Keep secrets separate from ordinary settings.

## Vault

Select an OS keychain or an independently reviewed encryption/key-management strategy only after an explicit threat model. Define lock/unlock semantics, key custody, plaintext lifetime, backup/export implications, loss recovery, and platform behavior. Do not persist plaintext secrets or invent cryptography. A malicious process running as the same OS user remains a relevant threat; ordinary settings permissions do not provide secret protection.

## Updates and releases

Review signed releases, authenticity verification and signing-key custody, stable/beta/nightly source separation, a user-visible release source, rollback protection, and recovery from failed upgrades. Authenticity and freshness require separate consideration. No updater, production signing, installer pipeline, or rollback engine is implemented.

## Upstream-derived modules

Before importing or redistributing ASF, StreamDropCollector, or other upstream code, review each upstream license and preserve its copyright, license, and required notices independently. VSA CORE's GPL-3.0-only declaration does not relicense dependencies or future derivative projects.
