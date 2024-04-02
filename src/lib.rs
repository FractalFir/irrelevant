//! This crate contians macros for ignoring variables in a more explict fasion.
//! It allows you to specify *why* a variable is ignored, and enforce certain assumptions about its value or type.
//! It also prevents you from accientaly using and unused variable by automaticaly shadowing it.

/// This marker sygnifies that a value has been explicitly ignored.
pub struct ExplicitlyIgnoredValue;
/// This macro allows you to explictly ignore a value, provide a reason for ignoring it, and automaticaly checks your assumptions.
///
/// WARNING: this macro runs checks in both debug and release mode. For debug-only checks, use [`debug_irrelevant`].
/// # Ignoring irrelevant types
/// You may, for example, assume that a value of a type is never relevant.
/// In this example, a function implements an operator in an iterpreted language.
/// Since this is an unpriveleged opertaion, we ignore the `PremisionSet`.
/// ```rust
/// use irrelevant::*;
/// # enum PremisionSet{};
/// /// Implements the + operator in an interpreted language
/// fn add_numbers(a:u32,b:u32,context:&PremisionSet)->u32{
///     irrelevant!(context,"Adding numbers does not require any priveleges.",&PremisionSet);
///     return a + b;
/// }
/// ```
/// After chaning your code, the value may become relevant again. For example, we may want to add profiling to our langauge.
/// We proapbly don't want to ingore `context` anymore. We should update this function, but this is something that you may accidently forget.
/// In such case, the macro will automaticaly detect the change, and raise a compiler error:
/// ```compile_fail
/// use irrelevant::*;
/// # enum PremisionSet{};
/// # enum AutomaticProfiler{};
/// /// Implements + operator in an interpreted language
/// fn add_numbers(a:u32,b:u32,context:&(PremisionSet,AutomaticProfiler))->u32{
///     // Will not compile, because the type has changed, and the value may have become relevant.
///     irrelevant!(context,"Adding numbers does not require any priveleges.",&PremisionSet);
///     return a + b;
/// }
/// ```
/// This check ensures you never accidentaly ignore a value that should not be ignored.
/// # Ignoring an argument.
/// Imagine that you are writing a service dealing with restaurant orders.
/// An order may come with a sauce.
/// When adding drinks, you realize they should never come with sauces. You ban that in the frontend, but that assumption is not baked into your backend.
/// You don't want to accidentaly overlook that when changing your code.
/// This macro allows you to check that assumption automaticaly.
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
/// # impl Dish for CocoaMilkSubstitite{fn new(exclude_allergens:&[Allergen],ammount:NonZeroU16,sauces:&[Sauce])->Box<dyn Dish> where Self: Sized{todo!()}}
/// # struct CocoaMilk{ammount:NonZeroU16};
/// impl Dish for CocoaMilk{
///     fn new(exclude_allergens:&[Allergen],ammount:NonZeroU16,sauces:&[Sauce])->Box<dyn Dish> where Self: Sized{
///         if exclude_allergens.contains(&Allergen::Lactose){
///             return CocoaMilkSubstitite::new(exclude_allergens,ammount,sauces);
///         }
///         irrelevant!(sauces,"No sauces should come with a drink!",sauces.is_empty());
///         return Box::new(Self{ammount});
///     }
/// }
/// ```
/// When this assummption is violated, an error message will be printed to `stderr`.
/// For this example, the message will look like this:
/// ```text
/// [src/main.rs:65:10] Assumption violated:No sauces should come with a drink!
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
/// # impl Dish for CocoaMilkSubstitite{fn new(exclude_allergens:&[Allergen],ammount:NonZeroU16,sauces:&[Sauce])->Box<dyn Dish> where Self: Sized{todo!()}}
/// # struct CocoaMilk{ammount:NonZeroU16};
/// # impl Dish for CocoaMilk{
/// #   fn new(exclude_allergens:&[Allergen],ammount:NonZeroU16,sauces:&[Sauce])->Box<dyn Dish> where Self: Sized{
/// #       if exclude_allergens.contains(&Allergen::Lactose){
/// #            return CocoaMilkSubstitite::new(exclude_allergens,ammount,sauces);
/// #       }
/// irrelevant!(sauces,"No sauces should come with a drink!",is_empty);
/// #       return Box::new(Self{ammount});
/// #   }
/// # }
/// ```
/// If you want to panic on a violated assumptions, use [`panic_irrelevant`].
/// # Ignoring without checks
/// You can also ignore a value without any checks.
/// ```
/// # use irrelevant::*;
/// # let val = ();
/// irrelevant!(val,"I don't like this value");
/// ```
/// Ignoring without any messages, while not recomended, is nontheless supported.
/// ```
/// # use irrelevant::*;
///  # let val = ();
/// // Propably should have an reason for ignoring this value...
/// irrelevant!(val);
/// ```
/// # Additional featrues
/// This macro also always automaticaly shadows the value, preventing you from using it accidentaly.
/// ```compile_fail
/// # use irrelevant::*;
/// # let sauces = &[];
/// irrelevant!(sauces,"No sauces should come with a drink!",sauces.is_empty());
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
            eprintln!("[{file}:{line}:{column}] Assumption violated:{}", $reason)
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
            eprintln!("[{file}:{line}:{column}] Assumption violated:{}", $reason)
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
            panic!("[{file}:{line}:{column}] Assumption violated:{}", $reason)
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
            panic!("[{file}:{line}:{column}] Assumption violated:{}", $reason)
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
