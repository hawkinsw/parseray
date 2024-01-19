pub enum EbnfToken {
    QuoteToken,
    CommaToken,
    EqualToken,
}

pub enum EbnfIdentifier { 
    Identifier(String),
}

pub struct TerminalDeclaration {}
pub struct NonTerminalDeclaration {}
