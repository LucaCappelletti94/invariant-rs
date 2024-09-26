//! A macro to check invariants greater than in debug mode and to optimize them away in release mode.

/// A debug assert macro to check whether an element is greater than than another in debug mode and optimize them away in release mode.
///
/// # Safety
/// Just like [`invariant!`], using this macro in release mode assumes the invariant holds,
/// so make sure to check the condition thoroughly in debug mode.
#[macro_export]
macro_rules! invariant_gt {
    ($left:expr, $right:expr $(,)?) => {
        {
            #[cfg(debug_assertions)]
            {
                if $left <= $right {
                    panic!("assertion failed: `(left > right)`\n  left: `{}`,\n right: `{}`", $left, $right);
                }
            }
            #[cfg(not(debug_assertions))]
            {
                if $left <= $right {
                    unsafe {
                        core::hint::unreachable_unchecked();
                    }
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        {
            #[cfg(debug_assertions)]
            {
                if $left <= $right {
                    panic!($($arg)+);
                }
            }
            #[cfg(not(debug_assertions))]
            {
                if $left <= $right {
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
    fn test_invariant_gt() {
        let a = 2;
        let b = 1;
        invariant_gt!(a, b);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_invariant_gt_panic() {
        let a = 1;
        let b = 2;
        invariant_gt!(a, b);
    }
}
