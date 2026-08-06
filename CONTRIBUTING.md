# Contributing

Keep changes small, deterministic, and easy to audit.

1. Open an issue before a large API or security-model change.
2. Add tests for every behavior change and boundary condition.
3. Run `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all-features`.
4. Do not add network access, signing, secret handling, or unsafe Rust without an explicit design discussion.
5. Update the README and changelog when public behavior changes.

Vulnerabilities in the library itself belong in a private report described by [SECURITY.md](SECURITY.md), not a public issue. Contributions are accepted under the repository's MIT License.

