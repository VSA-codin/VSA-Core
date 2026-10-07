# Contributing to VSA CORE

Thank you for your interest in contributing to VSA CORE.

VSA CORE is developed as an open-source, privacy-first project. Contributions should preserve those principles.

## Core principles

Contributions should:

- Respect user privacy
- Prefer local-first operation
- Avoid unnecessary data collection
- Never introduce hidden telemetry
- Keep secrets out of source code and logs
- Request only the permissions actually required
- Keep optional functionality disableable
- Support Windows and Linux where applicable
- Preserve clear and auditable behavior

## Before contributing

Please:

1. Check existing issues and pull requests.
2. Keep changes focused and reasonably small.
3. Do not include credentials, tokens, private keys, personal data, or other secrets.
4. Test your changes before submitting them.
5. Explain security- or privacy-relevant behavior clearly.

## Development stack

VSA CORE currently uses:

- Tauri 2
- Rust
- React
- TypeScript
- Vite
- pnpm

## Foundation validation

Use the committed lockfiles; avoid unrelated dependency upgrades. From the repository root:

```sh
pnpm install --frozen-lockfile
pnpm build
cargo fmt --check --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
git diff --check
```

CI runs these build and Rust checks on Linux and Windows. Storage tests use isolated temporary directories and must preserve corrupt originals and existing backups on failure. Keep IPC errors free of file contents and absolute paths. Native webview, keyboard, and minimum-window checks remain separate from compilation; see [Desktop readiness](docs/READINESS.md).

## Security vulnerabilities

Do not report security vulnerabilities through public GitHub Issues.

Use the repository’s GitHub Private Vulnerability Reporting when available.

See [SECURITY.md](SECURITY.md) for details.

## License

By contributing to VSA CORE, you agree that your contributions are provided under the GNU General Public License v3.0 only (GPL-3.0-only).
