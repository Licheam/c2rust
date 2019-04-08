# Known Limitations of Translation
This document tracks things that we know the translator can't handle, as well as things it probably won't ever handle.

## Unimplemented

  * variadic function definitions and macros that operate on `va_list`s (work in progress)
  * preserving comments (work in progress)
  * `long double` and `_Complex` types (partially blocked by Rust language)
  * Non x86/64 SIMD function/types and x86/64 SIMD function/types which have no rust equivalent

## Unimplemented, _might_ be implementable but very low priority

  * GNU packed structs (Rust has `#[repr(packed)]` compatible with `#[repr(C)]`)
  * `inline` functions (Rust has `#[inline]`)
  * `restrict` pointers (Rust has references)
  * inline assembly
  * macros

## Likely won't ever support

  * __`longjmp`/`setjmp`__ Although there are LLVM intrinsics for these, it is unclear how these interact with Rust (esp. idiomatic Rust).
  * __jumps into and out of statement expressions__ We support GNU C statement expressions, but we can not handle jumping into or out of these. Both entry and exit into the expression have to be through the usual fall-through evaluation of the expression.
