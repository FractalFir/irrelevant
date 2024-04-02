//! This crate contains macros for ignoring variables in a more explicit fashion.
//! It allows you to specify *why* a variable is ignored, and enforce certain assumptions about its value or type.
//! It also prevents you from accidentally using an ingored variable by automatically shadowing it.

/// This marker signifies that a value has been explicitly ignored.
pub struct ExplicitlyIgnoredValue;
/// This macro allows you to explicitly ignore a value, provide a reason for ignoring it, and automatically check your assumptions.
///
/// WARNING: this macro runs checks in both debug and release mode. For debug-only checks, use [`debug_irrelevant`].
/// # Ignoring irrelevant types
/// You may, for example, assume that a value of a type is never relevant.
/// In this example, a function implements an operator in an interpreted language.
/// Since this is an unprivileged operation, we ignore the `PremisionSet`.
/// ```rust
/// use irrelevant::*;
/// # enum PremisionSet{};
/// /// Implements the + operator in an interpreted language
/// fn add_numbers(a:u32,b:u32,context:&PremisionSet)->u32{
///     irrelevant!(context, "Adding numbers does not require any privileges.",&PremisionSet);
///     return a + b;
/// }
/// ```
/// After changing your code, the value may become relevant again. For example, we may want to add profiling to our language.
/// We probably don't want to ignore `context` anymore. We should update this function, but this is something that you may accidentally forget.
/// In such case, the macro will automatically detect the change, and raise a compiler error:
/// ```compile_fail
/// use irrelevant::*;
/// # enum PremisionSet{};
/// # enum AutomaticProfiler{};
/// /// Implements + operator in an interpreted language
/// fn add_numbers(a:u32,b:u32,context:&(PremisionSet,AutomaticProfiler))->u32{
///     // Will not compile, because the type has changed, and the value may have become relevant.
///     irrelevant!(context, "Adding numbers does not require any privileges.",&PremisionSet);
///     return a + b;
/// }
/// ```
/// This check ensures you never accidentally ignore a value that should not be ignored.
/// # Ignoring an argument.
/// Imagine that you are writing a service dealing with restaurant orders.
/// An order may come with a sauce.
/// When adding drinks, you realize they should never come with sauces. You ban that in the frontend, but that assumption is not baked into your backend.
/// You don't want to accidentally overlook that when changing your code.
/// This macro allows you to check that assumption automatically.
/// ```
/// # use std::num::NonZeroU16;
/// # #[derive(PartialEq)]
/// # enum Allergen{Lactose}
/// # struct Sauce;
/// # trait Dish{
/// #   fn new(exclude_allergens:&[Allergen],ammount:NonZeroU16,sauces:&[Sauce])->Box<dyn Dish> where Self: Sized;
/// # }
/// use irrelevant::*;
/// # struct CocoaMilkSubstitite;
/// # impl Dish for CocoaMilkSubstitite{fn new(exclude_allergens:&[Allergen],amount:NonZeroU16,sauces:&[Sauce])->Box<dyn Dish> where Self: Sized{todo!()}}
/// # struct CocoaMilk{ammount:NonZeroU16};
/// impl Dish for CocoaMilk{
///     fn new(exclude_allergens:&[Allergen],amount:NonZeroU16,sauces:&[Sauce])->Box<dyn Dish> where Self: Sized{
///         if exclude_allergens.contains(&Allergen::Lactose){
///             return CocoaMilkSubstitite::new(exclude_allergens,amount,sauces);
///         }
///         irrelevant!(sauces,"No sauces should come with a drink!",sauces.is_empty());
///         return Box::new(Self{ammount});
///     }
/// }
/// ```
/// When this assumption is violated, an error message will be printed to `stderr`.
/// For this example, the message will look like this:
/// ```text
/// [src/main.rs:65:10] Assumption violated: No sauces should come with a drink!
/// ```
/// This macro also supports a shorthand version of checks:
/// ```
/// # use std::num::NonZeroU16;
/// # #[derive(PartialEq)]
/// # enum Allergen{Lactose}
/// # struct Sauce;
/// # trait Dish{
/// #   fn new(exclude_allergens:&[Allergen],ammount:NonZeroU16,sauces:&[Sauce])->Box<dyn Dish> where Self: Sized;
/// # }
/// # use irrelevant::*;
/// # struct CocoaMilkSubstitite;
/// # impl Dish for CocoaMilkSubstitite{fn new(exclude_allergens:&[Allergen],amount:NonZeroU16,sauces:&[Sauce])->Box<dyn Dish> where Self: Sized{todo!()}}
/// # struct CocoaMilk{ammount:NonZeroU16};
/// # impl Dish for CocoaMilk{
/// #   fn new(exclude_allergens:&[Allergen],amount:NonZeroU16,sauces:&[Sauce])->Box<dyn Dish> where Self: Sized{
/// #       if exclude_allergens.contains(&Allergen::Lactose){
/// #            return CocoaMilkSubstitite::new(exclude_allergens,amount,sauces);
/// #       }
/// irrelevant!(sauces,"No sauces should come with a drink!",is_empty);
/// #       return Box::new(Self{ammount});
/// #   }
/// # }
/// ```
/// If you want to panic on a violated assumption, use [`panic_irrelevant`].
/// # Ignoring without checks
/// You can also ignore a value without any checks.
/// ```
/// # use irrelevant::*;
/// # let val = ();
/// irrelevant!(val,"I don't like this value");
/// ```
/// Ignoring without any messages, while not recommended, is nonetheless supported.
/// ```
/// # use irrelevant::*;
///  # let val = ();
/// //probably should have a reason for ignoring this value...
/// irrelevant!(val);
/// ```
/// # Additional features
/// This macro also always automatically shadows the value, preventing you from using it accidentally.
/// ```compile_fail
/// # use irrelevant::*;
/// # let sauces = &[];
/// irrelevant!(sauces, "No sauces should come with a drink!", sauces.is_empty());
/// // `sauces` has been ignored, so this variable can't be used here!
/// for sauce in sauces{
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! irrelevant {
    // A value is ignored without any given reason.
    ($val:ident) => {
        let _ = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
    // A value is ignored without any additional assumption.
    ($val:ident,$reason:literal) => {
        //$reason
        let _ = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
    // A value is ignored because of an assumption.
    ($val:ident,$reason:literal,$cond:ident) => {
        if !($val.$cond()) {
            let file = file!();
            let line = line!();
            let column = column!();
            eprintln!("[{file}:{line}:{column}] Assumption violated: {}", $reason)
        }

        let _ = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
    // A value is ignored because its type is not relevant.
    ($val:ident,$reason:literal,$tpe:ty) => {
        let _: $tpe = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
    // A value is ignored because of an assumption.
    ($val:ident,$reason:literal,$cond:expr) => {
        if !($cond) {
            let file = file!();
            let line = line!();
            let column = column!();
            eprintln!("[{file}:{line}:{column}] Assumption violated: {}", $reason)
        }

        let _ = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
}
/// A version of [`irrelevant`] that panics when an assumption is violated. Besides that, it behaves exactly like [`irrelevant`].  
#[macro_export]
macro_rules! panic_irrelevant {
    // A value is ignored without any given reason.
    ($val:ident) => {
        let _ = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
    // A value is ignored without any additional assumption.
    ($val:ident,$reason:literal) => {
        //$reason
        let _ = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
    // A value is ignored because of an assumption.
    ($val:ident,$reason:literal,$cond:ident) => {
        if !($val.$cond()) {
            let file = file!();
            let line = line!();
            let column = column!();
            panic!("[{file}:{line}:{column}] Assumption violated: {}", $reason)
        }

        let _ = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
    // A value is ignored because its type is not relevant.
    ($val:ident,$reason:literal,$tpe:ty) => {
        let _: $tpe = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
    // A value is ignored because of an assumption.
    ($val:ident,$reason:literal,$cond:expr) => {
        if !($cond) {
            let file = file!();
            let line = line!();
            let column = column!();
            panic!("[{file}:{line}:{column}] Assumption violated: {}", $reason)
        }

        let _ = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
}
/// A version of [`irrelevant`] that only runs checks in debug mode. Besides that, it behaves exactly like [`irrelevant`].  
#[macro_export]
macro_rules! debug_irrelevant {
    // A value is ignored without any given reason.
    ($val:ident) => {
        let _ = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
    // A value is ignored without any additional assumption.
    ($val:ident,$reason:literal) => {
        //$reason
        let _ = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
    // A value is ignored because of an assumption.
    ($val:ident,$reason:literal,$cond:ident) => {
        #[cfg(debug_assertions)]
        {
            if $val.$cond() {
                let file = file!();
                let line = line!();
                let column = column!();
                eprintln!("[{file}:{line}:{column}] Assumption violated:{}", $reason)
            }
        }
        let _ = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
    // A value is ignored because its type is not relevant.
    ($val:ident,$reason:literal,$tpe:ty) => {
        let _: $tpe = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
    // A value is ignored because of an assumption.
    ($val:ident,$reason:literal,$cond:expr) => {
        #[cfg(debug_assertions)]
        {
            if $cond {
                let file = file!();
                let line = line!();
                let column = column!();
                eprintln!("[{file}:{line}:{column}] Assumption violated:{}", $reason)
            }
        }
        let _ = $val;
        let $val = irrelevant::ExplicitlyIgnoredValue;
        let _ = $val;
    };
}
