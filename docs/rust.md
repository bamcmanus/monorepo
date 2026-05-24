# Rust in this monorepo

Rust is built by Bazel via
[`rules_rust`](https://github.com/bazelbuild/rules_rust). Dependency resolution
uses
[`crate_universe`](https://bazelbuild.github.io/rules_rust/crate_universe.html),
which reads a single Cargo workspace at the repo root and exposes each crate as
a Bazel target under `@crates//`.

The host `cargo`/`rustc`/`rustfmt` binaries are provided by
[`bazel_env.bzl`](https://github.com/buildbuddy-io/bazel_env.bzl) and put on
`PATH` via direnv — they always reflect the toolchain pinned in
`rust.MODULE.bazel`.

## Layout

- `Cargo.toml` (repo root) — Cargo workspace declaration; lists each Rust
  package as a member.
- `Cargo.lock` (repo root) — single lockfile for the whole workspace.
- `<package>/Cargo.toml` — per-package manifest with its own `[dependencies]`.
- `<package>/BUILD` — Bazel target definitions (`rust_binary`, `rust_test`,
  `rust_library`).
- `rust.MODULE.bazel` — `rules_rust` toolchain registration and `crate_universe`
  extension wiring.
- `tools/BUILD.bazel` — `bazel_env` declaration that exposes cargo/rustc/rustfmt
  on `PATH`.
- `.envrc` — direnv config that adds the `bazel_env` bin tree to `PATH`.

## Add a dependency to an existing package

1. **Edit the package's `Cargo.toml`** — add the crate to `[dependencies]`:

   ```toml
   [dependencies]
   colored = "3"
   ```

2. **Regenerate the lockfile** from the repo root:

   ```sh
   cargo generate-lockfile
   ```

   (Uses the hermetic cargo from `bazel_env`. Verify with `which cargo` — it
   should resolve under `bazel-out/bazel_env-opt/...`.)

3. **Repin Bazel's view of the crate graph**:

   ```sh
   CARGO_BAZEL_REPIN=1 bazel fetch @crates//...
   ```

   `CARGO_BAZEL_REPIN=1` tells `crate_universe` it may regenerate per-crate
   `BUILD` files from the new lockfile.

4. **Add the dep to the package's `BUILD`**:

   ```python
   rust_binary(
       name = "my-binary",
       srcs = ["src/main.rs"],
       deps = ["@crates//:colored"],
   )
   ```

   Use the unversioned alias (`@crates//:colored`) so the dep tracks whatever's
   in `Cargo.lock`.

5. **Use it in source** (`use colored::Colorize;` etc.).

6. **Verify**:
   ```sh
   bazel build //<package>:<target>
   bazel test  //<package>:<target>
   ```

### Repin commands

Both of these trigger `crate_universe` to regenerate; pick by intent:

- `CARGO_BAZEL_REPIN=1 bazel fetch @crates//...` — targeted; only resolves and
  fetches the crate graph. Fastest for the dep-change workflow.
- `CARGO_BAZEL_REPIN=1 bazel mod deps` — resolves _all_ module extensions and
  rewrites `MODULE.bazel.lock`. Use when you've also changed
  `MODULE.bazel`/`rust.MODULE.bazel`.

Without `CARGO_BAZEL_REPIN=1`, `crate_universe` treats the existing
lockfile-derived state as authoritative and won't touch crates.io.

## Add a new Rust package

1. **Create `<new-package>/Cargo.toml`** with `[package]` + `[dependencies]` +
   `[[bin]]`/`[lib]` as appropriate.
2. **Add `<new-package>` to `members = [...]`** in the root `Cargo.toml`.
3. **Create `<new-package>/BUILD`** with `rust_binary` / `rust_library` /
   `rust_test` targets.
4. Regenerate lockfile + repin (steps 2–3 above).

The workspace-level `Cargo.toml` is the only manifest `crate_universe` needs —
it discovers members automatically. No edits to `rust.MODULE.bazel` are required
when adding a new package.

## Bump a dependency version

1. Edit the relevant `Cargo.toml`.
2. `cargo update -p <crate>` (or `cargo generate-lockfile` to refresh broadly).
3. `CARGO_BAZEL_REPIN=1 bazel fetch @crates//...`
4. Rebuild.

## Hermetic toolchain

`cargo`, `rustc`, and `rustfmt` on your `PATH` come from `bazel_env`, not your
host install. They're materialized by:

```sh
bazel run //tools:bazel_env
```

The `.envrc` runs this implicitly when the bin tree is missing. To bump the Rust
version, edit `versions = [...]` in `rust.MODULE.bazel`, then `direnv reload`
(the `watch_file` line will pick up the change).
