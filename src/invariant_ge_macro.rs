//! A macro to check invariants greater or equal to in debug mode and to optimize them away in release mode.

/// A debug assert macro to check whether an element is greater or equal to than another in debug mode and optimize them away in release mode.
///
/// # Safety
/// Just like [`invariant!`], using this macro in release mode assumes the invariant holds,
/// so make sure to check the condition thoroughly in debug mode.
#[macro_export]
macro_rules! invariant_ge {
    ($left:expr, $right:expr $(,)?) => {
        {
            #[cfg(debug_assertions)]
            {
                if $left < $right {
                    panic!("assertion failed: `(left >= right)`\n  left: `{}`,\n right: `{}`", $left, $right);
                }
            }
            #[cfg(not(debug_assertions))]
            {
                if $left < $right {
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
                if $left < $right {
                    panic!($($arg)+);
                }
            }
            #[cfg(not(debug_assertions))]
            {
                if $left < $right {
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
    fn test_invariant_ge() {
        let a = 1;
        let b = 2;
        invariant_ge!(b, a);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_invariant_ge_panic() {
        let a = 1;
        let b = 2;
        invariant_ge!(a, b);
    }
}
