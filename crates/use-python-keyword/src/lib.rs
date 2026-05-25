#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common hard Python keywords.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PythonKeyword {
    False,
    None,
    True,
    And,
    As,
    Assert,
    Async,
    Await,
    Break,
    Class,
    Continue,
    Def,
    Del,
    Elif,
    Else,
    Except,
    Finally,
    For,
    From,
    Global,
    If,
    Import,
    In,
    Is,
    Lambda,
    Nonlocal,
    Not,
    Or,
    Pass,
    Raise,
    Return,
    Try,
    While,
    With,
    Yield,
}

impl PythonKeyword {
    /// Returns the Python source spelling for this keyword.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::False => "False",
            Self::None => "None",
            Self::True => "True",
            Self::And => "and",
            Self::As => "as",
            Self::Assert => "assert",
            Self::Async => "async",
            Self::Await => "await",
            Self::Break => "break",
            Self::Class => "class",
            Self::Continue => "continue",
            Self::Def => "def",
            Self::Del => "del",
            Self::Elif => "elif",
            Self::Else => "else",
            Self::Except => "except",
            Self::Finally => "finally",
            Self::For => "for",
            Self::From => "from",
            Self::Global => "global",
            Self::If => "if",
            Self::Import => "import",
            Self::In => "in",
            Self::Is => "is",
            Self::Lambda => "lambda",
            Self::Nonlocal => "nonlocal",
            Self::Not => "not",
            Self::Or => "or",
            Self::Pass => "pass",
            Self::Raise => "raise",
            Self::Return => "return",
            Self::Try => "try",
            Self::While => "while",
            Self::With => "with",
            Self::Yield => "yield",
        }
    }
}

impl fmt::Display for PythonKeyword {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PythonKeyword {
    type Err = PythonKeywordParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = non_empty(input)?;
        match trimmed {
            "False" => Ok(Self::False),
            "None" => Ok(Self::None),
            "True" => Ok(Self::True),
            "and" => Ok(Self::And),
            "as" => Ok(Self::As),
            "assert" => Ok(Self::Assert),
            "async" => Ok(Self::Async),
            "await" => Ok(Self::Await),
            "break" => Ok(Self::Break),
            "class" => Ok(Self::Class),
            "continue" => Ok(Self::Continue),
            "def" => Ok(Self::Def),
            "del" => Ok(Self::Del),
            "elif" => Ok(Self::Elif),
            "else" => Ok(Self::Else),
            "except" => Ok(Self::Except),
            "finally" => Ok(Self::Finally),
            "for" => Ok(Self::For),
            "from" => Ok(Self::From),
            "global" => Ok(Self::Global),
            "if" => Ok(Self::If),
            "import" => Ok(Self::Import),
            "in" => Ok(Self::In),
            "is" => Ok(Self::Is),
            "lambda" => Ok(Self::Lambda),
            "nonlocal" => Ok(Self::Nonlocal),
            "not" => Ok(Self::Not),
            "or" => Ok(Self::Or),
            "pass" => Ok(Self::Pass),
            "raise" => Ok(Self::Raise),
            "return" => Ok(Self::Return),
            "try" => Ok(Self::Try),
            "while" => Ok(Self::While),
            "with" => Ok(Self::With),
            "yield" => Ok(Self::Yield),
            _ => Err(PythonKeywordParseError::Unknown),
        }
    }
}

/// Python soft keywords used by pattern matching and newer syntax contexts.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PythonSoftKeyword {
    Match,
    Case,
    Type,
    Underscore,
}

impl PythonSoftKeyword {
    /// Returns the Python source spelling for this soft keyword.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Match => "match",
            Self::Case => "case",
            Self::Type => "type",
            Self::Underscore => "_",
        }
    }
}

impl fmt::Display for PythonSoftKeyword {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PythonSoftKeyword {
    type Err = PythonKeywordParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match non_empty(input)? {
            "match" => Ok(Self::Match),
            "case" => Ok(Self::Case),
            "type" => Ok(Self::Type),
            "_" => Ok(Self::Underscore),
            _ => Err(PythonKeywordParseError::Unknown),
        }
    }
}

/// A hard or soft reserved Python word.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PythonReservedWord {
    Keyword(PythonKeyword),
    SoftKeyword(PythonSoftKeyword),
}

impl PythonReservedWord {
    /// Returns the Python source spelling for this reserved word.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Keyword(keyword) => keyword.as_str(),
            Self::SoftKeyword(keyword) => keyword.as_str(),
        }
    }
}

impl fmt::Display for PythonReservedWord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PythonReservedWord {
    type Err = PythonKeywordParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        PythonKeyword::from_str(input).map_or_else(
            |_| PythonSoftKeyword::from_str(input).map(Self::SoftKeyword),
            |keyword| Ok(Self::Keyword(keyword)),
        )
    }
}

/// Error returned when a Python keyword label is empty or unknown.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PythonKeywordParseError {
    Empty,
    Unknown,
}

impl fmt::Display for PythonKeywordParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("Python keyword cannot be empty"),
            Self::Unknown => formatter.write_str("unknown Python keyword"),
        }
    }
}

impl Error for PythonKeywordParseError {}

/// Returns whether `input` is a hard Python keyword.
#[must_use]
pub fn is_python_keyword(input: &str) -> bool {
    input.parse::<PythonKeyword>().is_ok()
}

/// Returns whether `input` is a Python soft keyword.
#[must_use]
pub fn is_python_soft_keyword(input: &str) -> bool {
    input.parse::<PythonSoftKeyword>().is_ok()
}

/// Returns whether `input` is either a hard or soft Python reserved word.
#[must_use]
pub fn is_python_reserved_word(input: &str) -> bool {
    input.parse::<PythonReservedWord>().is_ok()
}

fn non_empty(input: &str) -> Result<&str, PythonKeywordParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(PythonKeywordParseError::Empty)
    } else {
        Ok(trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PythonKeyword, PythonKeywordParseError, PythonReservedWord, PythonSoftKeyword,
        is_python_keyword, is_python_reserved_word, is_python_soft_keyword,
    };

    #[test]
    fn parses_and_displays_hard_keywords() -> Result<(), PythonKeywordParseError> {
        assert_eq!("False".parse::<PythonKeyword>()?, PythonKeyword::False);
        assert_eq!("async".parse::<PythonKeyword>()?, PythonKeyword::Async);
        assert_eq!(PythonKeyword::Return.to_string(), "return");
        assert!(is_python_keyword("class"));
        assert!(!is_python_keyword("match"));
        Ok(())
    }

    #[test]
    fn parses_soft_keywords_and_reserved_words() -> Result<(), PythonKeywordParseError> {
        assert_eq!(
            "match".parse::<PythonSoftKeyword>()?,
            PythonSoftKeyword::Match
        );
        assert_eq!(PythonSoftKeyword::Underscore.to_string(), "_");
        assert_eq!(
            "case".parse::<PythonReservedWord>()?,
            PythonReservedWord::SoftKeyword(PythonSoftKeyword::Case)
        );
        assert!(is_python_soft_keyword("type"));
        assert!(is_python_reserved_word("lambda"));
        Ok(())
    }

    #[test]
    fn rejects_empty_and_unknown_labels() {
        assert_eq!(
            "".parse::<PythonKeyword>(),
            Err(PythonKeywordParseError::Empty)
        );
        assert_eq!(
            "FALSE".parse::<PythonKeyword>(),
            Err(PythonKeywordParseError::Unknown)
        );
    }
}
