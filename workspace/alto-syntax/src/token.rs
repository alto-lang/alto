use crate::Span;

/// An identifier token.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Id<'a> {
    pub span: Span,
    pub value: &'a str,
}

/// Tokens produced by the Alto lexer.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Token<'a> {
    Id(Id<'a>),
}
