# Security Policy

Security and privacy are core principles of VSA CORE.

## Reporting a vulnerability

Please do not publicly disclose security vulnerabilities through GitHub Issues.

Security vulnerabilities should be reported privately through GitHub's security reporting features when available for this repository.

Please include:

- A clear description of the vulnerability
- Steps to reproduce the issue
- The affected VSA CORE version or commit
- Potential security or privacy impact
- Any suggested mitigation, if known

## Security principles

VSA CORE aims to follow these principles:

- Local-first architecture
- No hidden telemetry
- No forced VSA account
- Secrets must remain local unless the user explicitly chooses otherwise
- Sensitive values must never be written to logs
- Permissions should be explicit and minimal
- Network access should be transparent
- Optional modules must be independently disableable
- Official releases should be verifiable
- Security-sensitive functionality should fail safely

## Current foundation boundary

CORE currently provides metadata, nonsecret local settings and local diagnostics. Logical module permissions do not provide an OS sandbox. Vault, encryption, module execution and updates are not implemented. Same-user processes and filesystem races remain outside the protection boundary.

See the [threat model](docs/THREAT_MODEL.md) and [foundation audit](docs/FOUNDATION_AUDIT.md) for implementation limits. The known moderate glib 0.18 stack advisory remains open pending an upstream-compatible resolution; it is not suppressed.

## Supported versions

VSA CORE is currently in early development.

Until the first stable release, security fixes are provided for the latest development version only.

## Public disclosure

Please allow reasonable time for a security issue to be investigated and fixed before publishing technical details.
