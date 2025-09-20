//! Math functions

/// const round up
pub const fn div_round_up(n: usize, d: usize) -> usize {
    (n + d - 1) / d
}
