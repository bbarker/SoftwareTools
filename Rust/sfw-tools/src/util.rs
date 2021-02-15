use std::io::{Error, Write};

pub const fn is_newline(bt: u8) -> bool {
    bt == b'\n'
}

pub const fn is_tab(bt: u8) -> bool {
    bt == b'\t'
}

pub const fn is_tab_or_newline(bt: u8) -> bool {
    is_tab(bt) || is_newline(bt)
}

//TODO: const
pub fn opt_as_empty_str<T: ToString>(str_opt: Option<T>) -> String {
    str_opt
        .map(|x| ToString::to_string(&x))
        .unwrap_or_else(|| String::from(""))
}

// Based on write_u8 from byteorder
#[inline]
pub fn write_u8(writer: &mut Write, n: u8) -> Result<(), Error> {
    writer.write_all(&[n])
}
