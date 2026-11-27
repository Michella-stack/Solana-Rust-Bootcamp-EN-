//! ex01 - Variables and types.
//!
//! Read `tasks/ex01.md` first. Replace each `todo!()` with an implementation.

/// The number of lamports in one SOL.
///
/// In a real Solana program this constant comes from
/// `solana_program::native_token::LAMPORTS_PER_SOL`. Here we define it locally
/// so the exercise has no dependencies.
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

/// Convert whole SOL into lamports.
///
/// `sol_to_lamports(2)` should return `2_000_000_000`.
pub fn sol_to_lamports(sol: u64) -> u64 {
    // LEARNING NOTE: both `sol` and `LAMPORTS_PER_SOL` are `u64`, so Rust lets
    // us multiply them directly. Rust never mixes integer types silently; a
    // `u32 * u64` would be a compile error (E0308) until you convert one side.
    //
    // The last expression in a function, with no semicolon, is its return
    // value. Adding `;` would turn it into a statement that returns `()`.
    sol * LAMPORTS_PER_SOL
}

/// Convert lamports into whole SOL, dropping any remainder.
///
/// `lamports_to_sol(2_500_000_000)` should return `2`.
pub fn lamports_to_sol(lamports: u64) -> u64 {
    // LEARNING NOTE: `/` between two integers is integer division. It throws
    // away the remainder instead of rounding: 5_500_000_000 / 1_000_000_000
    // is 5, not 5.5. That truncation is exactly what "whole SOL" means.
    // (The leftover lamports would be `lamports % LAMPORTS_PER_SOL`.)
    lamports / LAMPORTS_PER_SOL
}

/// Add a bonus to a balance and return the new balance.
pub fn apply_bonus(balance: u64, bonus: u64) -> u64 {
    // LEARNING NOTE: parameters are immutable bindings, like `let`. We do not
    // change `balance`; we compute a brand new value and return it. The caller
    // decides whether to store it.
    balance + bonus
}

/// Double a value.
///
/// Use shadowing in your implementation: declare a new `value` instead of
/// mutating the parameter.
pub fn double(value: u64) -> u64 {
    // LEARNING NOTE: this is SHADOWING, not mutation. `let value = ...` makes a
    // second, new variable that happens to reuse the name `value`. The
    // parameter itself was never declared `mut`, and we never change it.
    //
    // Compare:
    //   let mut x = 1; x = x * 2;   // mutation: same variable, new contents
    //   let x = 1; let x = x * 2;   // shadowing: a new variable, old name
    // Shadowing can even change the type (`let s = s.len();`); `mut` cannot.
    let value = value * 2;
    value
}

/// Return `true` when the balance is zero.
pub fn is_empty(balance: u64) -> bool {
    // LEARNING NOTE: `==` already produces a `bool`, so there is no need for
    // `if balance == 0 { true } else { false }`.
    //
    // Why `u64` for money and not `f64`? A `u64` can never be negative and is
    // exact: 0.1 + 0.2 is not 0.3 in floating point, which is unacceptable for
    // balances. Solana stores every balance as an integer count of lamports.
    balance == 0
}
