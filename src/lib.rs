//! Provides macros for constructing [Option] and [Result] values based on booleans.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-18

#![deny(missing_docs,)]

/// A macro for optionally producing [Option] or [Result] values.
/// 
/// # Examples
/// 
/// ```
/// # #[macro_use] extern crate maybe; fn main() {
/// assert_eq!(Some(0), maybe!(true => 0));
/// assert_eq!(Err(1), maybe!(false => 0 => 1));
/// # }
/// ```
#[macro_export]
macro_rules! maybe {
  ($check:expr => $val:expr) => (if $check { Some($val) } else { None });
  ($check:expr => $ok:expr => $err:expr) => (if $check { Ok($ok) } else { Err($err) });
}