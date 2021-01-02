use std::fs::File;
use std::io::Error;

use fp_core::{empty::*, monoid::*, semigroup::*};

use crate::bytes_iter::BytesIter;
use crate::constants::*;
use crate::error::*;
use crate::util::is_newline;

/// Convenience function for running wc in idiomatic fashion
/// (i.e.) errors are printed to user and the program exits.
pub fn run_wc(src: &str) {
    let wc_res = wc(src).user_err("Error in wc");
    println!("{}", wc_res);
}

pub fn wc(src: &str) -> Result<usize, Error> {
    let f_in = File::open(&src)
        .sfw_err(&*format!("Couldn't open source: {}", &src))?;
    wc_file(&f_in)
}

/// In Chapter 1, page 15 of Software Tools, the authors discuss the
/// hazards of boundary conditions in programming. Certainly this is still
/// a problem in Rust, but using Rust's functional programming facilities,
/// and types can help to greatly reduce the occurrence of such errors.
pub fn wc_file(f_in: &File) -> Result<usize, Error> {
    BytesIter::new(f_in, DEFAULT_BUF_SIZE)
        .try_fold(0_usize, |ac_tot, b_slice| {
            Ok(ac_tot + num_newlines(&b_slice?))
        })
}

/// The class of a character.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum CharType {
    /// The character represents a whitespace separator.
    IsSpace,
    /// The character does not represent a whitespace separator.
    NotSpace,
}

impl From<&u8> for CharType {
    fn from(other: &u8) -> Self {
        if other.is_ascii_whitespace() {
            // A line-feed is considered an ASCII whitespace
            // character by `is_ascii_whitespace`.
            CharType::IsSpace
        } else {
            CharType::NotSpace
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct WordCount {
    current: CharType,
    count: usize,
} // TODO: fold over these with monoid implementation

impl From<&u8> for WordCount {
    fn from(other: &u8) -> Self {
        WordCount {
            current: CharType::from(other),
            count: 0,
        }
    }
}

const WORD_COUNT_0: WordCount = WordCount {
    current: CharType::IsSpace,
    count: 0,
};

impl Empty for WordCount {
    fn empty() -> Self {
        WORD_COUNT_0
    }
}
//
impl Semigroup for WordCount {
    fn combine(self, other: Self) -> Self {
        let new_count = match other.current {
            CharType::IsSpace => self.count,
            CharType::NotSpace => match self.current {
                CharType::IsSpace => self.count + 1,
                CharType::NotSpace => self.count,
            },
        };
        WordCount {
            current: other.current,
            count: new_count,
        }
    }
}
//
impl Monoid for WordCount {}

pub fn word_count(b_slice: &[u8]) -> usize {
    b_slice
        .iter()
        .map(WordCount::from)
        .fold(Empty::empty(), Semigroup::combine)
        .count
}

pub fn num_newlines(b_slice: &[u8]) -> usize {
    b_slice.iter().fold(
        0_usize,
        |ac, bt| {
            if is_newline(*bt) {
                ac + 1
            } else {
                ac
            }
        },
    )
}

//  All of the Flux-based code below is Copyright (c) 2019 Martin Mroz

/// This is based on a parallel implementation
/// [by Martin Mroz](https://github.com/martinmroz/wc_rs)

/// The result of the `wc` operation.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Counts {
    pub bytes: usize,
    pub words: usize,
    pub lines: usize,
}

/// Representation of a chunk of text.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Flux {
    /// The type of the left-most character in the chunk.
    pub leftmost_char_type: CharType,
    /// The number of bytes in the chunk.
    pub bytes: usize,
    /// The number of words in the chunk.
    pub words: usize,
    /// The number of lines in the chunk.
    pub lines: usize,
    /// The type of the right-most character in the chunk.
    pub rightmost_char_type: CharType,
}

impl Flux {
    /// Returns a new instance of the receiver with the provided parameters.
    fn new(
        leftmost_char_type: CharType,
        bytes: usize,
        words: usize,
        lines: usize,
        rightmost_char_type: CharType,
    ) -> Self {
        Flux {
            leftmost_char_type,
            bytes,
            words,
            lines,
            rightmost_char_type,
        }
    }

    /// Returns a new Flux spanning the receiver on the left, and `rhs` on the right.
    fn span(self, rhs: Flux) -> Self {
        let words = {
            // If the span is formed along a non-space to non-space
            // boundary the word count is one less than the sum.
            if let (CharType::NotSpace, CharType::NotSpace) =
                (self.rightmost_char_type, rhs.leftmost_char_type)
            {
                self.words + rhs.words - 1
            } else {
                self.words + rhs.words
            }
        };

        Flux::new(
            self.leftmost_char_type,
            self.bytes + rhs.bytes,
            words,
            self.lines + rhs.lines,
            rhs.rightmost_char_type,
        )
    }
}

enum FluxMay {
    FluxSome(Flux),
    FluxEmpty,
}
use FluxMay::*;

impl FluxMay {
    /// Returns a new instance of the receiver with the provided parameters.
    fn new(
        leftmost_char_type: CharType,
        bytes: usize,
        words: usize,
        lines: usize,
        rightmost_char_type: CharType,
    ) -> Self {
        FluxMay::FluxSome(Flux::new(
            leftmost_char_type,
            bytes,
            words,
            lines,
            rightmost_char_type,
        ))
    }
}

impl Empty for FluxMay {
    fn empty() -> Self {
        FluxMay::FluxEmpty
    }
}
//
impl Semigroup for FluxMay {
    fn combine(self, other: Self) -> Self {
        match other {
            FluxEmpty => self,
            FluxSome(other_flux) => match self {
                FluxEmpty => other,
                FluxSome(self_flux) => {
                    FluxSome(Flux::span(self_flux, other_flux))
                }
            },
        }
    }
}
//
impl Monoid for FluxMay {}

impl From<&[u8]> for FluxMay {
    /// Creates a new instance of a Flux encoding a buffer.
    fn from(buf: &[u8]) -> Self {
        if buf.is_empty() {
            FluxMay::FluxEmpty
        } else {
            // A line-feed is considered an ASCII whitespace
            // character by `is_ascii_whitespace`.
            let lines = num_newlines(buf);
            let first_char = CharType::from(buf.first().unwrap_or(&b' '));
            let last_char = CharType::from(buf.last().unwrap_or(&b' '));

            FluxMay::new(
                first_char,
                buf.len(),
                word_count(buf),
                lines,
                last_char,
            )
        }
    }
}


// /// Computes the flux over the provided input byte string.
// fn flux_over_byte_string<T>(input: T) -> Option<Flux>
// where
//     T: AsRef<[u8]>,
// {
//     input
//         .as_ref()
//         .iter()//         .map(Flux::from)
//         .fold(|| None, |acc, next| span_opt(acc, Some(next)))
//         .reduce(|| None, |acc, next| span_opt(acc, next))
// }

// fn wc_all<T>(input: BytesIter) -> std::io::Result<Counts>
// {

//     // BytesIter::new(f_in, DEFAULT_BUF_SIZE).try_fold(Counts, |ac_tot, b_slice| {
//     //     let buf = b_slice?;
//     //     let buf_count = Count { buf.len(),
//     //     Ok(ac_tot
//     //         + b_slice?.iter().fold(0u32, |ac, bt| {
//     //             if is_newline(*bt) {
//     //                 ac + 1
//     //             } else {
//     //                 ac
//     //             }
//     //         }))
//     // })

//     /*
//     let mut bytes = 0;
//     let mut flux = None;

//     /*
//     'buffer_loop: loop {
//         let buffer = input.fill_buf()?;
//         let length = buffer.len();
//         if length == 0 {
//             break 'buffer_loop;
//         }

//         // Update the byte counter from the buffer.
//         bytes = bytes + length;

//         // Fold the flux of the next buffer into the existing.
//         flux = span_opt(flux, flux_over_byte_string(&buffer));

//         // Mark the buffer as consumed.
//         input.consume(length);
//     }
//     */
//     Ok(Counts {
//         bytes,
//         words: flux.map(|f| f.words).unwrap_or_default(),
//         lines: flux.map(|f| f.lines).unwrap_or_default(),
//     })
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count_over_byte_string() {
        let num_words1 = word_count("testing one\ntwo three".as_bytes());
        assert_eq!(num_words1, 4);
        let num_words2 = word_count("testing one\ntwo three\n".as_bytes());
        assert_eq!(num_words2, 4);
        let num_words3 = word_count("\ntesting one\ntwo three".as_bytes());
        assert_eq!(num_words3, 4);
        let num_words4 = word_count(" testing one  two three\n  ".as_bytes());
        assert_eq!(num_words4, 4);
    }

    //     #[test]
    //     fn test_flux_over_byte_string() {
    //         assert_eq!(
    //             flux_over_byte_string("testing one two three".as_bytes()),
    //             Some(Flux::new(CharType::NotSpace, 4, 0, CharType::NotSpace))
    //         );
    //     }

    //     #[test]
    //     fn test_span_opt_not_space_to_not_space() {
    //         let flux_l = flux_over_byte_string("testing on");
    //         let flux_r = flux_over_byte_string("e two three");

    //         assert_eq!(
    //             span_opt(flux_l, flux_r),
    //             Some(Flux::new(CharType::NotSpace, 4, 0, CharType::NotSpace))
    //         );
    //     }

    //     #[test]
    //     fn test_span_opt_space_to_space() {
    //         let flux_l = flux_over_byte_string("testing one ");
    //         let flux_r = flux_over_byte_string(" two three");

    //         assert_eq!(
    //             span_opt(flux_l, flux_r),
    //             Some(Flux::new(CharType::NotSpace, 4, 0, CharType::NotSpace))
    //         );
    //     }
}
