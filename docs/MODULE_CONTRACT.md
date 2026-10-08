# Metadata module contract v1

This experimental contract describes modules; it does not install, load, execute, download, or trust code. The bundled `modules/*.json` files are the four roadmap products used by the registry. They contain no implementation or credentials. Admission does not grant any permission.

## Admission

`ModuleManifest::parse` accepts at most 16 KiB of UTF-8 JSON, as an object. Schema version 1 is required. Unknown and duplicate fields, trailing JSON, unknown lifecycle/capability values, duplicate permissions, and malformed metadata fail without registry mutation. Errors are fixed messages with no input contents. The parser has no filesystem or network access. There is no manifest import IPC.

| Field | Contract |
| --- | --- |
| schemaVersion | Integer 1; unsupported versions are refused |
| id | 1–64 ASCII bytes: lowercase letters, digits, hyphens; starts/ends with a letter or digit |
| name | Nonblank; at most 128 UTF-8 bytes; no control or Unicode directional formatting characters |
| description | At most 2048 UTF-8 bytes; no control or Unicode directional formatting characters; may be empty |
| version | Null/omitted for Planned; canonical stable major.minor.patch for Available, bounded u32 components; prerelease/build syntax unsupported |
| requiredCoreApi | Optional/null canonical stable API version; does not authorize execution |
| publisherId | Optional/null validated metadata ID; unverified claim, not a trusted publisher |
| lifecycle | Planned or Available only |
| declaredPermissions | Required array of unique known capability names |

Version syntax and numeric API compatibility previews are implemented. Missing API metadata remains unspecified; experimental 0.x API minor changes are conservatively incompatible. Publisher IDs remain unverified. Signature encoding/trust-state contracts are separate from manifest admission and have no verifier or trusted state. Package resources, signatures, dependency resolution and executable entries remain unavailable. See [Foundation contracts](FOUNDATION_CONTRACTS.md). Unknown fields are refused rather than interpreted as authority. The experimental v1 extension adds optional metadata; older loaders refuse those fields and must not be advertised as interoperable.

Source manifests cannot assert Installed or Enabled: only a future trusted installation service could establish local state. Registry descriptors use a single lifecycle enum so Enabled implies Installed; no contradictory booleans exist. The registry separately validates descriptors and rejects duplicate IDs before mutation. Registration order is deterministic. IDs are identities, never filenames or paths; platform reserved filenames must be addressed before IDs are used in storage.

## Permissions and SDK

Capabilities are `network`, `filesystem.read`, `filesystem.write`, `process.execute`, `notifications`, `secrets.read`, and `clipboard`. Missing or undeclared permissions are denied. A manifest cannot supply a permission policy. Future user decisions must live separately from publisher-controlled declarations. Logical decisions do not provide an OS sandbox.

The Rust `sdk` module exposes the metadata parser, descriptor, lifecycle, and logical permission types through the existing library. It has no module entrypoint, dynamic loading, process launch, host callbacks, or plugin ABI. This is an unstable source-level contract, not a separately versioned SDK package. Modules must not assume binary compatibility with CORE. Rust callers can construct descriptors; registration remains the admission gate.

Before granting a module network access, define a broker that checks declaration, explicit user policy, resource scope, and revocation on each operation. A logical kill switch cannot stop native code using ambient OS access; platform isolation remains an owner security review prerequisite.

## Future source trust and store

A future store must separate descriptive catalog entries from verified package identity. Review publisher identity, canonical repository/source, immutable version, package digest, compatibility, permission changes, trusted signing roots, revocation, offline behavior, and update/rollback policy. A hash alone proves neither publisher identity nor safety. Use established signing standards and audited libraries; no signing keys belong in this repository. Manifest metadata alone must never authorize execution. Store networking and module downloads remain absent.
