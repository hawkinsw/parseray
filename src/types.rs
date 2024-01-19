pub enum ArrayToken {
    LetToken,
    MutToken,
    SemicolonToken,
    ColonToken,
    RBracketToken,
    LBracketToken,
    EqualToken,
    Comma,
}

pub enum ElementType {
    Ri32,
    Ru32,
    Ri64,
    Ru64,
}

pub struct ArrayDeclaration {}
pub struct ArrayType {}
pub struct ArrayInitializer{}
