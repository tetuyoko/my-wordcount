//! wordcount is simply charactor, word, line appearence freaquency counter;
//! If you access more details about this function, see this document [`count`](fn.count.html)
#![warn(missing_docs)]

use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// Option that [`count`](fn.count.html) uses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// by charactor
    Char,
    /// by word
    Word,
    /// by line
    Line,
}

/// Default is [`Word`](enum.CountOption.html#variant.Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// Count up from text file
/// * [`CountOption::Line`](enum.CountOption.html#variant.Line): Countlines that separated `\n` or `\r\n`
///
/// # Examples
///  Count inputing word frequency.
///
/// ``` rust
/// use std::io::Cursor;
/// use my_wordcount::{count, CountOption};
///
/// let mut input = Cursor::new("aa bb cc bb");
/// let freq = count(input, CountOption::Word);
///
/// assert_eq!(freq["aa"], 1);
/// assert_eq!(freq["bb"], 2);
/// assert_eq!(freq["cc"], 1);
/// ```
///
/// # Panics
///
///
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        use crate::CountOption::{Char, Line, Word};
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }
    freqs
}

#[cfg(test)]
mod test {
    use super::{count, CountOption, HashMap};
    use std::io::Cursor;
    // assert for each pairs of map.
    macro_rules! assert_map {
        ($expr: expr, { $($key: expr => $value: expr), * }) => {
            $(assert_eq!($expr[$key], $value));*
        };
    }

    #[test]
    fn wordcount_works() {
        let mut exp = HashMap::new();
        exp.insert("aa".to_string(), 1);
        exp.insert("bb".to_string(), 1);
        exp.insert("cc".to_string(), 1);
        exp.insert("dd".to_string(), 1);

        assert_eq!(count(Cursor::new("aa bb cc dd"), CountOption::Word), exp);
    }

    #[test]
    fn wordcount_works3() {
        use std::io::Cursor;

        let freqs = count(Cursor::new("aa cc dd"), CountOption::Word);
        assert_eq!(freqs.len(), 3);
        assert_map!(freqs, {"aa" => 1, "cc" => 1, "dd" => 1} );
    }
}
