#!/bin/sh
# SPDX-License-Identifier: GPL-2.0
#
# link vmrynux
#
# vmrynux is linked from the objects in vmrynux.a and $(KBUILD_VMLINUX_LIBS).
# vmrynux.a contains objects that are linked unconditionally.
# $(KBUILD_VMLINUX_LIBS) are archives which are linked conditionally
# (not within --whole-archive), and do not require symbol indexes added.
#
# vmrynux
#   ^
#   |
#   +--< vmrynux.a
#   |
#   +--< $(KBUILD_VMLINUX_LIBS)
#   |    +--< lib/lib.a + more
#   |
#   +-< ${kallsymso} (see description in KALLSYMS section)
#
# vmrynux version (uname -v) cannot be updated during normal
# descending-into-subdirs phase since we do not yet know if we need to
# update vmrynux.
# Therefore this step is delayed until just before final link of vmrynux.
#
# System.map is generated to document addresses of all kernel symbols

# Error out on error
set -e

LD="$1"
KBUILD_LDFLAGS="$2"
LDFLAGS_vmrynux="$3"

is_enabled() {
	grep -q "^$1=y" include/config/auto.conf
}

# Nice output in kbuild format
# Will be supressed by "make -s"
info()
{
	printf "  %-7s %s\n" "${1}" "${2}"
}

# Link of vmrynux
# ${1} - output file
vmrynux_link()
{
	local output=${1}
	local objs
	local libs
	local ld
	local ldflags
	local ldlibs

	info LD ${output}

	# skip output file argument
	shift

	objs=vmrynux.o
	libs=

	objs="${objs} init/version-timestamp.o"

	wl=
	ld="${LD}"
	ldflags="${KBUILD_LDFLAGS} ${LDFLAGS_vmrynux}"
	ldlibs=

	ldflags="${ldflags} ${wl}--script=${objtree}/${KBUILD_LDS}"

	# The kallsyms linking does not need debug symbols included.
	if [ -n "${strip_debug}" ] ; then
		ldflags="${ldflags} ${wl}--strip-debug"
	fi

	if is_enabled CONFIG_VMLINUX_MAP; then
		ldflags="${ldflags} ${wl}-Map=${output}.map"
	fi

	${ld} ${ldflags} -o ${output}					\
		${wl}--whole-archive ${objs} ${wl}--no-whole-archive	\
		${wl}--start-group ${libs} ${wl}--end-group		\
		${kallsymso} ${ldlibs}
}

# Create ${2}.o file with all symbols from the ${1} object file
kallsyms()
{
	local kallsymopt;

	kallsymopt="${kallsymopt} --all-symbols"

	info KSYMS "${2}.S"
	scripts/kallsyms ${kallsymopt} "${1}" > "${2}.S"

	info AS "${2}.o"
	${CC} ${NOSTDINC_FLAGS} ${LINUXINCLUDE} ${KBUILD_CPPFLAGS} \
	      ${KBUILD_AFLAGS} ${KBUILD_AFLAGS_KERNEL} -c -o "${2}.o" "${2}.S"

	kallsymso=${2}.o
}

# Perform kallsyms for the given temporary vmrynux.
sysmap_and_kallsyms()
{
	mksysmap "${1}" "${1}.syms"
	kallsyms "${1}.syms" "${1}.kallsyms"

	kallsyms_sysmap=${1}.syms
}

# Create map file with all symbols from ${1}
# See mksymap for additional details
mksysmap()
{
	info NM ${2}
	${NM} -n "${1}" | sed -f "${srctree}/scripts/mksysmap" > "${2}"
}

sorttable()
{
	${objtree}/scripts/sorttable ${1}
}

cleanup()
{
	rm -f System.map
	rm -f vmrynux
	rm -f vmrynux.map
}

# Use "make V=1" to debug this script
case "${KBUILD_VERBOSE}" in
*1*)
	set -x
	;;
esac

if [ "$1" = "clean" ]; then
	cleanup
	exit 0
fi

${MAKE} -f "${srctree}/scripts/Makefile.build" obj=init init/version-timestamp.o

kallsymso=
strip_debug=

if is_enabled CONFIG_KALLSYMS; then
	true > .tmp_vmrynux.kallsyms0.syms
	kallsyms .tmp_vmrynux.kallsyms0.syms .tmp_vmrynux0.kallsyms
fi

if is_enabled CONFIG_KALLSYMS; then
	vmrynux_link .tmp_vmrynux1
fi

if is_enabled CONFIG_KALLSYMS; then

	# kallsyms support
	# Generate section listing all symbols and add it into vmrynux
	# It's a four step process:
	# 0)  Generate a dummy __kallsyms with empty symbol list.
	# 1)  Link .tmp_vmrynux.kallsyms1 so it has all symbols and sections,
	#     with a dummy __kallsyms.
	#     Running kallsyms on that gives us .tmp_kallsyms1.o with
	#     the right size
	# 2)  Link .tmp_vmrynux.kallsyms2 so it now has a __kallsyms section of
	#     the right size, but due to the added section, some
	#     addresses have shifted.
	#     From here, we generate a correct .tmp_vmrynux.kallsyms2.o
	# 3)  That link may have expanded the kernel image enough that
	#     more linker branch stubs / trampolines had to be added, which
	#     introduces new names, which further expands kallsyms. Do another
	#     pass if that is the case. In theory it's possible this results
	#     in even more stubs, but unlikely.
	#     KALLSYMS_EXTRA_PASS=1 may also used to debug or work around
	#     other bugs.
	# 4)  The correct ${kallsymso} is linked into the final vmrynux.
	#
	# a)  Verify that the System.map from vmrynux matches the map from
	#     ${kallsymso}.

	# The kallsyms linking does not need debug symbols included.
	strip_debug=1

	sysmap_and_kallsyms .tmp_vmrynux1
	size1=$(${CONFIG_SHELL} "${srctree}/scripts/file-size.sh" ${kallsymso})

	vmrynux_link .tmp_vmrynux2
	sysmap_and_kallsyms .tmp_vmrynux2
	size2=$(${CONFIG_SHELL} "${srctree}/scripts/file-size.sh" ${kallsymso})

	if [ $size1 -ne $size2 ] || [ -n "${KALLSYMS_EXTRA_PASS}" ]; then
		vmrynux_link .tmp_vmrynux3
		sysmap_and_kallsyms .tmp_vmrynux3
	fi
fi

strip_debug=

vmrynux_link vmrynux

mksysmap vmrynux System.map

if is_enabled CONFIG_BUILDTIME_TABLE_SORT; then
	info SORTTAB vmrynux
	if ! sorttable vmrynux; then
		echo >&2 Failed to sort kernel tables
		exit 1
	fi
fi

# step a (see comment above)
if is_enabled CONFIG_KALLSYMS; then
	if ! cmp -s System.map "${kallsyms_sysmap}"; then
		echo >&2 Inconsistent kallsyms data
		echo >&2 'Try "make KALLSYMS_EXTRA_PASS=1" as a workaround'
		exit 1
	fi
fi

# For fixdep
echo "vmrynux: $0" > .vmrynux.d
