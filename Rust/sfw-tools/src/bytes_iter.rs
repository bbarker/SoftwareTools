use std::io::{BufRead, BufReader, Error, Read};

pub struct BytesIter<R: Read> {
    buf_reader: BufReader<R>,
    buf: Vec<u8>,
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
        }
    }
}

impl<R: Read> Iterator for BytesIter<R> {
    type Item = Result<Vec<u8>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let buf_len = self.buf.len();
        if buf_len > 0 {
            self.buf_reader.consume(buf_len);
            self.buf.clear();
        }
        match self.buf_reader.fill_buf() {
            Ok(buf) => {
                if !buf.is_empty() {
                    self.buf.extend_from_slice(&buf);
                    Some(Ok(self.buf.clone()))
                } else {
                    None
                }
            }
            Err(err) => Some(Err(err)),
        }
    }
}

// Copyright (c) 2017 Ted Mielczarek
#[cfg(test)]
mod tests {
    use super::*;

    use std::env;
    use std::env::consts::EXE_EXTENSION;
    use std::path::Path;
    use std::process::Command;

    const DEFAULT_BUF_SIZE: usize = 8 * 1024;

    #[test]
    fn readme_test() {
        let rustdoc = Path::new("rustdoc").with_extension(EXE_EXTENSION);
        let readme = Path::new(file!())
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("README.md");
        let exe = env::current_exe().unwrap();
        let outdir = exe.parent().unwrap();
        let mut cmd = Command::new(rustdoc);
        cmd.args(&["--verbose", "--test", "-L"])
            .arg(&outdir)
            .arg(&readme);
        println!("{:?}", cmd);
        let result = cmd
            .spawn()
            .expect("Failed to spawn process")
            .wait()
            .expect("Failed to run process");
        assert!(
            result.success(),
            "Failed to run rustdoc tests on README.md!"
        );
    }

    fn sliced(b: &[u8], size: usize) -> Vec<Vec<u8>> {
        let mut v = vec![];
        let mut iter = BytesIter::new(b, size);
        while let Some(chunk) = iter.next() {
            v.push(chunk.to_owned());
        }
        v
    }

    fn test<T: AsRef<[u8]>>(bytes: T, size: usize) {
        let bytes = bytes.as_ref();
        let a = sliced(bytes, size);
        let b = bytes.chunks(size).collect::<Vec<_>>();
        if a != b {
            panic!(
                "chunks are not equal!
read-byte-slice produced {} chunks with lengths: {:?}
slice.chunks produced {} chunks with lengths: {:?}",
                a.len(),
                a.iter().map(|c| c.len()).collect::<Vec<_>>(),
                b.len(),
                b.iter().map(|c| c.len()).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn test_simple() {
        let bytes = b"0123456789abcdef";
        test(bytes, 4);
    }

    #[test]
    fn test_non_even() {
        let bytes = b"0123456789abcd";
        test(bytes, 4);
    }

    #[test]
    fn test_chunks_larger_than_bufread_default_buffer() {
        let bytes = (0..DEFAULT_BUF_SIZE * 4)
            .map(|i| (i % 256) as u8)
            .collect::<Vec<u8>>();
        let size = DEFAULT_BUF_SIZE * 2;
        test(bytes, size);
    }
}
