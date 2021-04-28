use std::io::Error;
use std::io::ErrorKind::InvalidData;

pub trait IterExtra: Iterator {
    fn safe_take(self, nn: usize) -> Result<Vec<Self::Item>, Error>;
}

impl<I: Iterator> IterExtra for I {
    fn safe_take(self, nn: usize) -> Result<Vec<Self::Item>, Error>
    where
        I: Sized,
    {
        let result_vec = self.take(nn).collect::<Vec<Self::Item>>();
        let vec_len = result_vec.len();
        if vec_len == nn {
            Ok(result_vec)
        } else {
            Err(Error::new(InvalidData
                           , format!("safe_take could only take {} elements, needed at least {}."
                                     , vec_len, nn)))
        }
    }
}
