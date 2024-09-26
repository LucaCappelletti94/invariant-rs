//! A macro to check invariants equality in debug mode and to optimize them away in release mode.

/// A debug assert macro to check equality in debug mode and optimize them away in release mode.
///
/// # Implementative details
///
/// [`invariant_eq`] has the same syntax as the [`core::debug_assert_eq`] macro.
/// - On `debug` mode, i.e. when `debug_assertions` are enabled, it will call [`core::debug_assert_eq`].
/// - On `release` mode it will call [`core::hint::unreachable_unchecked`] if the equality does not hold.
///
/// While [`core::debug_assert_eq`] will be entirely removed in release mode, the
/// [`invariant_eq`] macro will be replaced by a call to [`core::hint::unreachable_unchecked`] when the condition is not met.
/// This informs the compiler that the code path is unreachable, leading to potential optimization
/// and improved performance.
///
/// # Example
///
/// The following code will panic in debug mode if `x` and `y` are not equal, and will be optimized away in release mode.
/// Note that we add the `#[inline(never)]` attribute to prevent the compiler from inlining the functions, so we can see
/// clearly the generated assembly code.
///
/// ```rust
/// use invariant_rs::invariant_eq;
///
/// #[inline(never)]
/// pub fn test_eq1(x: usize, y: usize) -> usize {
///     x + y
/// }
///
/// #[inline(never)]
/// pub fn test_eq2(x: usize, y: usize) -> usize {
///     invariant_eq!(x, y, "x and y must be equal");
///     x + y
/// }
/// ```
///
/// It means that, while the function `test_eq1` without the invariant check will look like this:
///
/// ```x86asm
/// test_eq1:
///     lea     eax, [rdi + rsi]
///     ret
/// ```
///
/// When the invariant check is added, `test_eq2`, the generated assembly code will be:
///
/// ```x86asm
/// test_eq2:
///     lea     eax, [rdi + rdi]
///     ret
/// ```
///
/// # Safety
/// Just like [`invariant!`], using this macro in release mode assumes the invariant holds,
/// so make sure to check the condition thoroughly in debug mode.
#[macro_export]
macro_rules! invariant_eq {
    ($left:expr, $right:expr $(,)?) => {
        {
            debug_assert_eq!($left, $right);
            #[cfg(not(debug_assertions))]
            {
                if $left != $right {
                    unsafe {
                        core::hint::unreachable_unchecked();
                    }
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        {
            debug_assert_eq!($left, $right, $($arg)+);
            #[cfg(not(debug_assertions))]
            {
                if $left != $right {
                    unsafe {
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
    fn test_invariant_eq() {
        invariant_eq!(1, 1);
        invariant_eq!(1, 1, "1 must be equal to 1");
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_invariant_eq_panic() {
        invariant_eq!(1, 2);
    }
}
