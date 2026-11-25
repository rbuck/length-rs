# Contributing

`length-rs` is a lightweight Rust library for representing, converting, and manipulating absolute physical lengths while preserving their units of measurement. The library prioritizes simplicity, numerical correctness, predictable behavior, and minimal dependencies.

Read [DESIGN.md](DESIGN.md) before proposing changes to the public API or numerical behavior.

## Development setup

Install Rust 1.89 or newer with [rustup](https://rustup.rs/). The library uses Rust edition 2024 and has a minimum supported Rust version (MSRV) of 1.89.

Run Rust commands from the repository root:

```sh
cargo build
cargo test
```

To build and test with optional features enabled:

```sh
cargo build --all-features
cargo test --all-features
```

## Before submitting

Run the formatting, linting, and test checks:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --no-default-features -- -D warnings
cargo clippy --all-targets --all-features -- -D warnings
cargo test --no-default-features
cargo test --all-features
```

Verify compatibility with the minimum supported Rust version:

```sh
rustup toolchain install 1.89.0
cargo +1.89.0 test --no-default-features
cargo +1.89.0 test --all-features
```

Check that the docs pass tests:

```sh
cargo test --locked --doc --no-default-features
cargo test --locked --doc --all-features
```

Check that the public API documentation builds successfully:

```sh
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
```

## Design guidelines

Changes must remain consistent with the library's design principles:

- Support absolute physical lengths exclusively.
- Preserve the specified unit of measurement.
- Maintain a small, explicit public API.
- Use native Rust numerical arithmetic.
- Preserve numerical invariants.
- Maintain consistent parsing, formatting, and serialization.
- Introduce no mandatory external dependencies.
- Do not introduce unsafe code.
- Return explicit errors for arithmetic operations that can fail;
  do not introduce implicit panics.

General-purpose dimensional analysis, derived quantities, relative units, custom unit registries, and custom numerical arithmetic engines are outside the scope of this library.

## Numerical correctness

Changes affecting numerical behavior require particular care.

- Use authoritative definitions for units and conversion factors.
- Preserve exact behavior where exactness is required.
- Use justified numerical tolerances where floating-point rounding is expected.
- Test overflow, underflow, subnormal values, zero, negative values, and non-finite inputs where applicable.
- Verify conversion accuracy against independently established reference values.
- Do not rely exclusively on round-trip conversions to establish numerical correctness.
- Document changes to numerical guarantees, limitations, and error behavior.
- Verify that arithmetic operations preserve the unit of the
  left-hand length operand.
- Verify that overloaded arithmetic operators produce the same
  results and errors as their corresponding checked methods.

Do not introduce arbitrary rounding or tolerance adjustments to conceal numerical defects.

## Dependencies and security

The library has no mandatory external dependencies. Serde is supported through an optional feature.

Avoid introducing additional dependencies unless their necessity is clearly established. Explain the purpose, maintenance implications, and security considerations of any proposed dependency.

Do not commit credentials, private data, secrets, or generated build artifacts.

## Pull requests

- Keep changes focused and explain the problem and intended behavior.
- Include tests for new behavior and regression tests for corrected defects.
- Update relevant documentation when public APIs, numerical behavior, or developer workflows change.
- Consider backward compatibility when modifying public APIs or serialized representations.
- Include the checks performed and disclose known limitations.
- Avoid unrelated refactoring and unnecessary abstractions.

## Preparing and publishing a release

The package version in the root `Cargo.toml` defines the release version. Releases follow [Semantic Versioning](https://semver.org/).

1. Update `[package].version` in `Cargo.toml`.

2. Run the checks listed in [Before submitting](#before-submitting).

3. Review public API changes, numerical behavior, documentation, and compatibility with previous releases.

4. Verify that the crate can be packaged and published:

   ```sh
   cargo package --list
   cargo package
   cargo publish --dry-run
   ```

5. Commit the release changes and create an annotated tag matching the package version:

   ```sh
   VERSION=0.1.0 # Replace with the release version.
   git tag -a "v$VERSION" -m "length-rs $VERSION"
   git push origin "v$VERSION"
   ```

   Do not reuse or move a published release tag.

6. Publish the crate to crates.io:

   ```sh
   cargo publish
   ```

7. Create a corresponding GitHub release documenting the changes, compatibility considerations, and any known limitations.

The crate is distributed as a Rust library. Platform-specific binaries and installation archives are not required.

Release automation and continuous integration policies are not yet established.
