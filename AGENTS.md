# AGENTS.md

## Repo Shape

- This is a personal Bazel monorepo; prefer Bazel targets over ad hoc `go test ./...` or `cargo test` when verifying repo changes.
- Top-level `MODULE.bazel` only includes language-specific module files: `go.MODULE.bazel` and `rust.MODULE.bazel`.
- Go module path is `github.com/bmcmanus-apex/monorepo`; Gazelle owns the usual Go `BUILD.bazel` target/dependency wiring.
- Rust packages are a single Cargo workspace rooted at `Cargo.toml`; `Cargo.lock` is shared by all Rust packages.

## Toolchains And Setup

- Bazel version is pinned by `.bazelversion` to `8.1.0`.
- Rust `cargo`, `rustc`, and `rustfmt` are expected to come from `bazel_env`, not the host install.
- On a fresh checkout or after Rust toolchain changes, run `bazel run //tools:bazel_env`; `.envrc` adds `bazel-out/bazel_env-opt/bin/tools/bazel_env/bin` to `PATH`.
- Rust toolchain registration is in `rust.MODULE.bazel`; `rules_rust` is configured with Rust `1.95.0` and `crate_universe` reads only the root Cargo workspace manifest.

## Common Commands

- List targets: `bazel query //...`
- Build everything: `bazel build //...`
- Test everything: `bazel test //...`
- Run Gazelle after Go package/import changes: `bazel run //:gazelle`
- Run one Go test target: `bazel test //LeetCode/TwoSum:TwoSum_test`
- Run one Rust test target: `bazel test //hello-world-rust:test`
- Build or run one binary target: `bazel build //simplego:simplego` or `bazel run //simplego:simplego`

## Rust Dependency Workflow

- To add a Rust dependency, edit the package `Cargo.toml`, then run `cargo generate-lockfile`, then `CARGO_BAZEL_REPIN=1 bazel fetch @crates//...`, then add the unversioned Bazel dep such as `@crates//:colored` to that package's `BUILD`/`BUILD.bazel`.
- To add a new Rust package, add it to root `Cargo.toml` `workspace.members`, create its package `Cargo.toml`, and create a Bazel `rust_binary`/`rust_library`/`rust_test`; no `rust.MODULE.bazel` edit is needed for package discovery.
- Rust formatting config is repo-specific: `rustfmt.toml` sets `max_width = 80`, `wrap_comments = true`, and `comment_width = 80`.

## Project Notes

- `LeetCode/` contains Go exercise libraries and Bazel `go_test` targets; target names often preserve directory capitalization, e.g. `//LeetCode/AddTwoNumbers:AddTwoNumbers_test`.
- `my-first-controller/` is a Kubernetes `client-go` learning controller with manifests under `my-first-controller/manifests/`.
- `rules/gosec.bzl` defines a custom `go_sec` rule but is not wired into the current target graph by default.
