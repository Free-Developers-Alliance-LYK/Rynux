/// Concatenates constants of primitive types into a `&'static str`.
///
/// Each argument is stringified after evaluating it, so `concatcp!(1u8 + 3) == "4"`
///
/// [For **examples** look here](#examples)
///
/// `concatcp` stands for "concatenate constants (of) primitives"
///
/// # Limitations
///
/// This macro can only take constants of these types as inputs:
///
/// - `&str`
///
/// - `i*`/`u*` (all the primitive integer types).
///
/// - `char`
///
/// - `bool`
///
/// This macro also shares
/// [the limitations described in here](./index.html#macro-limitations)
/// as well.
///
/// # Examples
///
/// ### Literal arguments
///
///
/// ```rust
/// use const_format::concatcp;
///
/// const MSG: &str = concatcp!(2u8, "+", 2u8, '=', 2u8 + 2);
///
/// assert_eq!(MSG, "2+2=4");
///
/// ```
///
/// ### `const` arguments
///
/// ```rust
/// use const_format::concatcp;
///
/// const PASSWORD: &str = "password";
///
/// const fn times() -> u64 { 10 }
///
/// const MSG: &str =
///     concatcp!("The password is \"", PASSWORD, "\", you can only guess ", times(), " times.");
///
/// assert_eq!(MSG, r#"The password is "password", you can only guess 10 times."#);
///
/// ```
///
#[macro_export]
macro_rules! concatcp {
    ()=>{""};
    ($($arg: expr),* $(,)?)=>(
        $crate::const_format::__str_const! {{
            use $crate::const_format::__cf_osRcTFl4A;
            $crate::pmr::__concatcp_impl!{
                $( ( $arg ), )*
            }
        }}
    );
}

