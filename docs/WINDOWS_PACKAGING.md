# Windows packaging and uninstall persistence

Status: packaging configuration implemented; native installer behavior requires owner retest. No Authenticode pipeline exists.

## Identity and scope

Cargo.toml supplies desktop version 0.1.0; package.json must match (unit tested). The product/Start Menu name stays **VSA CORE**, identifier stays `com.vsa.core`, and publisher is explicitly `VSA-codin` (the existing Cargo author). These identities must remain stable across upgrades; publisher is informational metadata, not authentication. Existing PNG/ICO assets remain bundled; branded icon quality at all Windows sizes needs visual review.

NSIS explicitly uses supported `currentUser` installation, normally under `%LOCALAPPDATA%\VSA CORE`, without requesting administrative scope for the application. MSI retains Tauri's WiX system installation behavior, owner-observed under Program Files. Settings resolve through Tauri's application config directory, separately from installation files. Windows settings are ordinarily `%APPDATA%\com.vsa.core\settings.json`. Verify actual locations through explicit Trust Center reveal; do not include them in public reports. Both installer formats disallow downgrades in configuration. This does not enforce future module/update authenticity or settings compatibility.

Windows 10/11 x86_64 MSVC is the intended native target. Build success does not certify every Windows version, DPI setup or WebView2 version. WebView2 is an OS runtime prerequisite. Tauri's existing default bootstrapper can use Microsoft's network endpoint when the runtime is missing; this is installer provisioning, not application telemetry or a promise of fully offline installation. An offline distribution strategy remains a release decision.

## NSIS investigation: unresolved owner-reported data deletion

The owner reported successful per-user installation followed by uninstall that removed shortcuts, application files **and user data**. Preserve this as an observed failure; the checkbox state and precise artifact version were not supplied. Do not claim it is fixed or infer that the owner selected deletion.

The pinned CLI is 2.12.1 in pnpm-lock.yaml. Its [upstream NSIS template](https://github.com/tauri-apps/tauri/blob/tauri-cli-v2.12.1/crates/tauri-bundler/src/bundle/windows/nsis/installer.nsi) creates an initially unchecked **Delete app data** checkbox. Recursive removal of the bundle's Roaming/Local AppData directories is conditional on that checkbox being selected and update mode being absent. Application-directory removal is nonrecursive after deleting bundled files. This source review predicts preservation with the checkbox unchecked; it does not replace a native test of the generated artifact.

The [supported NSIS configuration](https://v2.tauri.app/reference/config/#nsisconfig) has no app-data-deletion toggle. Tauri supports hooks and custom templates, but changing private checkbox variables through a hook would couple CORE to undocumented template internals. A full copied template would be a large maintenance/security burden. Neither is added. No app-directory override, settings relocation, backup-moving hook or pretend preservation setting is used. The supported default preservation path is retained; the primary reported bug remains unresolved pending native reproduction. If unchecked uninstall still deletes data, preserve the artifact hash, generated NSIS script and private before/after evidence, and report the minimal reproduction upstream before choosing a reviewed template change.

Until retested: back up test data separately and **leave Delete app data unchecked**. The upstream checked option intentionally destroys persistent data; CORE adds no deletion feature. Current configuration cannot guarantee preservation when that option is selected. MSI preservation is owner-observed and should be retested independently. Uninstall must not be treated as an application-data reset.

## Automated checks and release boundary

Rust checks stable identity, explicit NSIS per-user scope, downgrade refusal, absence of installer hooks/custom templates, version synchronization, exact window permissions and production CSP. Windows CI builds both MSI and NSIS using the lockfile. These checks validate configuration and compilation, not interactive or silent uninstall persistence.

Git commit signing != Windows Authenticode binary signing. Signed Git commits authenticate source history, not PE executables, MSI or NSIS artifacts. Current installers may display unknown publisher / SmartScreen warnings. Do not advise disabling security controls. Certificate procurement, trusted timestamping, signing-key custody, certificate renewal and actual artifact verification remain owner-controlled release work. No certificate or signing secret is requested or stored.

Use [Native Windows QA](WINDOWS_QA.md) for uninstall/reinstall/upgrade acceptance. Do not publish a release until the persistence discrepancy and signing/release expectations are reviewed.
