# What is `irrelevant`
`irrelevant` is a Rust crate aiming to make ignoring variables more explicit.
# What problem it solves
Sometimes, you ignore a variable, because it seems like you don't need it. For example, you may assume that a `Dish` of type `Drink` will never come with a sauce, so you ignore this field of an order. 
This is an *implicit* assumption about the behavior of your program, and it may not be immediately obvious *why* sauces are ignored.
```rust
// Why are sauces ignored?
let _ = sauces;
```
This crate allows you to specify why that happens, and check that assumption:
```rust
irrelevant!(sauces,"No sauces should come with a drink!",sauces.is_empty());
// Shorthand version supported too!
irrelevant!(sauces,"No sauces should come with a drink!",is_empty);
```
Those checks allow you to make the implicit assumption into an explicit one. This way, if you change your code in such a way that the assumption is violated, an error message will be logged.
This prevents you from accidentally violating your assumptions about the code.

`irrelevant` also allows you to check the type you ignore. An argument may not be relevant at some point in the past, but it may become important after a change. Imagine implementing some operation for an 
interpreted language:
```rust
/// Implements the + operator in an interpreted language
fn add_numbers(a:u32,b:u32,context:&PremisionSet)->u32{
    irrelevant!(context,"Adding numbers does not require any privileges.",&PremisionSet);
    return a + b;
}
```
The `context` may be ignored for now, since adding numbers is an unprivileged operation. But, if we add profiling to our interpreter, ignoring context is no longer OK.
```rust
use irrelevant::*;
/// Implements + operator in an interpreted language
fn add_numbers(a:u32,b:u32,context:&(PremisionSet,AutomaticProfiler))->u32{
    // Will not compile, because the type has changed, and the value may have become relevant.
    irrelevant!(context,"Adding numbers does not require any privileges.",&PremisionSet);
    return a + b;
}
```
This change is caught at compile time and reported as an error.

The macros also automatically shadow the ignored value, preventing you from using it accidentally.
```
irrelevant!(sauces,"No sauces should come with a drink!",sauces.is_empty());
// `sauces` has been ignored, so this variable can't be used here! 
for sauce in sauces{
    // ...
}
```
# Variants of the macro.
There are 3 variants of the macro:
1. `irrelevant` - always logs the error
2. `debug_irrelevant` - logs the error if built-in debug
3. `panic_irrelevant` - always panics on error
# License
This crate is dual licensed under the MIT license and the Apache License, Version 2.0.
