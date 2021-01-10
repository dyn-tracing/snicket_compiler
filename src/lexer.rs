use regex::Regex;
use regex::RegexSet;

lazy_static! {
    static ref TOKENS: Regex =
        Regex::new(r"[0-9]+|[A-Za-z_][A-Za-z0-9_]*|-->|-\*>|:|,|\.|==|\S+").unwrap();
    static ref KEYWORDS: Regex = Regex::new(r"^(MATCH|WHERE|RETURN|GROUP|BY)$").unwrap();
    static ref IDENTIFIERS: RegexSet =
        RegexSet::new(&[r"^[A-Za-z][A-Za-z0-9_]*$", r#"^"[A-Za-z][A-Za-z0-9_-]*"$"#]).unwrap();
    static ref VALUES: Regex = Regex::new(r"^([0-9]+)$").unwrap();
}

use crate::token::Token;
fn get_single_token(tok_str: &str) -> Token {
    if KEYWORDS.is_match(tok_str) {
        match tok_str {
            "MATCH" => Token::Match,
            "WHERE" => Token::Where,
            "RETURN" => Token::Return,
            "GROUP" => Token::Group,
            "BY" => Token::By,
            _ => panic!("Unrecognized keyword string: {}", tok_str),
        }
    } else if IDENTIFIERS.is_match(tok_str) {
        Token::Identifier(tok_str)
    } else if VALUES.is_match(tok_str) {
        Token::Value(tok_str.parse::<u32>().unwrap())
    } else {
        match tok_str {
            "-->" => Token::Edge,
            "-*>" => Token::Path,
            ":" => Token::Colon,
            "." => Token::Period,
            "==" => Token::Equals,
            "," => Token::Comma,
            _ => panic!("Unrecognized token string: {}", tok_str),
        }
    }
}

pub fn get_tokens(input_program: &str) -> Vec<Token> {
    let mut token_array = Vec::new();
    for cap in TOKENS.captures_iter(input_program) {
        let tok_str = cap.get(0).unwrap().as_str();
        token_array.push(get_single_token(tok_str));
    }
    token_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_1() {
        let input_program = r"MATCH n-->m";
        assert_eq!(
            get_tokens(input_program),
            vec![
                Token::Match,
                Token::Identifier("n"),
                Token::Edge,
                Token::Identifier("m")
            ]
        );
    }

    #[test]
    fn test_lexer_2() {
        let input_program = r"MATCH n-->m WHERE n:Person";
        assert_eq!(
            get_tokens(input_program),
            vec![
                Token::Match,
                Token::Identifier("n"),
                Token::Edge,
                Token::Identifier("m"),
                Token::Where,
                Token::Identifier("n"),
                Token::Colon,
                Token::Identifier("Person")
            ]
        );
    }

    #[test]
    fn test_lexer_3() {
        let input_program = r"MATCH n-->m WHERE n:Swedish WHERE n:Person";
        assert_eq!(
            get_tokens(input_program),
            vec![
                Token::Match,
                Token::Identifier("n"),
                Token::Edge,
                Token::Identifier("m"),
                Token::Where,
                Token::Identifier("n"),
                Token::Colon,
                Token::Identifier("Swedish"),
                Token::Where,
                Token::Identifier("n"),
                Token::Colon,
                Token::Identifier("Person"),
            ]
        );
    }

    #[test]
    fn test_lexer_4() {
        let input_program = r"MATCH n-->m WHERE n.name == 5";
        assert_eq!(
            get_tokens(input_program),
            vec![
                Token::Match,
                Token::Identifier("n"),
                Token::Edge,
                Token::Identifier("m"),
                Token::Where,
                Token::Identifier("n"),
                Token::Period,
                Token::Identifier("name"),
                Token::Equals,
                Token::Value(5),
            ]
        );
    }

    #[test]
    fn test_lexer_5() {
        let input_program = r"MATCH m-->n MATCH n-->o";
        assert_eq!(
            get_tokens(input_program),
            vec![
                Token::Match,
                Token::Identifier("m"),
                Token::Edge,
                Token::Identifier("n"),
                Token::Match,
                Token::Identifier("n"),
                Token::Edge,
                Token::Identifier("o"),
            ]
        );
    }

    #[test]
    fn test_lexer_6() {
        let input_program = r"MATCH n-*>m MATCH m-*>o WHERE n.abc == 10";
        assert_eq!(
            get_tokens(input_program),
            vec![
                Token::Match,
                Token::Identifier("n"),
                Token::Path,
                Token::Identifier("m"),
                Token::Match,
                Token::Identifier("m"),
                Token::Path,
                Token::Identifier("o"),
                Token::Where,
                Token::Identifier("n"),
                Token::Period,
                Token::Identifier("abc"),
                Token::Equals,
                Token::Value(10)
            ]
        );
    }

    #[test]
    fn test_lexer_7() {
        let input_program = r"MATCH n-->m WHERE m-*>x";
        assert_eq!(
            get_tokens(input_program),
            vec![
                Token::Match,
                Token::Identifier("n"),
                Token::Edge,
                Token::Identifier("m"),
                Token::Where,
                Token::Identifier("m"),
                Token::Path,
                Token::Identifier("x"),
            ]
        );
    }

    #[test]
    fn test_lexer_8() {
        let input_program = r"MATCH n-->m WHERE m.abc == 5";
        assert_eq!(
            get_tokens(input_program),
            vec![
                Token::Match,
                Token::Identifier("n"),
                Token::Edge,
                Token::Identifier("m"),
                Token::Where,
                Token::Identifier("m"),
                Token::Period,
                Token::Identifier("abc"),
                Token::Equals,
                Token::Value(5)
            ]
        );
    }

    #[test]
    fn test_lexer_9() {
        let input_program = r"MATCH n-*>m";
        assert_eq!(
            get_tokens(input_program),
            vec![
                Token::Match,
                Token::Identifier("n"),
                Token::Path,
                Token::Identifier("m"),
            ],
        );
    }

    #[test]
    fn test_lexer_10() {
        let input_program = r"MATCH l-->m MATCH n-->m";
        assert_eq!(
            get_tokens(input_program),
            vec![
                Token::Match,
                Token::Identifier("l"),
                Token::Edge,
                Token::Identifier("m"),
                Token::Match,
                Token::Identifier("n"),
                Token::Edge,
                Token::Identifier("m"),
            ]
        );
    }

    #[test]
    fn test_lexer_11() {
        let input_program: &str =
            r"MATCH a-->b MATCH b-->c WHERE a.path == 1 WHERE b.path == 2 WHERE c.path == 3";
        assert_eq!(
            get_tokens(input_program),
            vec![
                Token::Match,
                Token::Identifier("a"),
                Token::Edge,
                Token::Identifier("b"),
                Token::Match,
                Token::Identifier("b"),
                Token::Edge,
                Token::Identifier("c"),
                Token::Where,
                Token::Identifier("a"),
                Token::Period,
                Token::Identifier("path"),
                Token::Equals,
                Token::Value(1),
                Token::Where,
                Token::Identifier("b"),
                Token::Period,
                Token::Identifier("path"),
                Token::Equals,
                Token::Value(2),
                Token::Where,
                Token::Identifier("c"),
                Token::Period,
                Token::Identifier("path"),
                Token::Equals,
                Token::Value(3),
            ]
        );
    }

    #[test]
    fn test_lexer_12() {
        let input_program: &str = r"MATCH a-->b MATCH b-->c MATCH a-->d";
        assert_eq!(
            get_tokens(input_program),
            vec![
                Token::Match,
                Token::Identifier("a"),
                Token::Edge,
                Token::Identifier("b"),
                Token::Match,
                Token::Identifier("b"),
                Token::Edge,
                Token::Identifier("c"),
                Token::Match,
                Token::Identifier("a"),
                Token::Edge,
                Token::Identifier("d"),
            ]
        );
    }

    #[test]
    fn test_lexer_nested_properties() {
        let input = r"WHERE a.x.y == 5";
        assert_eq!(
            get_tokens(input),
            vec![
                Token::Where,
                Token::Identifier("a"),
                Token::Period,
                Token::Identifier("x"),
                Token::Period,
                Token::Identifier("y"),
                Token::Equals,
                Token::Value(5),
            ]
        );
    }

    #[test]
    fn test_lexer_str_value() {
        let input = "WHERE a.x == k";
        assert_eq!(
            get_tokens(input),
            vec![
                Token::Where,
                Token::Identifier("a"),
                Token::Period,
                Token::Identifier("x"),
                Token::Equals,
                Token::Identifier("k"),
            ]
        );
    }

    #[test]
    fn test_lexer_return() {
        let input = "RETURN a.x.y";
        assert_eq!(
            get_tokens(input),
            vec![
                Token::Return,
                Token::Identifier("a"),
                Token::Period,
                Token::Identifier("x"),
                Token::Period,
                Token::Identifier("y")
            ]
        )
    }
}
