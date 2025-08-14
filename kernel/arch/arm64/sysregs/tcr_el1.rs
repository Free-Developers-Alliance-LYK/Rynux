/// TCR_ELx
use crate::{
    cfg_if,
    bitflags::bitflags,
    arch::arm64::sysregs::{
        midr_el1,MidrEl1,
        id_aa64mmfr0_el1,IdAa64mmfr0El1,
    },
};

bitflags!{
    /// TCR flags
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct Tcr: u64 {
        /// DS
        const DS = 1 << 59;
        /// TCMA1
        const TCMA1 = 1 << 58;
        /// TCMA0
        const TCMA0 = 1 << 57;
        /// E0PD1
        const E0PD1 = 1 << 56;
        /// E0PD0
        const E0PD0 = 1 << 55;
        /// NFD1
        const NFD1 = 1 << 54;
        /// NFD0
        const NFD0 = 1 << 53;
        /// TBID1
        const TBID1 = 1 << 52;
        /// TBID0
        const TBID0 = 1 << 51;
        /// HPD1
        const HPD1 = 1 << 42;
        /// HPD0
        const HPD0 = 1 << 41;
        /// HD
        const HD = 1 << 40;
        /// HA
        const HA = 1 << 39;
        /// TBI1
        const TBI1 = 1 << 38;
        /// TBI0
        const TBI0 = 1 << 37;
        /// AS
        const AS = 1 << 36;
        /// RES1
        const RES1 = 1 << 35;
        /// IPS
        const IPS = 0b111 << 32;
        /// TG1
        const TG1 = 0b11 << 30;
        /// SH1
        const SH1 = 0b11 << 28;
        /// ORGN1
        const ORGN1 = 0b11 << 26;
        /// IRGN1
        const IRGN1 = 0b11 << 24;
        /// EPD1
        const EPD1 = 1 << 23;
        /// A1
        const A1 = 1 << 22;
        /// T1S1
        const T1SZ = 0b11_1111 << 16;
        /// TG0
        const TG0 = 0b11 << 14;
        /// SH0
        const SH0 = 0b11 << 12;
        /// ORGNO
        const ORGNO = 0b11 << 10;
        /// IRGN0
        const IRGNO = 0b11 << 8;
        /// EPD0
        const EPD0 = 1 << 7;
        /// RES0
        const RES0 = 0b1 << 6;
        /// T0SZ
        const T0SZ = 0b11_1111 << 0;
    }
}

// Inner cacheability attribute for memory associated
#[allow(dead_code)]
#[derive(Copy, Clone)]
enum IRGN {
    /// Normal memory, Inner Non-cacheable.
    NC = 0,
    /// Normal memory, Inner Write-Back Read-Allocate Write-Allocate Cacheable.
    WBWA = 1,
    /// Normal memory, Inner Write-Through Read-Allocate No Write-Allocate Cacheable.
    WT = 2,
    /// Normal memory, Inner Write-Back Read-Allocate No Write-Allocate Cacheable.
    WBnWA = 3,
}

impl IRGN {
    const IRGN0_OFFSET: u64 = 8;
    const IRGN1_OFFSET: u64 = 24;

    #[inline(always)]
    const fn irgn0(x: IRGN) -> u64 {
        (x as u64) << Self::IRGN0_OFFSET
    }

    #[inline(always)]
    const fn irgn1(x: IRGN) -> u64 {
        (x as u64) << Self::IRGN1_OFFSET
    }

    #[inline(always)]
    const fn irgnx(x: IRGN) -> u64 {
        Self::irgn0(x) | Self::irgn1(x)
    }
}

// Outer cacheability attribute for memory associated 
#[allow(dead_code)]
#[derive(Copy, Clone)]
enum ORGN {
    /// Normal memory, Outer Non-cacheable.
    NC = 0,
    /// Normal memory, Outer Write-Back Read-Allocate Write-Allocate Cacheable.
    WBWA = 1,
    /// Normal memory, Outer Write-Through Read-Allocate No Write-Allocate Cacheable.
    WT = 2,
    /// Normal memory, Outer Write-Back Read-Allocate No Write-Allocate Cacheable.
    WBnWA = 3,
}

impl ORGN {
    const ORGN0_OFFSET: u64 = 10;
    const ORGN1_OFFSET: u64 = 26;

    #[inline(always)]
    const fn orgn0(x: ORGN) -> u64 {
        (x as u64) << Self::ORGN0_OFFSET
    }

    #[inline(always)]
    const fn orgn1(x: ORGN) -> u64 {
        (x as u64) << Self::ORGN1_OFFSET
    }

    #[inline(always)]
    const fn orgnx(x: ORGN) -> u64 {
        Self::orgn0(x) | Self::orgn1(x)
    }
}

// Shareability attribute for memory associated
#[allow(dead_code)]
#[derive(Copy, Clone)]
enum SharedFlags {
    /// Non-shareable
    NON = 0,
    /// Inner shareable
    INNER = 1,
    /// Outer shareable
    OUTER = 2,
}

impl SharedFlags {
    const SH0_OFFSET: u64 = 12;
    #[inline(always)]
    const fn sh0(x: SharedFlags) -> u64 {
        (x as u64) << Self::SH0_OFFSET
    }

    const SH1_OFFSET: u64 = 28;
    #[inline(always)]
    const fn sh1(x: SharedFlags) -> u64 {
        (x as u64) << Self::SH1_OFFSET
    }

    #[inline(always)]
    const fn shx(x: SharedFlags) -> u64 {
        Self::sh0(x) | Self::sh1(x)
    }
}

// Translation granule size
#[derive(Copy, Clone)]
enum TG {
    /// 4KB
    _4K = 0,
    /// 16KB
    _16K = 1,
    /// 64KB
    _64K = 2,
}

impl TG {
    const TG0_OFFSET: u64 = 14;
    const TG1_OFFSET: u64 = 30;

    #[inline(always)]
    const fn tg0(x: TG) -> u64 {
        (x as u64) << Self::TG0_OFFSET
    }

    #[inline(always)]
    const fn tg1(x: TG) -> u64 {
        (x as u64) << Self::TG1_OFFSET
    }

    #[inline(always)]
    const fn tgx(x: TG) -> u64 {
        Self::tg0(x) | Self::tg1(x)
    }
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum IPS {
    _32_BIT = 0b000,
    _36_BIT = 0b001,
    _40_BIT = 0b010,
    _42_BIT = 0b011,
    _44_BIT = 0b100,
    _48_BIT = 0b101,
    _52_BIT = 0b110,
    _56_BIT = 0b111,
}

impl From<id_aa64mmfr0_el1::PaRange> for IPS {
    fn from(parange: id_aa64mmfr0_el1::PaRange) -> Self {
        match parange {
            id_aa64mmfr0_el1::PaRange::Bits32 => IPS::_32_BIT,
            id_aa64mmfr0_el1::PaRange::Bits36 => IPS::_36_BIT,
            id_aa64mmfr0_el1::PaRange::Bits40 => IPS::_40_BIT,
            id_aa64mmfr0_el1::PaRange::Bits42 => IPS::_42_BIT,
            id_aa64mmfr0_el1::PaRange::Bits44 => IPS::_44_BIT,
            id_aa64mmfr0_el1::PaRange::Bits48 => IPS::_48_BIT,
            id_aa64mmfr0_el1::PaRange::Bits52 => IPS::_52_BIT,
            id_aa64mmfr0_el1::PaRange::Bits56 => IPS::_56_BIT,
        }
    }
}

impl IPS {
    const IPS_BITS_OFFSET: u64 = 32;
    #[inline(always)]
    fn to_tcr(self) -> u64 {
        (self as u64) << Self::IPS_BITS_OFFSET
    }
}

#[allow(dead_code)]
impl Tcr {
    /// PTWs cacheable, inner/outer WBWA
    pub const CACHE_FLAGS: u64 = IRGN::irgnx(IRGN::WBWA) | ORGN::orgnx(ORGN::WBWA);
    /// Inner shareable
    pub const SHARED: u64 = SharedFlags::shx(SharedFlags::INNER);

    cfg_if! {
        if #[cfg(CONFIG_ARM64_64K_PAGES)] {
            /// 64KB granule
            pub const TG_FLAGS: u64 = TG::tgx(TG::_64K);
        } else if #[cfg(CONFIG_ARM64_16K_PAGES)] {
            /// 16KB granule
            pub const TG_FLAGS: u64 = TG::tgx(TG::_16K);
        } else {
            /// 4KB granule
            pub const TG_FLAGS: u64 = TG::tgx(TG::_4K);
        }
    }

    /// Read TCR_EL1 register.
    #[inline(always)]
    pub fn read() -> Self {
        let tcr: u64;
        sys_coproc_read_raw!(u64, "TCR_EL1", "x", tcr);
        Self::from_bits_truncate(tcr)
    }

    /// WRITE TCR_EL1 register.
    #[inline(always)]
    pub fn write(&self) {
        let tcr = self.bits();
        sys_coproc_write_raw!(u64, "TCR_EL1", "x", tcr);
    }

    const CLEAR_FUJITSU_ERRATUM_010001: Self = Self::from_bits_truncate(Tcr::NFD1.bits() | Tcr::NFD0.bits());
    const MIDR_FUJITSU_ERRATUM_010001: u64 = midr_el1::FUJITSU_A64FX.bits();
    // Fujitsu Erratum 010001 affects A64FX 1.0 and 1.1, (v0r0 and v1r0)
    const MIDR_FUJITSU_ERRATUM_010001_MASK: u64 = !MidrEl1::cpu_var_rev(1, 0);

    /// Clear TCR bits that trigger an errata on this CPU
    #[inline(always)]
    pub fn clear_errata_bits(&mut self) {
        #[cfg(CONFIG_FUJITSU_ERRATUM_010001)] {
            let midr = MidrEl1::read_raw() & Self::MIDR_FUJITSU_ERRATUM_010001_MASK;
            if midr ==  Self::MIDR_FUJITSU_ERRATUM_010001 {
                self.remove(Self::CLEAR_FUJITSU_ERRATUM_010001);
            }
        }
    }
    
    const T0SZ_OFFSET: u64 = 0;
    /// T0SZ
    #[inline(always)]
    pub const fn t0sz(x: u64) -> u64 {
        (64 - x) << Self::T0SZ_OFFSET
    }

    const T1SZ_OFFSET: u64 = 16;
    /// T1SZ
    #[inline(always)]
    pub const fn t1sz(x: u64) -> u64 {
        (64 - x) << Self::T1SZ_OFFSET
    }

    /// TxSZ
    #[inline(always)]
    pub const fn txsz(x: u64) -> u64 {
        Self::t0sz(x) | Self::t1sz(x)
    }

    #[inline(always)]
    fn set_ips(&mut self, ips: IPS) {
        // clear
        self.remove(Self::IPS);
        *self = Self::from_bits_truncate(self.bits() | ips.to_tcr())  
    }

    /// Compute the maximum physical address size supported by the system
    #[inline(always)]
    pub fn compute_set_pa_size(&mut self) {
        let parange = IdAa64mmfr0El1::read().parange();
        self.set_ips(IPS::from(parange));
    }
}
