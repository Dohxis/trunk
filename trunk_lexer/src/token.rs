use std::fmt::Display;

pub type Span = (usize, usize);

#[derive(Debug, PartialEq, Clone)]
pub enum OpenTagKind {
    Full,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Abstract,
    Ampersand,
    And,
    AndEqual,
    Array,
    ArrayCast,
    Arrow,
    As,
    Asterisk,
    Attribute,
    Bang,
    BoolCast,
    BooleanAnd,
    BooleanOr,
    Break,
    Callable,
    Caret,
    Case,
    Catch,
    Class,
    ClassConstant,
    Clone,
    CloseTag,
    Coalesce,
    CoalesceEqual,
    Colon,
    Comma,
    Comment(String),
    ConcatEqual,
    Const,
    ConstantString(String),
    Continue,
    CurlyOpen,
    Declare,
    Decrement,
    Default,
    DirConstant,
    DivEqual,
    Do,
    DocComment,
    DocOpen(String),
    Dot,
    DoubleArrow,
    DoubleCast,
    DoubleColon,
    DoubleEquals,
    Echo,
    Ellipsis,
    Else,
    ElseIf,
    Empty,
    EndDeclare,
    EndFor,
    EndForeach,
    EndIf,
    EndSwitch,
    EndWhile,
    Enum,
    Eof,
    Equals,
    Extends,
    False,
    Final,
    Float(f64),
    Fn,
    FullyQualifiedIdentifier(String),
    Function,
    GreaterThan,
    GreaterThanEquals,
    Identifier(String),
    If,
    Implements,
    InlineHtml(String),
    Int(i64),
    Interface,
    LeftBrace,
    LeftBracket,
    LeftParen,
    LeftShift,
    LessThan,
    LessThanEquals,
    Minus,
    Namespace,
    NamespaceSeparator,
    New,
    Null,
    OpenTag(OpenTagKind),
    Percent,
    Pipe,
    Plus,
    Pow,
    Private,
    Protected,
    Public,
    QualifiedIdentifier(String),
    Question,
    Return,
    RightBrace,
    RightBracket,
    RightParen,
    SemiColon,
    Slash,
    Static,
    Trait,
    TripleEquals,
    True,
    Use,
    Var,
    Variable(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Default for Token {
    fn default() -> Self {
        Self { kind: TokenKind::Eof, span: (0, 0) }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Abstract => "abstract",
            Self::Ampersand => "&",
            Self::And => "&&",
            Self::AndEqual => "&=",
            Self::Arrow => "->",
            Self::Array => "array",
            Self::ArrayCast => "(array)",
            Self::As => "as",
            Self::Asterisk => "*",
            Self::Attribute => "#[",
            Self::Bang => "!",
            Self::BoolCast => "(bool)",
            Self::BooleanAnd => "&&",
            Self::BooleanOr => "||",
            Self::Break => "break",
            Self::Callable => "callable",
            Self::Caret => "^",
            Self::Case => "case",
            Self::Catch => "catch",
            Self::Class => "class",
            Self::ClassConstant => "__CLASS__",
            Self::Clone => "clone",
            Self::CloseTag => "?>",
            Self::Coalesce => "??",
            Self::CoalesceEqual => "??=",
            Self::Colon => ":",
            Self::Comma => ",",
            Self::Comment(comment) => &comment[..],
            Self::ConcatEqual => ".=",
            Self::Const => "const",
            Self::ConstantString(comment) => &comment[..],
            Self::Continue => "continue",
            Self::CurlyOpen => "{$",
            Self::Declare => "declare",
            Self::Decrement => "--",
            Self::Default => "default",
            Self::DirConstant => "__DIR__",
            Self::DivEqual => "/=",
            Self::Do => "do",
            Self::DocComment => todo!(),
            Self::DocOpen(doc_open) => &doc_open[..],
            Self::Dot => ".",
            Self::DoubleArrow => "=>",
            Self::DoubleCast => "(float)",
            Self::DoubleColon => "::",
            Self::DoubleEquals => "==",
            Self::Echo => "echo",
            Self::Ellipsis => "...",
            Self::Else => "else",
            Self::ElseIf => "elseif",
            Self::Empty => "empty",
            Self::EndDeclare => "enddeclare",
            Self::EndFor => "endfor",
            Self::EndForeach => "endforeach",
            Self::EndIf => "endif",
            Self::EndSwitch => "endswitch",
            Self::EndWhile => "endwhile",
            Self::Enum => "enum",
            Self::Eof => "",
            Self::Equals => "=",
            Self::Extends => "extends",
            Self::False => "false",
            Self::Final => "final",
            Self::Float(_) => "float",
            Self::Fn => "fn",
            Self::FullyQualifiedIdentifier(id) => &id[..],
            Self::Function => "function",
            Self::GreaterThan => ">",
            Self::GreaterThanEquals => ">=",
            Self::Identifier(id) => &id[..],
            Self::If => "if",
            Self::Implements => "implements",
            Self::InlineHtml(_) => "InlineHtml",
            Self::Int(_) => "int",
            Self::LeftBrace => "{",
            Self::LeftBracket => "[",
            Self::LeftParen => "(",
            Self::LeftShift => "<<",
            Self::LessThan => "<",
            Self::LessThanEquals => "<=",
            Self::Minus => "-",
            Self::Namespace => "namespace",
            Self::NamespaceSeparator => "\\",
            Self::New => "new",
            Self::Null => "null",
            Self::OpenTag(kind) => match kind {
                OpenTagKind::Full => "<?php",
            },
            Self::Percent => "%",
            Self::Pipe => "|",
            Self::Plus => "+",
            Self::Pow => "**",
            Self::Private => "private",
            Self::Protected => "protected",
            Self::Public => "public",
            Self::QualifiedIdentifier(id) => &id[..],
            Self::Question => "?",
            Self::Return => "return",
            Self::RightBrace => "}",
            Self::RightBracket => "]",
            Self::RightParen => ")",
            Self::SemiColon => ";",
            Self::Slash => "/",
            Self::Static => "static",
            Self::Trait => "trait",
            Self::TripleEquals => "===",
            Self::True => "true",
            Self::Use => "use",
            Self::Var => "var",
            Self::Variable(var) => &var[..],
            _ => todo!("format token: {:?}", self)
        })
    }
}