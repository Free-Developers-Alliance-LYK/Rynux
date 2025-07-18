//! ARM64 system registers

macro_rules! __read_raw {
    ($width:ty, $asm_instr:tt, $asm_reg_name:tt, $asm_width:tt, $out:ident) => {
        unsafe {
            core::arch::asm!(
                concat!($asm_instr, " {out:", $asm_width, "}, ", $asm_reg_name),
                out = out(reg) $out,
                options(nomem, nostack));
        }
    };
}

macro_rules! __write_raw {
    ($width:ty, $asm_instr:tt, $asm_reg_name:tt, $asm_width:tt, $value:ident) => {
        unsafe {
            core::arch::asm!(
                concat!($asm_instr, " ", $asm_reg_name, ", {in_reg:", $asm_width, "}"),
                in_reg = in(reg) $value,
                options(nomem, nostack))
        }
    };
}

/// Raw read from system coprocessor registers.
macro_rules! sys_coproc_read_raw {
    ($width:ty, $asm_reg_name:tt, $asm_width:tt, $out:ident) => {
        __read_raw!($width, "mrs", $asm_reg_name, $asm_width, $out);
    };
}

/// Raw write to system coprocessor registers.
macro_rules! sys_coproc_write_raw {
    ($width:ty, $asm_reg_name:tt, $asm_width:tt, $in:ident) => {
        __write_raw!($width, "msr", $asm_reg_name, $asm_width, $in);
    };
}

/// Raw read from (ordinary) registers.
macro_rules! read_raw {
    ($width:ty, $asm_reg_name:tt, $asm_width:tt, $out:ident) => {
        __read_raw!($width, "mov", $asm_reg_name, $asm_width, $out);
    };
}
/// Raw write to (ordinary) registers.
macro_rules! write_raw {
    ($width:ty, $asm_reg_name:tt, $asm_width:tt, $in:ident) => {
        __write_raw!($width, "mov", $asm_reg_name, $asm_width, $in);
    };
}


/// Read from general purpose registers.
#[macro_export]
macro_rules! read_gpr {
    ($reg:ident) => {{
        let val: u64;
        unsafe {
            core::arch::asm!(
                concat!("mov {out}, ", stringify!($reg)),
                out(reg) val
            );
        }
        val
    }};
}

/// Write to general purpose registers.
#[macro_export]
macro_rules! write_gpr {
    ($reg:ident, $value:expr) => {{
        unsafe {
            core::arch::asm!(
                concat!("mov ", stringify!($reg), ", {inval}"),
                inval = in(reg) $value
            );
        }
    }};
}
