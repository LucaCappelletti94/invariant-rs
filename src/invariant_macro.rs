//! Support for invariants in debug mode and optimized away in release mode.

/// An assert macro to check invariants in debug mode and to optimize them away in release mode.
///
/// # Implementative details
///
/// [`invariant`] has the same syntax as the [`core::debug_assert`] macro.
/// - On `debug` mode, i.e. when `debug_assertions are` enabled, it will call [`core::debug_assert`].
/// - On `release` mode it will call [`core::hint::unreachable_unchecked`].
///
/// While [`core::debug_assert`] will be entirely removed in release mode, the
/// [`invariant`] macro will be replaced by a call to [`core::hint::unreachable_unchecked`].
/// Such hint will be employed to inform the compiler that the code path is unreachable,
/// and thus the compiler can optimize the code accordingly, often leading to less
/// code being generated and overall better performance.
///
/// # Example
///
/// In the following code, we have two functions, `test1` and `test2`, that calculate the integer
/// logarithm base 2 of a given number. The first function does not check if the input is positive,
/// while the second one does. Note that we add the `#[inline(never)]` attribute to prevent the
/// compiler from inlining the functions, so we can see clearly the generated assembly code.
///
/// ```rust
/// use invariant_rs::invariant;
///
/// #[inline(never)]
/// pub fn test1(x: usize) -> u32 {
///     x.ilog2()
/// }
///
/// #[inline(never)]
/// pub fn test2(x: usize) -> u32 {
///     invariant!(x > 0, "x must be positive");
///     x.ilog2()
/// }
/// ```
///
/// The first function, `test1`,  will generate the following assembly code:
///
/// ```x86asm
/// test1:
///     test    rdi, rdi
///     je      .LBB0_2
///     bsr     rax, rdi
///     ret
/// .LBB0_2:
///     push    rax
///     lea     rdi, [rip + .L__unnamed_1]
///     call    qword ptr [rip + core::num::int_log10::panic_for_nonpositive_argument::h3a8d3f879c6e5198@GOTPCREL]
/// ```
///
/// While the second function, `test2`, will generate the following simpler assembly code:
///
/// ```x86asm
/// bsr     rax, rdi
/// ret
/// ```
///
/// # Safety
/// Using this macro in release mode assumes the invariant holds, so make sure to check the condition thoroughly in debug mode.
#[macro_export]
macro_rules! invariant {
    ($cond:expr $(,)?) => {
        {
            debug_assert!($cond);
            #[cfg(not(debug_assertions))]
            {
                if !($cond) {
                    unsafe{
                        core::hint::unreachable_unchecked();
                    }
                }
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        {
            debug_assert!($cond, $($arg)+);
            #[cfg(not(debug_assertions))]
            {
                if !($cond) {
                    unsafe{
                        core::hint::unreachable_unchecked();
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_invariant() {
        invariant!(true);
        invariant!(true, "true must be true");
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_invariant_panic() {
        invariant!(false);
    }
}
