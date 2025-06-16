# SPDX-License-Identifier: GPL-2.0
###
# scripts contains sources for various helper programs used throughout
# the kernel for the build process.

hostprogs-always-$(CONFIG_BUILDTIME_TABLE_SORT)		+= sorttable
hostprogs-always-$(CONFIG_RUST_KERNEL_DOCTESTS)		+= rustdoc_test_builder
hostprogs-always-$(CONFIG_RUST_KERNEL_DOCTESTS)		+= rustdoc_test_gen

hostprogs += generate_rust_target
generate_rust_target-rust := y
rustdoc_test_builder-rust := y
rustdoc_test_gen-rust := y

HOSTCFLAGS_sorttable.o = -I$(srctree)/tools/include
HOSTLDLIBS_sorttable = -lpthread

# Let clean descend into subdirs
subdir-	+= basic dtc kconfig mod
