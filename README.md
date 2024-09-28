# Invariant-rs
Rust macros for invariant assertions, allowing for the definition of axiomatic conditions that are necessarily always true at a particular point in a program, checked in debug mode and exploited by the compiler as certainly true in release mode to optimize the code.

## What is an invariant?
An invariant is a axiomatic condition that is always true at a particular point in a program. While we generally want to test for such conditions during debug mode, as one never knows whether the code is actually correct, we want to inform the compiler that the condition can be assumed to be true in release mode. This is where macros such as `invariant!` comes in. Take the following examples:

```rust
use invariant_rs::invariant_ne;

#[inline(never)]
pub fn my_log1(x: usize) -> u32 {
    debug_assert_ne!(x, 0);
    x.ilog2()
}

#[inline(never)]
pub fn my_log2(x: usize) -> u32 {
    invariant_ne!(x, 0);
    x.ilog2()
}
```

The first method, `my_log1`, will be compiled in relase as:

```assembly
my_log1:
    test    rdi, rdi
    je      .LBB0_2
    bsr     rax, rdi
    ret
.LBB0_2:
    push    rax
    lea     rdi, [rip + .L__unnamed_1]
    call    qword ptr [rip + panic_for_nonpositive_argument]
```

While the second method, `my_log2`, will be compiled to the much simpler:

```assembly
my_log2:
    bsr     rax, rdi
    ret
```

The second method is identical to:
    
```rust
#[inline(never)]
pub fn my_log2(x: usize) -> u32 {
    debug_assert_ne!(x, 0);
    if x == 0 {
        unsafe {
            core::hint::unreachable_unchecked();
        }
    }
    x.ilog2()
}
```

Note that the compile directive `#[inline(never)]` is used to prevent the compiler from inlining the method, which would make the assembly code harder to read. The `invariant_ne!` macro is used to inform the compiler that the condition `x != 0` is always true, and thus the compiler can optimize the code accordingly.

## Usage

You can use the `invariant!` macro to add assertions to your code:

```rust
use invariant_rs::invariant;

let x = 0;
invariant!(x == 0);
```

Similarly, the crate offers the `invariant_eq!` macro to check for equality:

```rust
use invariant_rs::invariant_eq;

let x = 0;
invariant_eq!(x, 0);
```

And the `invariant_ne!` macro to check for inequality:

```rust
use invariant_rs::invariant_ne;

let x = 0;
invariant_ne!(x, 1);
```

And the `invariant_lt!` macro to check for less than:

```rust
use invariant_rs::invariant_lt;

let x = 0;
invariant_lt!(x, 1);
```

And the `invariant_le!` macro to check for less than or equal:

```rust
use invariant_rs::invariant_le;

let x = 0;
invariant_le!(x, 0);
```

And the `invariant_gt!` macro to check for greater than:

```rust
use invariant_rs::invariant_gt;

let x = 1;
invariant_gt!(x, 0);
```

And the `invariant_ge!` macro to check for greater than or equal:

```rust
use invariant_rs::invariant_ge;

let x = 0;
invariant_ge!(x, 0);
```

## What happens if unchecked unreacheable code is reached?
If the `debug_assertions` are enabled, the program will panic. If the `debug_assertions` are disabled, the compiler may place a [`ud2` instruction](https://en.wikipedia.org/wiki/Illegal_opcode), a type of illegal instruction, which will cause the program to crash. In some other cases, the code may not have any apparent effect, **which is why you should use test the condition thoroughly in debug mode!**

### Example generating a `ud2` instruction in release mode
If you try to compile the following code in release mode, the compiler will generate a `ud2` instruction, which will cause the program to crash.

```no_run
use invariant_rs::invariant;

let x = 0;
invariant!(x == 1);
```

The assembly code it generates is simply:

```x86asm
main:
    ud2
```

## No STD
This crate is `no_std` compatible, and can be used in embedded systems.

## Contributing
Would you like to add some more assertions to this collection? Open up an issue and let's discuss it!

## License
This crate is released under the MIT license. You can find the complete text in the [`LICENSE`](https://github.com/LucaCappelletti94/invariant-rs/blob/main/LICENSE) file.