pub const fn is_newline(bt: u8) -> bool {
    bt == b'\n'
}

//TODO: const
pub fn opt_as_empty_str<T: ToString>(str_opt: Option<T>) -> String {
    str_opt
        .map(|x| ToString::to_string(&x))
        .unwrap_or_else(|| String::from(""))
}
