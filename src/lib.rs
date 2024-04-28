#![no_std]
#![warn(clippy::pedantic, clippy::nursery, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::question_mark_used,
    clippy::pattern_type_mismatch,
    clippy::use_debug,
    clippy::shadow_unrelated
)]

extern crate alloc;

use alloc::{
    string::{String, ToString as _},
    vec::Vec,
};
use core::fmt::{self, Display, Formatter};
use itertools::{Itertools, PeekingNext};
use object::Object;

pub mod error;
pub mod object;

/// Checks whether the next characters of the iterator are equal to those of the string.
fn compare_iter_str(iter: impl IntoIterator<Item = char>, string: &str) -> bool {
    iter.into_iter()
        .take(string.len())
        .zip(string.chars())
        .filter(|(left, right)| left == right)
        .count()
        == string.len()
}

/// An enum representing a json value
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Object),
}

impl FromIterator<char> for Json {
    #[allow(clippy::unwrap_used)]
    #[inline]
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self::new(&mut iter.into_iter().peekable()).unwrap()
    }
}

impl Display for Json {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(formatter, "null"),
            Self::Bool(value) => write!(formatter, "{value}"),
            Self::Number(value) => write!(formatter, "{value}"),
            Self::String(value) => write!(formatter, "\"{value}\""),
            Self::Array(values) => {
                write!(formatter, "[")?;
                if let Some(first_value) = values.first() {
                    write!(formatter, "{first_value}")?;
                    for value in values.iter().skip(1) {
                        write!(formatter, ",{value}")?;
                    }
                }
                write!(formatter, "]")
            }
            Self::Object(values) => write!(formatter, "{values}"),
        }
    }
}

impl Json {
    /// Creates a json object from an iterator
    ///
    /// # Errors
    /// Returns an error if the iterator ended unexpectedly or a number failed to parse.
    #[inline]
    pub fn new<T: Iterator<Item = char> + PeekingNext>(iter: &mut T) -> Result<Self, error::Error> {
        // Skip all whitespace
        iter.peeking_take_while(|ch| ch.is_whitespace()).last();

        // Take the next character
        let current = iter.next().ok_or(error::Error::UnexpectedEndOfText)?;
        match current {
            // If it's 'n', it must be null
            'n' => {
                if !compare_iter_str(iter, "ull") {
                    return Err(error::Error::InvalidJsonValue);
                }
                Ok(Self::Null)
            }

            // If it's 't', it must be true
            't' => {
                if !compare_iter_str(iter, "rue") {
                    return Err(error::Error::InvalidJsonValue);
                }
                Ok(Self::Bool(true))
            }

            // If it's 'f', it must be false
            'f' => {
                if !compare_iter_str(iter, "alse") {
                    return Err(error::Error::InvalidJsonValue);
                }
                Ok(Self::Bool(false))
            }

            // If it's a numeric character, it must be a number
            '0'..='9' | '.' | '-' => Ok(Self::Number(Self::read_number(current, iter)?)),

            // If it's '"', it must be a string
            '"' => Ok(Self::String(Self::read_string(iter))),

            // If it's '[', it must be an array
            '[' => Ok(Self::Array(Self::read_array(iter)?)),

            // If it's '{', it must be an object or dictionary
            '{' => Ok(Self::Object(Self::read_object(iter)?)),

            // Otherwise, it's invalid
            invalid_char => Err(error::Error::InvalidStartOfJsonValue(invalid_char)),
        }
    }

    /// Reads a number from a json iterator
    #[allow(clippy::single_call_fn)]
    fn read_number<T: IntoIterator<Item = char>>(first: char, iter: T) -> Result<f64, error::Error>
    where
        T::IntoIter: PeekingNext,
    {
        // Take the iterator
        iter.into_iter()
            // Only take numeric characters
            .peeking_take_while(|ch| matches!(ch, '0'..='9' | '.'))
            // Store them in a string
            .fold(first.to_string(), |mut out, value| {
                out.push(value);
                out
            })
            // parse that string as a float
            .parse::<f64>()
            .map_err(error::Error::FloatError)
    }

    /// Reads a string from a json iterator
    fn read_string(iter: &mut impl Iterator<Item = char>) -> String {
        // Stores whether the current character is escaped
        let mut escaped = false;

        // Take all characters until an unescaped '"' is found and store them in a string
        let mut result = iter
            .take_while(|&ch| {
                let should_continue = escaped || ch != '"';
                escaped = ch == '\\' && !escaped;
                should_continue
            })
            .collect::<String>();

        // Shrink the string to decrease memory usage
        result.shrink_to_fit();
        result
    }

    /// Read an array from a json iterator
    #[allow(clippy::single_call_fn)]
    fn read_array<T: Iterator<Item = char> + PeekingNext>(
        iter: &mut T,
    ) -> Result<Vec<Self>, error::Error> {
        // Create a vector
        let mut values = Vec::new();

        loop {
            // Skip all whitespace
            iter.peeking_take_while(|ch| ch.is_whitespace()).last();

            // Stop if the next character closes this array
            if iter.peeking_next(|ch| *ch == ']').is_some() {
                break;
            }

            // Add the value to the array
            values.push(Self::new(iter)?);

            // Skip all whitespace
            iter.peeking_take_while(|ch| ch.is_whitespace()).last();

            // Stop if the next character closes the array.
            // Ignore ','
            // Panic if any other character is found
            match iter.next() {
                Some(']') => break,
                Some(',') => {}
                None => return Err(error::Error::UnexpectedEndOfText),
                Some(invalid_char) => {
                    return Err(error::Error::UnexpectedCharacterInArray(invalid_char))
                }
            }
        }
        // Shrink the array
        values.shrink_to_fit();

        Ok(values)
    }

    #[allow(clippy::single_call_fn)]
    fn read_object<T: Iterator<Item = char> + PeekingNext>(
        iter: &mut T,
    ) -> Result<Object, error::Error> {
        // Create a vector
        let mut values = Object::default();

        loop {
            // Skip all whitespace
            iter.peeking_take_while(|ch| ch.is_whitespace()).last();

            // Stop if the next character closes the object
            if iter.peeking_next(|ch| *ch == '}').is_some() {
                break;
            }

            // Make sure the next character starts a string
            if iter.next() != Some('"') {
                return Err(error::Error::InvalidObjectKey);
            }

            // Read the string as the key
            let key = Self::read_string(iter);

            // Skip all whitespace
            iter.peeking_take_while(|ch| ch.is_whitespace()).last();

            // Make sure the next character is a key-value seperator
            if iter.next() != Some(':') {
                return Err(error::Error::MissingKeyValueSeparator);
            }

            // Read the value
            let value = Self::new(iter)?;

            // Add the key and value to the array
            values.add(key, value);

            // Skip all whitespace
            iter.peeking_take_while(|ch| ch.is_whitespace()).last();

            // Stop if the next character ends the object.
            // Ignore ','
            // Panic if any other character is found
            match iter.next() {
                Some('}') => break,
                Some(',') => {}
                None => return Err(error::Error::UnexpectedEndOfText),
                Some(invalid_char) => {
                    return Err(error::Error::UnexpectedCharacterInArray(invalid_char))
                }
            }
        }

        // Shrink the array
        values.shrink_to_fit();
        Ok(values)
    }
}

#[cfg(test)]
mod tests {
    use alloc::{borrow::ToOwned as _, vec};

    use crate::Json;

    #[test]
    fn null() {
        assert_eq!("null".chars().collect::<Json>(), Json::Null);
    }

    #[test]
    fn boolean() {
        assert_eq!("false".chars().collect::<Json>(), Json::Bool(false));
        assert_eq!("true".chars().collect::<Json>(), Json::Bool(true));
    }

    #[test]
    fn number() {
        assert_eq!("-0.25".chars().collect::<Json>(), Json::Number(-0.25));
    }

    #[test]
    fn string() {
        assert_eq!(
            "\"abcdefghijklm\\nopqrs\\tuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789\\\"\""
                .chars()
                .collect::<Json>(),
            Json::String(
                "abcdefghijklm\\nopqrs\\tuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789\\\"".to_owned()
            )
        );
    }

    #[test]
    fn multitype_array() {
        assert_eq!(
            "[null,true,128,\"test\"]".chars().collect::<Json>(),
            Json::Array(vec![
                Json::Null,
                Json::Bool(true),
                Json::Number(128.0),
                Json::String("test".to_owned())
            ])
        );
    }

    #[test]
    fn multitype_object() {
        assert_eq!(
            "{\"first\":null,\"second\":true,\"third\":128,\"last\":\"test\"}"
                .chars()
                .collect::<Json>(),
            Json::Object(
                vec![
                    ("first".to_owned(), Json::Null),
                    ("second".to_owned(), Json::Bool(true)),
                    ("third".to_owned(), Json::Number(128.0)),
                    ("last".to_owned(), Json::String("test".to_owned()))
                ]
                .into()
            )
        );
    }
}
