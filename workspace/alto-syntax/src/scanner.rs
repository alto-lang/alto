//! The lexer implementation for Alto, implemented using Parseme.

use parseme::{
    parser::{Group, GroupError},
    Error, NoMatchError, Parser, Source,
};

use crate::{
    token::{
        Id, Inst, InstKind, Keyword, KeywordKind, Punct, PunctKind, Symbol, SymbolKind, Token,
    },
    Span,
};

/// An error that occurred during scanning.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ScanError {
    /// No match could be found.
    NoMatch,

    /// A character was invalid.
    UnrecognizedCharacter { pos: Span },

    /// A symbol name was expected after an `@` symbol.
    SymbolNameExpected {
        /// The position of the `@` before the symbol name.
        pos: usize,
    },

    /// Expected a letter or underscore after the `@` symbol.
    InvalidSymbolName {
        /// The position of the offending character.
        pos: usize,
    },
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

/// Scans for a single symbol token.
pub fn scan_symbol<'a>(input: &mut Source<'a>) -> Result<Token<'a>, ScanError> {
    let start_pos = input.pos();

    parseme::iter::next_if(input, |c| c == '@').ok_or(ScanError::NoMatch)?;

    if !parseme::xid::is_continue(
        input
            .next()
            .ok_or(ScanError::SymbolNameExpected { pos: start_pos })?,
    ) {
        return Err(ScanError::InvalidSymbolName { pos: start_pos + 1 });
    }

    parseme::iter::advance_while(input, parseme::xid::is_continue);

    Ok(Token::Symbol(Symbol {
        span: start_pos..input.pos(),
        kind: SymbolKind::Id,
        value: &input.src()[start_pos..input.pos()],
    }))
}

/// Scans for a single identifier token.
pub fn scan_id<'a>(input: &mut Source<'a>) -> Result<Id<'a>, ScanError> {
    let start_pos = input.pos();

    // supports underscores as the starting character
    parseme::iter::next_if(input, |c| parseme::xid::is_start(c) || c == '_')
        .ok_or(ScanError::NoMatch)?;
    parseme::iter::advance_while(input, parseme::xid::is_continue);

    Ok(Id {
        span: start_pos..input.pos(),
        value: &input.src()[start_pos..input.pos()],
    })
}

/// Scans for a single identifier or keyword token.
pub fn scan_id_or_keyword<'a>(input: &mut Source<'a>) -> Result<Token<'a>, ScanError> {
    let id = scan_id(input)?;

    Ok(Token::Inst(Inst {
        span: id.span.clone(),
        kind: match id.value.to_lowercase().as_str() {
            "ret" => InstKind::Ret,
            _ => {
                return Ok(Token::Keyword(Keyword {
                    span: id.span.clone(),
                    kind: match id.value {
                        "fn" => KeywordKind::Fn,
                        _ => return Ok(Token::Id(id)),
                    },
                }))
            }
        },
    }))
    // Ok(Token::Keyword(Keyword {
    //     span: id.span.clone(),
    //     kind: match id.value {
    //         "fn" => KeywordKind::Fn,
    //         _ => return Ok(Token::Id(id)),
    //     },
    // }))
}

/// Scans for a single arrow token.
pub fn scan_arrow<'a>(input: &mut Source<'a>) -> Result<Token<'a>, ScanError> {
    let start_pos = input.pos();

    if input.peek() != Some('-') && input.peek_nth(1) != Some('>') {
        return Err(ScanError::NoMatch);
    }

    input.nth(1);

    Ok(Token::Punct(Punct {
        span: start_pos..input.pos(),
        kind: PunctKind::Arrow,
    }))
}

/// Scans any single-character punctuation token.
pub fn scan_single_punct<'a>(input: &mut Source<'a>) -> Result<Token<'a>, ScanError> {
    let start_pos = input.pos();
    let kind = match input.peek().ok_or(ScanError::NoMatch)? {
        '(' => PunctKind::OpenParen,
        ')' => PunctKind::CloseParen,
        ':' => PunctKind::Colon,
        _ => return Err(ScanError::NoMatch),
    };

    input.next();

    Ok(Token::Punct(Punct {
        span: start_pos..input.pos(),
        kind,
    }))
}

/// Catches any unrecognized characters.
pub fn catchall<'a>(input: &mut Source<'a>) -> Result<Token<'a>, ScanError> {
    let start_pos = input.pos();

    input.next().ok_or(ScanError::NoMatch)?;

    Err(ScanError::UnrecognizedCharacter {
        pos: start_pos..input.pos(),
    })
}

/// Initializes a lexical scanner instance.
#[inline]
pub fn new<'a>() -> impl Parser<Source<'a>, Token<'a>, GroupError<ScanError>> {
    Group::new()
        .add(skip)
        .add(scan_symbol)
        .add(scan_id_or_keyword)
        .add(scan_arrow)
        .add(scan_single_punct)
        .add(catchall)
}
