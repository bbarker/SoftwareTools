use std::io::Read;
use std::io::{self, BufRead, BufReader};

pub struct BytesIter<R: Read> {
    buf_reader: BufReader<R>,
    buf: Vec<u8>,
    /// Since Iterator returns an Option instead of an Error,
    /// we log the error here, should it occur.
    error: Option<io::Error>,
}

/// Inspired by ByteSliceIter, but using new and improved std Iterator trait
/// One downside is that we must clone the buffer due to Iterator's next signature,
/// which only permits returning `Option<Self::Item>` and *not* `&Option<Self::Item>`.
/// Thus, for applications where the returned buffer slice is only read and not
/// consumed, it may be more efficient to use ByteSliceIter or related approaches.
impl<R: Read> BytesIter<R> {
    /// The default size in std [is 8 * 1024](https://github.com/rust-lang/rust/blob/6ccfe68076abc78392ab9e1d81b5c1a2123af657/src/libstd/sys_common/io.rs#L10).
    pub fn new(reader: R, size: usize) -> BytesIter<R> {
        BytesIter {
            buf_reader: BufReader::with_capacity(size, reader),
            buf: Vec::with_capacity(size),
            error: None,
        }
    }
}

impl<R: Read> Iterator for BytesIter<R> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let buf_len = self.buf.len();
        if buf_len > 0 {
            self.buf_reader.consume(buf_len);
            self.buf.clear();
        }
        let buf_len = self.buf_reader.buffer().len();
        match self.buf_reader.fill_buf() {
            Ok(buf) => {
                self.buf.extend_from_slice(&buf);
                Some(self.buf.clone())
            }
            Err(err) => {
                self.error = Some(err);
                None
            }
        }
    }
}
