//! A macro to check invariants inequality in debug mode and to optimize them away in release mode.

/// A debug assert macro to check inequality in debug mode and optimize them away in release mode.
///
/// # Implementative details
///
/// [`invariant_ne`] has the same syntax as the [`core::debug_assert_ne`] macro.
/// - On `debug` mode, i.e. when `debug_assertions` are enabled, it will call [`core::debug_assert_ne`].
/// - On `release` mode it will call [`core::hint::unreachable_unchecked`] if the inequality does not hold.
///
/// While [`core::debug_assert_ne`] will be entirely removed in release mode, the
/// [`invariant_ne`] macro will be replaced by a call to [`core::hint::unreachable_unchecked`] when the condition is not met.
/// This informs the compiler that the code path is unreachable, leading to potential optimization
/// and improved performance.
///
/// # Safety
/// Just like [`invariant!`], using this macro in release mode assumes the invariant holds,
/// so make sure to check the condition thoroughly in debug mode.
#[macro_export]
macro_rules! invariant_ne {
    ($left:expr, $right:expr $(,)?) => {
        {
            debug_assert_ne!($left, $right);
            #[cfg(not(debug_assertions))]
            {
                if $left == $right {
                    unsafe {
                        core::hint::unreachable_unchecked();
                    }
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        {
            debug_assert_ne!($left, $right, $($arg)+);
            #[cfg(not(debug_assertions))]
            {
                if $left == $right {
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
    fn test_invariant_ne() {
        let a = 1;
        let b = 2;
        invariant_ne!(a, b);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_invariant_ne_panic() {
        let a = 1;
        let b = 1;
        invariant_ne!(a, b);
    }
}
