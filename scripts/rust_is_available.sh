#!/bin/sh
# SPDX-License-Identifier: GPL-2.0
#
# Tests whether a suitable Rust toolchain is available.

set -e

min_tool_version=$(dirname $0)/min-tool-version.sh

# Convert the version string x.y.z to a canonical up-to-7-digits form.
#
# Note that this function uses one more digit (compared to other
# instances in other version scripts) to give a bit more space to
# `rustc` since it will reach 1.100.0 in late 2026.
get_canonical_version()
{
	IFS=.
	set -- $1
	echo $((100000 * $1 + 100 * $2 + $3))
}

# Print a reference to the Quick Start guide in the documentation.
print_docs_reference()
{
	echo >&2 "***"
	echo >&2 "*** Please see Documentation/rust/quick-start.rst for details"
	echo >&2 "*** on how to set up the Rust support."
	echo >&2 "***"
}

# Print an explanation about the fact that the script is meant to be called from Kbuild.
print_kbuild_explanation()
{
	echo >&2 "***"
	echo >&2 "*** This script is intended to be called from Kbuild."
	echo >&2 "*** Please use the 'rustavailable' target to call it instead."
	echo >&2 "*** Otherwise, the results may not be meaningful."
	exit 1
}

# If the script fails for any reason, or if there was any warning, then
# print a reference to the documentation on exit.
warning=0
trap 'if [ $? -ne 0 ] || [ $warning -ne 0 ]; then print_docs_reference; fi' EXIT

# Check that the expected environment variables are set.
if [ -z "${RUSTC+x}" ]; then
	echo >&2 "***"
	echo >&2 "*** Environment variable 'RUSTC' is not set."
	print_kbuild_explanation
fi

if [ -z "${CC+x}" ]; then
	echo >&2 "***"
	echo >&2 "*** Environment variable 'CC' is not set."
	print_kbuild_explanation
fi

# Check that the Rust compiler exists.
if ! command -v "$RUSTC" >/dev/null; then
	echo >&2 "***"
	echo >&2 "*** Rust compiler '$RUSTC' could not be found."
	echo >&2 "***"
	exit 1
fi

# Check that the Rust compiler version is suitable.
#
# Non-stable and distributions' versions may have a version suffix, e.g. `-dev`.
rust_compiler_output=$( \
	LC_ALL=C "$RUSTC" --version 2>/dev/null
) || rust_compiler_code=$?
if [ -n "$rust_compiler_code" ]; then
	echo >&2 "***"
	echo >&2 "*** Running '$RUSTC' to check the Rust compiler version failed with"
	echo >&2 "*** code $rust_compiler_code. See output and docs below for details:"
	echo >&2 "***"
	echo >&2 "$rust_compiler_output"
	echo >&2 "***"
	exit 1
fi
rust_compiler_version=$( \
	echo "$rust_compiler_output" \
		| sed -nE '1s:.*rustc ([0-9]+\.[0-9]+\.[0-9]+).*:\1:p'
)
if [ -z "$rust_compiler_version" ]; then
	echo >&2 "***"
	echo >&2 "*** Running '$RUSTC' to check the Rust compiler version did not return"
	echo >&2 "*** an expected output. See output and docs below for details:"
	echo >&2 "***"
	echo >&2 "$rust_compiler_output"
	echo >&2 "***"
	exit 1
fi
rust_compiler_min_version=$($min_tool_version rustc)
rust_compiler_cversion=$(get_canonical_version $rust_compiler_version)
rust_compiler_min_cversion=$(get_canonical_version $rust_compiler_min_version)
if [ "$rust_compiler_cversion" -lt "$rust_compiler_min_cversion" ]; then
	echo >&2 "***"
	echo >&2 "*** Rust compiler '$RUSTC' is too old."
	echo >&2 "***   Your version:    $rust_compiler_version"
	echo >&2 "***   Minimum version: $rust_compiler_min_version"
	echo >&2 "***"
	exit 1
fi

# If the C compiler is Clang, then we can also check whether its version
# matches the `libclang` version used by the Rust bindings generator.
#
# In the future, we might be able to perform a full version check, see
cc_name=$($(dirname $0)/cc-version.sh $CC | cut -f1 -d' ')
if [ "$cc_name" = Clang ]; then
	clang_version=$( \
		LC_ALL=C $CC --version 2>/dev/null \
			| sed -nE '1s:.*version ([0-9]+\.[0-9]+\.[0-9]+).*:\1:p'
	)
fi

# Check that the source code for the `core` standard library exists.
#
# `$KRUSTFLAGS` is passed in case the user added `--sysroot`.
rustc_sysroot=$("$RUSTC" $KRUSTFLAGS --print sysroot)
rustc_src=${RUST_LIB_SRC:-"$rustc_sysroot/lib/rustlib/src/rust/library"}
rustc_src_core="$rustc_src/core/src/lib.rs"
if [ ! -e "$rustc_src_core" ]; then
	echo >&2 "***"
	echo >&2 "*** Source code for the 'core' standard library could not be found"
	echo >&2 "*** at '$rustc_src_core'."
	echo >&2 "***"
	exit 1
fi
