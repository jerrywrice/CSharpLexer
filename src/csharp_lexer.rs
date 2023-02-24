pub mod csharp_lexer {

    pub struct CsLexer {
        pub filename: String,
        pub tokens: Vec<Token>,
    }

    impl CsLexer {
        pub fn new() -> Self {
            CsLexer {
                filename: "".to_string(),
                tokens: Vec::new(),
            }
        }
    }

    impl Default for CsLexer {
        fn default() -> Self {
            CsLexer {
                filename: "".to_string(),
                tokens: Vec::new(),
            }
        }
    }

    #[derive(Clone)]
    pub struct Token {
        pub line_num: u32,
        pub line_column: u32,
        pub info: TokenKind,
    }

    impl Token {
        pub fn new(lnum: u32, lcolumn: u32, inf: TokenKind) -> Self {
            Token {
                line_num: lnum,
                line_column: lcolumn,
                info: inf,
            }
        }
    }

    impl Default for Token {
        fn default() -> Self {
            Token {
                line_num: 0,
                line_column: 0,
                info: TokenKind::None,
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    pub enum IntLiteralDenotedType {
        TypI32,
        TypU32,
        TypI64,
        TypU64,
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    pub enum RealLititeralDenotedType {
        TypF32,
        TypF64,
        TypDecimal,
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    pub enum RealLiteralActualValueType {
        FloatVal(f64),                  // f32 and f64 values
        DecimalVal(i32, i32, i32, i32), // decimal values i32[4]
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    pub enum CharacterLiteralDenotedType {
        TypUtf8,
        TypUtf,
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    pub enum OperatorOrPunctuatorType {
        LeftCurlyBracket,
        RightCurlyBracket,
        LeftSquareBracket,
        RightSquareBracket,
        LeftParenthesis,
        RightParenthesis,
        Dot,
        Comma,
        Colon,
        Semicolon,
        Plus,
        Minus,
        Asterisk, /* '*' */
        Slash,    /* '/' */
        Percent,
        And,             /* '&' */
        Or,              /* '|' */
        Xor,             /* '^' */
        Exclamation,     /* '!' */
        Tilde,           /* '~' */
        Assign,          /* '=' */
        LessThan,        /* '<' */
        GreaterThan,     /* '>' */
        QMark,           /* '?' */
        QMarkQMark,      /* "??" */
        ColonColon,      /* "::" */
        PlusPlus,        /* "++" */
        MinusMinus,      /* "--" */
        AndAnd,          /* "&&" */
        OrOr,            /* "||" */
        AndAssign,       /* "&=" */
        OrAssign,        /* "|=" */
        XorAssign,       /* "^=" */
        ShiftLeft,       /* "<<" */
        ShiftLeftAssign, /* "<<=" */
        RightArrow,      /* "=>" */
                         /* Note that the ">>" and ">>=* operators are not lexical tokens in c#, rather these are grammatical constructs.
                            This is because the c# template definition and usage syntax uses ">>" when closing certain template argument sequences, which involves contextual (semantic)
                            understanding for the lexer to recognize.
                          */
    }

    #[allow(dead_code)]
    #[derive(Clone)]
    pub enum TokenKind {
        None, // Indicates no token is set
        Identifier {
            identifier_val: String,
        },
        Keyword {
            keyword_val: String,
        },
        BoolLiteral {
            bool_val: bool,
        },
        IntegerLiteral {
            int_val: i128,
            denoted_type: IntLiteralDenotedType,
        },
        RealLiteral {
            real_val: RealLiteralActualValueType,
            denoted_type: RealLititeralDenotedType,
        },
        CharacterLiteral {
            char_val: char,
            denoted_type: CharacterLiteralDenotedType,
        },
        StringLiteral {
            string_val: String,
        },
        NullLiteral,
        OperatorOrPunctuator {
            op_punctuator_val: OperatorOrPunctuatorType,
        },
    }
}
