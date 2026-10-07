# VSA CORE threat model

## Scope and assets

Current assets are local settings integrity, truthful registry metadata, UI trust, and application source/build integrity. There is no module execution, secret store, authentication, updater, or public server. The user controls the device; hostile code running with the same local privileges is outside the current protection boundary.

| Threat | Current mitigation | Future work / current risk |
| --- | --- | --- |
| Malicious future module or compromised plugin | Metadata only; no download, load, or execution API | Execution isolation, signature verification, permission enforcement need separate review |
| Over-permissioned module | Typed declarations; missing and undeclared permissions denied by logical policy | No OS enforcement exists; future grants need scoped resources and explicit consent |
| Compromised dependency / supply-chain attack | Lockfiles, existing CI/security checks; no bulk updates | Dependencies and build tools execute with developer privileges; review provenance and releases |
| Secret leakage | Settings schema has no secret fields and rejects unknown fields; no secret features | Never introduce credentials into settings; future Vault requires independent architecture review |
| Log leakage | No application logging or support bundle; support preview uses a path-free field allowlist | Future logging requires redaction tests and retention policy; diagnostics paths can identify local users when explicitly revealed |
| Unsafe filesystem access | Fixed app-specific filename, Tauri-resolved config location, bounded reads, regular-file checks, exclusive temporary creation | Same-user TOCTOU races, parent-directory symlinks, ACLs, concurrent processes, and unusual filesystem semantics remain risks |
| Corrupted local configuration | Invalid content is reported and preserved; missing files use defaults; save refuses corrupt targets | Explicit confirmed recovery backs up bounded invalid settings before reset; existing backups/interrupted writes require manual review; parent directory is not synced for power-loss durability |
| Command execution | No module or shell execution commands | Review any future process launch, arguments, privilege boundary, and sandbox |
| Unsafe IPC input | Typed strict settings schema; eight narrow local commands; no user-supplied paths | Tauri custom commands are reachable by compromised authorized webview content; future inputs need size and semantic validation |
| Malicious webview content | Local assets and restrictive CSP; no remote navigation feature or opener permission | CSP is defense in depth, not a complete XSS solution; dependencies remain trusted |
| Malicious future updater | No updater | Signing, transport, rollback, and key custody need owner review |
| Local privilege abuse | No root/system service, startup integration, or elevated runtime feature | Same-user processes can alter application files; no tamper resistance claim |
| Misleading security UI | Backend-derived diagnostics and explicit unfinished systems | No encryption, sandbox, integrity verification, or generic “Protected” claim |

## Accepted/current risks

The known moderate glib 0.18.x alert in the GTK/WebKit/Tauri Linux stack is an upstream stack issue. No unsupported overrides or alert suppression are introduced. Monitor supported upstream fixes; this document does not assert the alert is harmless.

Linux and Windows automated compilation and storage tests do not establish native webview behavior. Native Windows testing, especially rename replacement and inherited directory permissions, is required before release. No reproducible release, production signing, or installer pipeline is claimed.

Local diagnostics omits config/data locations from default IPC responses and allows explicit local reveal. It never enumerates unrelated files or full environment values. User-initiated support preview excludes paths and file contents even after reveal. There is no automatic export or transmission mechanism; users control manual copying/sharing. Users should review paths before sharing screenshots.

## Future review gates

Review executable modules, updater, Vault, credentials, browser integrations, external command execution, cloud accounts, remote administration, and production signing independently before implementation. Logical permission metadata must not be presented as enforced isolation.

See [Future boundaries](FUTURE_BOUNDARIES.md) for the review gates before adding those systems. Current settings files have no protection against a compromised local account; parent-directory symlinks and Windows reparse points are not comprehensively defended against.
