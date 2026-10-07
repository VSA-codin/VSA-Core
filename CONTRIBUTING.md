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

## Local setup

Use Node.js 24, pnpm 12.9.1 (matching CI), and stable Rust with `rustfmt` and Clippy. Windows development requires the Microsoft C++ build tools and WebView2 runtime. Ubuntu 24.04 CI installs `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `libxdo-dev`, `libssl-dev`, and `build-essential`; equivalent native development libraries are needed on other Linux distributions.

```sh
pnpm install --frozen-lockfile
pnpm tauri dev
```

`pnpm dev` starts only the Vite frontend; it cannot reach Rust IPC. Development uses a loopback Vite server and HMR, while production loads bundled assets. Do not mistake development tooling traffic for application telemetry. `pnpm build` checks/builds the frontend; `cargo build --manifest-path src-tauri/Cargo.toml` builds the native executable after frontend assets exist. Packaging and release signing require separate review.

Use focused feature branches and pull requests; do not work directly on `main`. Use the existing signed-commit configuration; never disable signing to work around authentication. This foundation PR remains Draft until owner review. Cargo.toml is the desktop version source; Tauri inherits it and backend facts use `CARGO_PKG_VERSION`. Keep the private frontend package version in sync; a Rust test detects drift.

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

## Developer security checklist

- Preserve default deny and validate serialized inputs, including duplicate fields.
- Keep secrets, identifying paths, and internal OS errors out of normal IPC and reports.
- Review every new network, process, filesystem, and webview capability.
- Preserve originals on storage failure; use isolated test directories.
- Review direct dependencies and lockfile changes; avoid unrelated upgrades.
- Match UI/security claims and documentation to implemented, tested behavior.

See [Future boundaries](docs/FUTURE_BOUNDARIES.md) before proposing executable modules, Vault, profiles, or updates.

## Security vulnerabilities

Do not report security vulnerabilities through public GitHub Issues.

Use the repository’s GitHub Private Vulnerability Reporting when available.

See [SECURITY.md](SECURITY.md) for details.

## License

By contributing to VSA CORE, you agree that your contributions are provided under the GNU General Public License v3.0 only (GPL-3.0-only).
