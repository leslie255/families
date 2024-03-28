# Families

Rust HKT implementations for playing around, with derive macro for automatically declaring/implementing type families.

## What is HKT?

See [Generalizing over Generics in Rust (Part 1) - AKA Higher Kinded Types in Rust](https://rustyyato.github.io/type/system,type/families/2021/02/15/Type-Families-1.html) for an explanation of HKT in Rust syntax

Although in the honest opinion of the author, dipping your toes into Haskell for a bit really helps understanding these concepts, as the traditional generic syntax of Rust (and many other langauges) often hides important patterns regarding understanding of FP concepts.

## Overview

The repo provides a `Family` trait for generalizing over type functions (i.e. a generic type with one parameter).

A derive macro `#[derive(Family)]` exists for automatically deriving implementation of a type families for a generic ADT (i.e. struct, enum, union).

The repo also provides type family implementation of some common ADT's in the Rust STD.

An typical type family implementation for a generic type looks like this:

```rs
struct OneParam<T>(T);
struct OneParamFamily;
impl Family for OneParamFamily {
  type This<T> = OneParam<T>;
}
```

The above is (almost) the exact same code generated by:

```rs
#[derive(Family)]
struct OneParam<T>(T);
```

A type function could then be invoked by:

```rs
<OneParamFamily as Family>::This<T>
```

Although under most context the shorthand form produces the same result:

```rs
OneParamFamily::This<T>
```

A multi-parameter generic ADT can also be implemented as type families via curried type functions:

```rs
#[derive(Family)]
struct TwoParam<T, U>(T, U);
```

This generates:

```rs
struct TwoParam<T, U>(T, U);
struct TwoParamFamily<T>(PhantomData<T>);
impl Family for TwoParamFamily {
  type This<U> = TwoParam<T, U>,
}
struct TwoParamFamilyFamily;
impl Family for TwoParamFamilyFamily {
  type This<T> = TwoParamFamily<T>,
}
```

## Unresolved questions

- Fitting lifetimes and const parameters into the system.
- Generic constraints in type families (large sections of the code in `typefamilies-derive/src/lib.rs` are commented out due to a failed attempt at implementing generic constraints).
- Somehow, relating monads with `Try` trait?

## TODO

- Allow speicifying order of generic parameters in `#[derive(Family)]`, this is useful for cases like `Result`, where having `T` (the success type) after `E` in its family types is preferable for implementing Monads.