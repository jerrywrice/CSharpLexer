use crate::csharp_lexer::csharp_lexer::{CsLexer, Token, TokenKind};

mod csharp_lexer;

fn main() {
    // Demo code
    let mut csharp_lex_analyzer = CsLexer::new();
    let str_count = csharp_lex_analyzer.tokens.len();
    let my_first_token = Token::new(1, 1, TokenKind::Identifier {identifier_val : "Index".to_string()});
    csharp_lex_analyzer.tokens.push(my_first_token.clone());

    println!("Initial analyzer token count is {str_count}.");
}
