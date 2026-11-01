# Releasing

1. Verify `npm run lint`, `npm test`, `npm run typecheck`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`.
2. Smoke-test hover-without-open, click-to-open, drag-without-open, explicit collapse, `Esc`, edge snap, language switch, always-on-top toggle, reset-credit popover, refresh, tray, offline/cache, and login states on Windows and macOS.
3. Update `CHANGELOG.md` and ensure versions match in `package.json`, `src-tauri/Cargo.toml`, and `tauri.conf.json`.
4. Add `docs/release-notes-vX.Y.Z.md` for the matching version, tag `vX.Y.Z`, and push the tag only after CI passes.
5. Review the draft GitHub Release, platform ZIP files, and `.sha256` files before publishing.

The workflow produces unsigned packages unless signing secrets are configured. Do not claim Windows signing or Apple notarization until the corresponding secrets and workflow steps are verified. On macOS, users may need to approve or remove quarantine from an unsigned build.

If a hosted runner is unavailable, manually dispatch `release.yml` from `main` with the existing release tag and a `platforms` JSON array to rebuild only the missing platform. For example, `[{"os":"macos-15-intel","label":"macos-arm64","target":"aarch64-apple-darwin"}]` builds the Apple Silicon package on an Intel Mac, installs the Rust target, and verifies the ARM64 executable before upload. It still checks out the specified release tag; completed assets from other platforms are retained. See the [Tauri GitHub build pipeline](https://v2.tauri.app/distribute/pipelines/github/) for target-based macOS builds.

Use an explicit cross-compilation target only after native tests and Clippy have passed for that release tag: this supplemental path skips those host checks and builds the selected target directly.
