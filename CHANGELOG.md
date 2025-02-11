# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.12](https://github.com/tgross35/rust-libm/compare/libm-v0.2.11...libm-v0.2.12) - 2025-02-11

### Other

- Add `roundeven{,f,f16,f128}`
- Add `fminimum`, `fmaximum`, `fminimum_num`, and `fmaximum_num`
- Combine `fmin{,f,f16,f128}` and `fmax{,f,f16,128}` into a single file
- Small refactor of bigint tests
- Eliminate the use of `force_eval!` in `ceil`, `floor`, and `trunc`
- Migrate away from nonfunctional `fenv` stubs
- Introduce a trait constant for the minimum positive normal value
- Implement `u256` with two `u128`s rather than `u64`
- Replace an `assert!` with `debug_assert!` in `u256::shr`
- Add simple icount benchmarks for `u256` operations
- Port the CORE-MATH version of `cbrt`
- Add an enum representation of rounding mode
- Uncomment some hex float tests that should work now
- Convert `fmaf` to a generic implementation
- Remove or reduce the scope of `allow(unused)` where possible
- fix exponent calculation for subnormals
- Add better edge case testing for `scalbn`
- Add `fmaf128`
- Make it possible to use `hf32!` and similar macros outside of `libm`
- Improve tidy output
- Ensure zero has the correct sign
- Start converting `fma` to a generic function
- Add checks via annotation that lists are sorted or exhaustive
- Do not add `libm_helper.rs` to the sources list
- Add `scalbnf16`, `scalbnf128`, `ldexpf16`, and `ldexpf128`
- Fix hex float trait recursion problem
- Rename `EXP_MAX` to `EXP_SAT`
- Specify license as just MIT
- Introduce a wrapper type for IEEE hex float formatting
- Support parsing NaN and infinities from the `hf*` functions
- Switch musl from a script download to a submodule
- Ignore specific `atan2` and `sin` tests on i586
- Rework the available Cargo profiles
- Remove remnants of the `checked` feature
- Upgrade all dependencies to the latest version
- Add `fmodf128`
- Add `fmodf16` using the generic implementation
- Add a generic version of `fmod`
- Add `fminf16`, `fmaxf16`, `fminf128`, and `fmaxf128`
- Add a generic version of `fmin` and `fmax`
- Add `roundf16` and `roundf128`
- Add a generic version of `round`
- Add a generic version of `scalbn`
- Change `from_parts` to take a `u32` exponent rather than `i32`
- Add `hf16!` and `hf128!`
- Add `rintf16` and `rintf128`
- Add a generic version of `rint`
- Adjust `ceil` style to be more similar to `floor`
- Add `floorf16` and `floorf128`
- Add a generic version of `floor`
- Add `ceilf16` and `ceilf128`
- Add a generic version of `ceil`
- Make `Float::exp` return an unsigned integer
- Shift then mask, rather than mask then shift
- Add `sqrtf16` and `sqrtf128`
- Copy the u256 implementation from compiler_builtins
- Port the most recent version of Musl's `sqrt` as a generic algorithm
- Ignore files relevant to benchmarking
- Add benchmarks using iai-callgrind
- Adjust precision and add xfails based on new tests
- Simplify and optimize `fdim` ([#442](https://github.com/tgross35/rust-libm/pull/442))
- Don't set `codegen-units=1` by default in CI
- Add `fdimf16` and `fdimf128`
- Add a generic version of `fdim`
- Add `truncf16` and `truncf128`
- Add a generic version of `trunc`
- Add a utility crate for quick evaluation
- Enable `build-mpfr` and `build-musl` by default
- Rename the `test-multiprecision` feature to `build-mpfr`
- Introduce arch::aarch64 and use it for rint{,f}
- Use wasm32 arch intrinsics for rint{,f}
- Expose C versions of `libm` functions in the `cb` crate
- Add `biteq` and `exp_unbiased` to `Float`
- Add a `release-checked` profile with debug and overflow assertions
- Remove `ExpInt` from `Float`, always use `i32` instead
- Split `cast` into `cast` and `cast_lossy`
- Use `core::arch::wasm` functions rather than intrinsics
- Account for optimization levels other than numbers
- Replace "intrinsic" config with "arch" config
- Don't use intrinsics abs for `f16` and `f128` on wasm32
- Remove an unused `feature = "force-soft-floats"` gate
- Switch from using `unstable-intrinsics` to `intrinsics_enabled`
- Add test infrastructure for `f16` and `f128`
- Add `fabsf16`, `fabsf128`, `copysignf16`, and `copysignf128`
- Enable `f16` and `f128` when creating the API change list
- Add more detailed definition output for `update-api-list.py`
- Rename `unstable-test-support` to `unstable-public-internals`
- Add a way for tests to log to a file
- Use intrinsics for `abs` and `copysign` when available
- Rename generic `abs` to `fabs`
- Use `rustdoc` output to create a list of public API
- Remove an `is_nan` workaround that is no longer needed
- Update and slightly refactor some of the `Float` trait
- Add `f16` and `f128` configuration from `compiler-builtins`
- Introduce generic `abs` and `copysign`
- Fix new `clippy::precedence` lints
- Introduce helper types for accessing trait items
- Fix a bug in `abs_diff`
- Remove tests against system musl
- Use `https:` links in `README.md`
- Move some numeric trait logic to default implementations
- Resolve clippy errors in `libm` tests and check this in CI
- Add some more basic docstrings ([#352](https://github.com/tgross35/rust-libm/pull/352))
- Introduce `hf32!` and `hf64!` macros for hex float support
- Fix errors reported by Clippy in `libm`
- Expose the `support` module publicly with a test feature
- Update libm `Float` and `Int` with functions from the test traits
- Change prefixes used by the `Float` trait
- Remove `libm-bench`
- Rename `canonical_name` to `base_name`
- Add float and integer traits from compiler-builtins
- Move architecture-specific code to `src/math/arch`
- Update `select_implementation` to accept arch configuration
- Add an "arch" Cargo feature that is on by default
- Vendor `cfg_if::cfg_if!`
- Make use of `select_implementation`
- Introduce a `select_implementation` macro
- Introduce `math::arch::intrinsics`
- Replace `feature = "unstable-intrinsics"` with `intrinsics_enabled`
- Move the existing "unstable" feature to "unstable-intrinsics"

## [0.2.11](https://github.com/rust-lang/libm/compare/libm-v0.2.10...libm-v0.2.11) - 2024-10-28

### Fixed

- fix type of constants in ported sincosf ([#331](https://github.com/rust-lang/libm/pull/331))

### Other

- Disable a unit test that is failing on i586
- Add a procedural macro for expanding all function signatures
- Introduce `musl-math-sys` for bindings to musl math symbols
- Add basic docstrings to some functions ([#337](https://github.com/rust-lang/libm/pull/337))

## [0.2.10](https://github.com/rust-lang/libm/compare/libm-v0.2.9...libm-v0.2.10) - 2024-10-28

### Other

- Set the MSRV to 1.63 and test this in CI

## [0.2.9](https://github.com/rust-lang/libm/compare/libm-v0.2.8...libm-v0.2.9) - 2024-10-26

### Fixed

- Update exponent calculations in nextafter to match musl

### Changed

- Update licensing to MIT AND (MIT OR Apache-2.0), as this is derivative from
  MIT-licensed musl.
- Set edition to 2021 for all crates
- Upgrade all dependencies

### Other

- Don't deny warnings in lib.rs
- Rename the `musl-bitwise-tests` feature to `test-musl-serialized`
- Rename the `musl-reference-tests` feature to `musl-bitwise-tests`
- Move `musl-reference-tests` to a new `libm-test` crate
- Add a `force-soft-floats` feature to prevent using any intrinsics or
  arch-specific code
- Deny warnings in CI
- Fix `clippy::deprecated_cfg_attr` on compiler_builtins
- Corrected English typos
- Remove unneeded `extern core` in `tgamma`
- Allow internal_features lint when building with "unstable"

## [v0.2.1] - 2019-11-22

### Fixed

- sincosf

## [v0.2.0] - 2019-10-18

### Added

- Benchmarks
- signum
- remainder
- remainderf
- nextafter
- nextafterf

### Fixed

- Rounding to negative zero
- Overflows in rem_pio2 and remquo
- Overflows in fma
- sincosf

### Removed

- F32Ext and F64Ext traits

## [v0.1.4] - 2019-06-12

### Fixed

- Restored compatibility with Rust 1.31.0

## [v0.1.3] - 2019-05-14

### Added

- minf
- fmin
- fmaxf
- fmax

## [v0.1.2] - 2018-07-18

### Added

- acosf
- asin
- asinf
- atan
- atan2
- atan2f
- atanf
- cos
- cosf
- cosh
- coshf
- exp2
- expm1
- expm1f
- expo2
- fmaf
- pow
- sin
- sinf
- sinh
- sinhf
- tan
- tanf
- tanh
- tanhf

## [v0.1.1] - 2018-07-14

### Added

- acos
- acosf
- asin
- asinf
- atanf
- cbrt
- cbrtf
- ceil
- ceilf
- cosf
- exp
- exp2
- exp2f
- expm1
- expm1f
- fdim
- fdimf
- floorf
- fma
- fmod
- log
- log2
- log10
- log10f
- log1p
- log1pf
- log2f
- roundf
- sinf
- tanf

## v0.1.0 - 2018-07-13

- Initial release

[Unreleased]: https://github.com/japaric/libm/compare/v0.2.1...HEAD
[v0.2.1]: https://github.com/japaric/libm/compare/0.2.0...v0.2.1
[v0.2.0]: https://github.com/japaric/libm/compare/0.1.4...v0.2.0
[v0.1.4]: https://github.com/japaric/libm/compare/0.1.3...v0.1.4
[v0.1.3]: https://github.com/japaric/libm/compare/v0.1.2...0.1.3
[v0.1.2]: https://github.com/japaric/libm/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/libm/compare/v0.1.0...v0.1.1
