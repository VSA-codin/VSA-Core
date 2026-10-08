# Release engineering foundation

Status: validation and design only. No production release, signing certificate, updater root, artifact publication or store upload is configured by this sprint.

## Validation and versioning

Cargo.toml is the desktop version source; package.json matches and Tauri inherits it. Keep versions synchronized in a reviewed commit. ReleaseVersion contracts support numeric stable triples only; review prerelease/build syntax before beta/nightly artifacts. Existing CI uses locked pnpm/Cargo dependency graphs, Node 24, stable Rust and Linux/Windows compilation/testing. Windows CI now builds both MSI and NSIS. `stable` Rust and major-tag actions are mutable inputs: lockfiles alone do not establish reproducible binaries. Record actual toolchain versions and consider reviewed digest/commit pinning and dependency provenance before releases; do not silently describe current CI as hermetic or reproducible.

Current actions retain repository policy/version choices, `contents: read`, no signing/publishing secrets and no release upload step. Expanded Clippy uses all targets/features with warnings denied. Installer compilation does not test uninstall or native accessibility. See [Windows QA](WINDOWS_QA.md) and [Accessibility QA](ACCESSIBILITY_QA.md).

## Owner-controlled candidate procedure

1. Review scope, license notices, dependency provenance and open vulnerability findings. Resolve or explicitly assess the unsuppressed moderate glib alert.
2. Choose version and signed source commit; run all validation gates. Keep feature PR #2 Draft until owner approval, never merge it automatically.
3. Build native Windows MSVC MSI/NSIS and Linux artifacts using recorded clean toolchains. Record artifact names as generated rather than promising guessed names. Linux targets depend on installed bundling prerequisites and need separate candidate validation.
4. Record SHA-256 checksums per artifact; checksums detect corruption but do not authenticate an untrusted source. Establish owner-reviewed artifact signing/provenance before calling updates authenticated.
5. Generate/review an SBOM from the locked Rust/npm graph with an audited tool selected by the owner, including licenses and transitive dependencies. No new SBOM tool/dependency is introduced here; no complete license audit or provenance attestation is claimed.
6. Complete native install/uninstall/reinstall/upgrade/privacy/accessibility tests, especially unresolved NSIS unchecked-uninstall preservation. Preserve private evidence separately from public support reports.
7. Review Authenticode certificate custody/trusted timestamps, artifact signature validation and renewal. Git commit signatures are not binary signatures. Unknown publisher/SmartScreen warnings remain possible today.
8. Publish only through a separately approved least-privilege release workflow. No release is published by this sprint.

## Release notes template

- Version / source commit / date / target OS and architecture.
- Implemented changes and bug fixes.
- Foundation-only changes and unavailable execution/security capabilities.
- Automated checks and observed native QA (with precise platform coverage).
- Installer scope and persistent user-data behavior; settings compatibility and recovery guidance.
- Known issues/security advisories; unsigned-artifact warning where applicable.
- Artifact/checksum/signature/provenance references once real infrastructure exists.
- Rollback instructions preserving data, with unsupported-schema refusal; no silent data resets.

A future changelog should record user-visible changes and incompatible contracts per release. Do not treat every experimental SDK type as a released stable API. Binary rollback and schema rollback are separate; recover from a preserved compatible snapshot only after explicit review. No rollback engine is implemented.
