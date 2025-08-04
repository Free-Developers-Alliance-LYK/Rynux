//! ARM64 barrier

macro_rules! dmb_dsb {
    ($A:ident) => {
        impl sealed::Dmb for $A {
            #[inline(always)]
            fn __dmb(&self) {
                unsafe {
                    core::arch::asm!(concat!("DMB ", stringify!($A)), options(nostack))
                }
            }
        }

        impl sealed::Dsb for $A {
            #[inline(always)]
            fn __dsb(&self) {
                unsafe {
                    core::arch::asm!(concat!("DSB ", stringify!($A)), options(nostack))
                }
            }
        }
    };
}

mod sealed {
    pub trait Dmb {
        fn __dmb(&self);
    }

    pub trait Dsb {
        fn __dsb(&self);
    }
}

/// Data Memory Barrier
pub struct SY;
/// Store-Store Barrier
pub struct ST;
/// Load-Load Barrier
pub struct LD;
/// Inner Shareable Barrier
pub struct ISH;
/// Inner Shareable Store-Store Barrier
pub struct ISHST;
/// Inner Shareable Load-Load Barrier
pub struct ISHLD;
/// Non-shareable Barrier
pub struct NSH;
/// Non-shareable Store-Store Barrier
pub struct NSHST;
/// Non-shareable Load-Load Barrier
pub struct NSHLD;
/// Outer Shareable Barrier
pub struct OSH;
/// Outer Shareable Store-Store Barrier
pub struct OSHST;
/// Outer Shareable Load-Load Barrier
pub struct OSHLD;

dmb_dsb!(SY);
dmb_dsb!(ST);
dmb_dsb!(LD);
dmb_dsb!(ISH);
dmb_dsb!(ISHST);
dmb_dsb!(ISHLD);
dmb_dsb!(NSH);
dmb_dsb!(NSHST);
dmb_dsb!(NSHLD);
dmb_dsb!(OSH);
dmb_dsb!(OSHST);
dmb_dsb!(OSHLD);

/// Instruction Synchronization Barrier
#[inline(always)]
pub fn isb()
{
    unsafe { core::arch::asm!("isb", options(nostack)) }
}


/// Data Memory Barrier.
#[inline(always)]
pub fn dmb<A>(arg: A)
where
    A: sealed::Dmb,
{
    arg.__dmb()
}

/// Data Synchronization Barrier.
#[inline(always)]
pub fn dsb<A>(arg: A)
where
    A: sealed::Dsb,
{
    arg.__dsb()
}
