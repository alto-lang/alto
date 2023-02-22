//! The lexer, parser and abstract syntax trees for Alto.

pub use parseme;
pub mod scanner;
pub mod token;

/// The location of an expression in the source code.
pub type Span = std::ops::Range<usize>;
