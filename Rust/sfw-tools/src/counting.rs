use std::fs::File;
use std::io::Error;

use fp_core::{empty::*, monoid::*, semigroup::*};
use seahorse::{App, Command, Context, Flag, FlagType};

use crate::bytes_iter::BytesIter;
use crate::constants::*;
use crate::error::*;
use crate::util::{is_newline, opt_as_empty_str};

pub fn wc_app() -> App {
    App::new("wc")
        .author("Brandon Elam Barker")
        .action(run_wc_seahorse_action)
        .command(run_wc_seahorse_cmd())
}

const WC_USAGE: &str = r#"
wc [OPTION] FILE

No option implies lines, words and bytes will be printed.

Valid options are:
-c            print the byte counts
-w            print the word counts
-l            print the line counts

"#;

pub fn run_wc_seahorse_cmd() -> Command {
    Command::new("wc")
        .description("wc: line, word, and byte counting")
        .usage(WC_USAGE)
        .action(run_wc_seahorse_action)
        .flag(
            Flag::new("bytes", FlagType::Bool)
                .alias("c")
                .description("wc -c some_file"),
        )
        .flag(
            Flag::new("words", FlagType::Bool)
                .alias("w")
                .description("wc -w some_file"),
        )
        .flag(
            Flag::new("lines", FlagType::Bool)
                .alias("l")
                .description("wc -l some_file"),
        )
}

pub fn run_wc_seahorse_action(ctxt: &Context) {
    let src = ctxt.args.first().user_err("wc: missing source");
    let do_bytes = ctxt.bool_flag("bytes");
    let do_words = ctxt.bool_flag("words");
    let do_lines = ctxt.bool_flag("lines");
    let do_all = do_lines && do_words && do_bytes;
    let do_all = do_all || (!do_lines && !do_words && !do_bytes);
    let counts: Counts;
    if do_all {
        counts = wc_all(src).user_err("Error in wc_all");
    } else {
        let mut build_counts = Counts::null();
        if do_bytes {
            build_counts = wc_bytes(src)
                .map(|b| build_counts.bytes(b))
                .user_err("Error in wc_bytes");
        }
        if do_words {
            build_counts = wc_words(src)
                .map(|b| build_counts.words(b))
                .user_err("Error in wc_words");
        }
        if do_lines {
            build_counts = wc_lines(src)
                .map(|b| build_counts.lines(b))
                .user_err("Error in wc_lines");
        }
        counts = build_counts;
    }
    println!("{}", Counts::format(&counts));
}

/// Convenience function for running wc in idiomatic fashion
/// (i.e.) errors are printed to user and the program exits.
pub fn run_wc_lines(src: &str) {
    let wc_res = wc_lines(src).user_err("Error in wc_lines");
    println!("{}", Counts::format(&Counts::null().lines(wc_res)));
}

pub fn wc_lines(src: &str) -> Result<usize, Error> {
    let f_in =
        File::open(src).sfw_err(&format!("Couldn't open source: {}", &src))?;
    wc_lines_file(&f_in)
}

/// In Chapter 1, page 15 of Software Tools, the authors discuss the
/// hazards of boundary conditions in programming. Certainly this is still
/// a problem in Rust, but using Rust's functional programming facilities,
/// and types can help to greatly reduce the occurrence of such errors.
pub fn wc_lines_file(f_in: &File) -> Result<usize, Error> {
    BytesIter::new(f_in, DEFAULT_BUF_SIZE)
        .try_fold(0_usize, |ac_tot, b_slice| {
            Ok(ac_tot + num_newlines(&b_slice?))
        })
}

/// Convenience function for running wc in idiomatic fashion
/// (i.e.) errors are printed to user and the program exits.
pub fn run_wc_bytes(src: &str) {
    let wc_res = wc_bytes(src).user_err("Error in wc_bytes");
    println!("{} {}", wc_res, &src);
}

pub fn wc_bytes(src: &str) -> Result<usize, Error> {
    let f_in =
        File::open(src).sfw_err(&format!("Couldn't open source: {}", &src))?;
    wc_bytes_file(&f_in)
}

pub fn wc_bytes_file(f_in: &File) -> Result<usize, Error> {
    BytesIter::new(f_in, DEFAULT_BUF_SIZE)
        .try_fold(0_usize, |ac_tot, b_slice| Ok(ac_tot + b_slice?.len()))
}

/// Convenience function for running wc in idiomatic fashion
/// (i.e.) errors are printed to user and the program exits.
pub fn run_wc_words(src: &str) {
    let wc_res = wc_words(src).user_err("Error in wc_words");
    println!("{}", Counts::format(&Counts::null().words(wc_res)));
}

pub fn wc_words(src: &str) -> Result<usize, Error> {
    let f_in =
        File::open(src).sfw_err(&format!("Couldn't open source: {}", &src))?;
    wc_words_file(&f_in)
}

pub fn wc_words_file(f_in: &File) -> Result<usize, Error> {
    BytesIter::new(f_in, DEFAULT_BUF_SIZE)
        .try_fold(0_usize, |ac_tot, b_slice| {
            Ok(ac_tot + word_count(b_slice?.as_slice()))
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
}

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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Counts {
    pub bytes: Option<usize>,
    pub words: Option<usize>,
    pub lines: Option<usize>,
}

impl Counts {
    const fn new(bytes: usize, words: usize, lines: usize) -> Self {
        Counts {
            bytes: Some(bytes),
            words: Some(words),
            lines: Some(lines),
        }
    }
    const fn empty() -> Self {
        Self::new(0, 0, 0)
    }
    const fn null() -> Self {
        Counts {
            bytes: None,
            words: None,
            lines: None,
        }
    }
    const fn bytes(self, bytes: usize) -> Self {
        Counts {
            bytes: Some(bytes),
            ..self
        }
    }
    const fn words(self, words: usize) -> Self {
        Counts {
            words: Some(words),
            ..self
        }
    }
    const fn lines(self, lines: usize) -> Self {
        Counts {
            lines: Some(lines),
            ..self
        }
    }
    //TODO: const
    fn format(&self) -> String {
        let b_str = opt_as_empty_str(self.bytes);
        let w_str = opt_as_empty_str(self.words);
        let l_str = opt_as_empty_str(self.lines);
        format!("{} {} {}", l_str, w_str, b_str)
    }
}

/// Representation of a chunk of text.
///
/// All of the Flux-based code below is inspired
/// [by Martin Mroz](https://github.com/martinmroz/wc_rs)
/// The result of the `wc` operation.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Flux {
    /// The type of the left-most character in the chunk.
    pub left_char_type: CharType,
    /// The number of bytes in the chunk.
    pub bytes: usize,
    /// The number of words in the chunk.
    pub words: usize,
    /// The number of lines in the chunk.
    pub lines: usize,
    /// The type of the right-most character in the chunk.
    pub right_char_type: CharType,
}

impl Flux {
    /// Returns a new instance of the receiver with the provided parameters.
    fn new(
        left_char_type: CharType,
        bytes: usize,
        words: usize,
        lines: usize,
        right_char_type: CharType,
    ) -> Self {
        Flux {
            left_char_type,
            bytes,
            words,
            lines,
            right_char_type,
        }
    }

    /// Returns a new Flux spanning the receiver on the left, and `rhs` on the right.
    fn span(self, rhs: Flux) -> Self {
        let words = {
            // If the span is formed along a non-space to non-space
            // boundary the word count is one less than the sum.
            if let (CharType::NotSpace, CharType::NotSpace) =
                (self.right_char_type, rhs.left_char_type)
            {
                self.words + rhs.words - 1
            } else {
                self.words + rhs.words
            }
        };

        Flux::new(
            self.left_char_type,
            self.bytes + rhs.bytes,
            words,
            self.lines + rhs.lines,
            rhs.right_char_type,
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum FluxMay {
    FluxSome(Flux),
    FluxEmpty,
}
use FluxMay::*;

impl FluxMay {
    /// Returns a new instance of the receiver with the provided parameters.
    fn new(
        left_char_type: CharType,
        bytes: usize,
        words: usize,
        lines: usize,
        right_char_type: CharType,
    ) -> Self {
        FluxMay::FluxSome(Flux::new(
            left_char_type,
            bytes,
            words,
            lines,
            right_char_type,
        ))
    }

    fn counts(&self) -> Counts {
        match self {
            FluxSome(flux) => Counts::new(flux.bytes, flux.words, flux.lines),
            FluxEmpty => Counts::empty(),
        }
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

/// Convenience function for running wc in idiomatic fashion
/// (i.e.) errors are printed to user and the program exits.
pub fn run_wc_all(src: &str) {
    let wc_res = wc_all(src).user_err("Error in wc_all");
    println!("{}", Counts::format(&wc_res));
}

pub fn wc_all(src: &str) -> Result<Counts, Error> {
    let f_in =
        File::open(src).sfw_err(&format!("Couldn't open source: {}", &src))?;
    wc_all_file(&f_in)
}

pub fn wc_all_file(f_in: &File) -> Result<Counts, Error> {
    BytesIter::new(f_in, DEFAULT_BUF_SIZE)
        .try_fold(FluxEmpty, |flux_may, b_slice| {
            Ok(Semigroup::combine(
                flux_may,
                FluxMay::from(b_slice?.as_slice()),
            ))
        })
        .map(|f| FluxMay::counts(&f))
}

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

    #[test]
    fn test_flux_may_from() {
        assert_eq!(
            FluxMay::from("testing one two three ".as_bytes()),
            FluxSome(Flux::new(
                CharType::NotSpace,
                22,
                4,
                0,
                CharType::IsSpace
            ))
        );
    }

    #[test]
    fn test_flux_may_combine() {
        let flux_l = FluxMay::from("testing on".as_bytes());
        let flux_r = FluxMay::from("e two three".as_bytes());

        assert_eq!(
            Semigroup::combine(flux_l, flux_r),
            FluxSome(Flux::new(
                CharType::NotSpace,
                21,
                4,
                0,
                CharType::NotSpace
            ))
        );
    }

    #[test]
    fn test_flux_may_combine_space() {
        let flux_l = FluxMay::from("testing one ".as_bytes());
        let flux_r = FluxMay::from(" two three".as_bytes());

        assert_eq!(
            Semigroup::combine(flux_l, flux_r),
            FluxSome(Flux::new(
                CharType::NotSpace,
                22,
                4,
                0,
                CharType::NotSpace
            ))
        );
    }
}
