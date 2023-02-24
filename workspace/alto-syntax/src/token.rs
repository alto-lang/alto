use crate::Span;

/// An identifier token.
///
/// ```alto
/// _identifier
/// identifier
/// i123
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Id<'a> {
    pub span: Span,
    pub value: &'a str,
}

/// The kind of a [Symbol]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum SymbolKind {
    /// An identifier was used as the symbol's value.
    Id,

    /// A string was used as the symbol's value.
    Str,
}

/// The name of a symbol.
///
/// ```alto
/// @main
/// @"str" // TODO
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Symbol<'a> {
    pub span: Span,
    pub kind: SymbolKind,
    pub value: &'a str,
}

/// The kind of a [Keyword].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum KeywordKind {
    Fn,
}

/// Any keyword token.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Keyword {
    pub span: Span,
    pub kind: KeywordKind,
}

/// The kind of a [Punct].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum PunctKind {
    OpenParen,
    CloseParen,
    /// `->`
    Arrow,
    Colon,
}

/// Any punctuation token.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Punct {
    pub span: Span,
    pub kind: PunctKind,
}

/// The kind of an [Inst] token.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum InstKind {
    Ret,
}

/// Any instruction token.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Inst {
    pub span: Span,
    pub kind: InstKind,
}

/// Tokens produced by the Alto lexer.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Token<'a> {
    Id(Id<'a>),
    Symbol(Symbol<'a>),
    Keyword(Keyword),
    Punct(Punct),
    Inst(Inst),
}
