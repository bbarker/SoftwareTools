pub const fn is_newline(bt: u8) -> bool {
    bt == b'\n'
}

//TODO: const
pub fn opt_as_empty_str<T: ToString>(str_opt: Option<T>) -> String {
    str_opt
        .map(|x| ToString::to_string(&x))
        .unwrap_or_else(|| String::from(""))
}

// /// A dummy parallel macro to be used in non-threaded builds.
// #[macro_export]
// macro_rules! parallel {
//     // no-op, just pass all tokens without processing
//     ($($tokens:tt)*) => { $($tokens)* }
// }

/// We use a macro here to avoid complex and potentially changing types,
/// when really all we need is to switch the string being used for `iter`
/// at build-time.
#[macro_export]
macro_rules! par_iter {
    ($($tokens:tt)*) => {{
      #[cfg(feature = "threaded")]
      let itr = $($tokens)*.par_iter();
      #[cfg(not(feature = "threaded"))]
      let itr = $($tokens)*.iter();
      itr
    }}
}

