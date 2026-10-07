# VSA CORE

**VSA CORE** is the open-source foundation for the VSA software ecosystem.

It provides shared infrastructure for VSA applications, modules, integrations, automation tools, gaming utilities, browser extensions, and self-hosted services.

## Core principles

- Open source
- Privacy first
- Local first
- No forced VSA account
- No hidden telemetry
- No secret collection or storage implemented
- No telemetry or crash reporting implemented
- Target platforms: Windows 10 / Windows 11 / Linux (native verification pending)
- Modular architecture
- Planned: independently disableable optional modules
- Self-hostable where applicable
- Transparent permissions
- Planned: reproducible and verifiable releases

## Technology

- Tauri 2
- Rust
- React
- TypeScript
- Vite
- pnpm

## Roadmap beyond current foundations

- Module system
- Secure local vault
- Permission system
- Unified scheduler
- Notifications
- Update system with rollback
- Stable / Beta / Nightly channels
- Expanded Trust Center
- Expanded diagnostics
- Plugin SDK
- Module Store
- Network controls
- Offline mode
- Profiles
- Backup and restore
- Signed modules
- Integrity verification
- CLI
- Headless mode
- Remote management
- Local web panel
- Automation engine

## Development status

VSA CORE is currently in early development. Implemented foundations include a metadata-only module registry, working navigation, local compact-layout settings with explicit backup-preserving corruption recovery, deny-by-default permission metadata, and local Trust Center diagnostics with an opt-in sanitized support preview. Planned products cannot be installed or executed.

See [Architecture](docs/ARCHITECTURE.md) and [Threat model](docs/THREAT_MODEL.md) for current behavior and limitations. Vault, automation, updater, and executable modules are not implemented.

## License

VSA CORE is licensed under the GNU General Public License v3.0 only (GPL-3.0-only).

See [LICENSE](LICENSE) for the full license text.

## Official project

Official VSA repositories are maintained by **VSA-codin**.
