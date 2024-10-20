#!/bin/sh

set -eux

export RUST_BACKTRACE="${RUST_BACKTRACE:-full}"
# Needed for no-panic to correct detect a lack of panics
export RUSTFLAGS="${RUSTFLAGS:-} -Ccodegen-units=1"

target="${1:-}"

if [ -z "$target" ]; then
    host_target=$(rustc -vV | awk '/^host/ { print $2 }')
    echo "Defaulted to host target $host_target"
    target="$host_target"
fi

# Config for the following:
#
# - Targets that aren't cross compiled can build MPFR for multiprecision tests
# - Targets that musl-math-sys can't build on need to specifically exclude that
#   crate
# - windows-gnu has a problem testing proc macros

# arch-dpecific configuration
case "$target" in
    # x86 and aarch64 get run on real hosts
    aarch64*) extra_flags="--features libm-test/multiprecision-tests" ;;
    i*86*) extra_flags="--features libm-test/multiprecision-tests" ;;
    x86*) extra_flags="--features libm-test/multiprecision-tests" ;;
    # can't build musl
    *wasm*) extra_flags="--exclude musl-math-sys" ;;
    *thumb*) extra_flags="--exclude musl-math-sys" ;;
    # can't cross compile
    *) extra_flags="" ;;
esac

# os-specific configuration
case "$target" in
    *apple*) extra_flags="$extra_flags --features libm-test/multiprecision-tests" ;;
    *windows-msvc*) extra_flags="$extra_flags --exclude musl-math-sys" ;;
    # FIXME: `STATUS_DLL_NOT_FOUND` on CI for some reason
    # <https://github.com/rust-lang/rust/issues/128944>
    *windows-gnu) extra_flags="$extra_flags --exclude libm-macros" ;;
esac

if [ "${BUILD_ONLY:-}" = "1" ]; then
    cmd="cargo build --target $target --package libm"
    $cmd
    $cmd --features 'unstable'

    echo "no tests to run for no_std"
else
    cmd="cargo test --all --target $target $extra_flags"

    # stable by default
    $cmd
    $cmd --release

    # unstable with a feature
    $cmd --features 'unstable'
    $cmd --release --features 'unstable'

    if [ "$(uname -a)" = "Linux" ]; then
        # also run the reference tests when we can. requires a Linux host.
        $cmd --features 'unstable libm-test/musl-bitwise-tests'
        $cmd --release --features 'unstable libm-test/musl-bitwise-tests'
    fi
fi
