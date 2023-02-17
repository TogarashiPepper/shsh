use logos::Logos;

#[derive(Logos, Debug, Copy, Clone, PartialEq)]
enum Token {
    #[regex(r"\d+")]
    Number,

    #[token("fun")]
    FnKw,

    #[token("let")]
    Let,

    #[regex("[A-Za-z_][A-Za-z0-9]+")]
    Ident,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[regex(" +")]
    Whitespace,

    #[error]
    Error
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! gen_test {
        ($test_name:ident, $target:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                assert($target, $expected);
            }
        };
    }

    fn assert(input: &str, kind: Token) {
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(kind));
        assert_eq!(lexer.slice(), input);
    }

    gen_test!(whitespace, " ", Token::Whitespace);

    gen_test!(number, "123456789", Token::Number);

    gen_test!(fnkw, "fn", Token::FnKw);

    gen_test!(r#let, "let", Token::Let);

    gen_test!(plus, "+", Token::Plus);

    gen_test!(minus, "-", Token::Minus);

    gen_test!(star, "*", Token::Star);

    gen_test!(slash, "/", Token::Slash);

    gen_test!(lparen, "(", Token::LParen);

    gen_test!(rparen, ")", Token::RParen);

    gen_test!(lbrace, "{", Token::LBrace);

    gen_test!(rbrace, "}", Token::RBrace);

    #[test]
    fn ident() {
        assert("identifier", Token::Ident);
        assert("Identifier", Token::Ident);
        assert("iDenTifier123", Token::Ident);
    }
}
