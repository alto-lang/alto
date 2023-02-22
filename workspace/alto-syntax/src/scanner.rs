//! The lexer implementation for Alto, implemented using Parseme.

use parseme::{
    parser::{Group, GroupError},
    Error, NoMatchError, Parser, Source,
};

use crate::token::{Id, Token};

/// An error that occurred during scanning.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScanError {
    NoMatch,
}

impl Error for ScanError {
    fn is_no_match(&self) -> bool {
        matches!(self, ScanError::NoMatch)
    }
}

/// Skips any leading whitespace in the provided source.  Always returns [NoMatchError].
pub fn skip_whitespace<'a>(input: &mut Source<'a>) -> Result<(), NoMatchError> {
    parseme::iter::next_if(input, char::is_whitespace).ok_or(NoMatchError)?;
    parseme::iter::advance_while(input, char::is_whitespace);
    Ok(())
}

/// Skips any leading comments.
pub fn skip_comment<'a>(input: &mut Source<'a>) -> Result<(), NoMatchError> {
    if input.peek().ok_or(NoMatchError)? == '/' && input.peek_nth(1).ok_or(NoMatchError)? == '/' {
        input.nth(1);
    } else {
        return Err(NoMatchError);
    }

    parseme::iter::advance_until(input, |c| c == '\n' || c == '\r');
    Ok(())
}

/// Skips anything that should be skipped, such as a comment or whitespace.
pub fn skip<'a>(input: &mut Source<'a>) -> Result<Token<'a>, ScanError> {
    loop {
        if skip_whitespace(input).is_err() && skip_comment(input).is_err() {
            break;
        }
    }

    Err(ScanError::NoMatch)
}

/// Scans for a single identifier token.
pub fn scan_id<'a>(input: &mut Source<'a>) -> Result<Token<'a>, ScanError> {
    let start_pos = input.pos();

    // supports underscores as the starting character
    parseme::iter::next_if(input, |c| parseme::xid::is_start(c) || c == '_')
        .ok_or(ScanError::NoMatch)?;
    parseme::iter::advance_while(input, parseme::xid::is_continue);

    Ok(Token::Id(Id {
        span: start_pos..input.pos(),
        value: &input.src()[start_pos..input.pos()],
    }))
}

/// Initializes a lexical scanner instance.
#[inline]
pub fn new<'a>() -> impl Parser<Source<'a>, Token<'a>, GroupError<ScanError>> {
    Group::new().add(skip).add(scan_id)
}
