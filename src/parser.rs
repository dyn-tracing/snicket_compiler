use proto::grammar::*;
use protobuf::RepeatedField;
use std::iter::Peekable;
use token::Token;

type TokenIterator<'a> = Peekable<std::slice::Iter<'a, Token<'a>>>;

// Helper function to consume next token and match it against a specified token
// Throw an error if either:
// 1. token_iter is empty
// 2. the next token does not match
fn consume_token<'a>(
    token_iter: &mut TokenIterator<'a>,
    expected: &Token,
    error_msg: &'static str,
) {
    match token_iter.next() {
        None => {
            panic!(
                "Invalid token: Null, expected {:?}.\nError message: {:?}",
                expected, error_msg
            );
        }
        Some(next_token) => {
            if next_token != expected {
                panic!(
                    "\nInvalid token: {:?}, expected {:?}.\nError message: {:?}",
                    next_token, expected, error_msg
                );
            }
        }
    }
}

pub fn parse_prog<'a>(token_iter: &mut TokenIterator<'a>) -> Program {
    Program {
        patterns: parse_patterns(token_iter),
        filters: parse_filters(token_iter),
        actions: parse_actions(token_iter),
        ..Default::default()
    }
}

// Parses multiple T and returns the result as a vector.
fn parse_repeated<'a, T>(
    token_iter: &mut TokenIterator<'a>,
    parse_func: fn(&mut TokenIterator<'a>) -> T,
    expected_token: Token,
    allow_empty: bool,
) -> RepeatedField<T> {
    match token_iter.peek() {
        None => RepeatedField::<T>::new(),
        Some(&next_token) => {
            if *next_token != expected_token {
                if allow_empty {
                    return RepeatedField::<T>::new();
                } else {
                    panic!(
                        "\nInvalid token: {}, expected {}.\nError message: {} must start with the keyword {}.",
                        next_token, expected_token,
                        std::any::type_name::<T>(),
                        expected_token,
                    );
                }
            }
            // Consume next token.
            token_iter.next();
            let mut elem_vec = Vec::new();
            while let Some(Token::Identifier(_)) = token_iter.peek() {
                elem_vec.push(parse_func(token_iter));
                consume_token(token_iter, &Token::Comma, "Expected a comma (')");
            }
            RepeatedField::<T>::from_vec(elem_vec)
        }
    }
}

fn parse_patterns<'a>(token_iter: &mut TokenIterator<'a>) -> RepeatedField<Pattern> {
    parse_repeated::<Pattern>(token_iter, parse_pattern, Token::Match, false)
}

fn parse_pattern<'a>(token_iter: &mut TokenIterator<'a>) -> Pattern {
    let src_id = parse_identifier(token_iter);
    let rel_typ = match token_iter.next() {
        Some(Token::Edge) => Pattern_RelationshipType::EDGE,
        Some(Token::Path) => Pattern_RelationshipType::PATH,
        Some(token) => panic!("Unsupported relationship type: {:?}", token),
        None => panic!("Expected relationship type token, found none."),
    };
    let dst_id = parse_identifier(token_iter);
    consume_token(
        token_iter,
        &Token::Colon,
        "Pattern must end with relationship name.",
    );
    let rel_id = parse_identifier(token_iter);
    Pattern {
        src_id,
        dst_id,
        rel_typ,
        rel_id,
        ..Default::default()
    }
}

fn parse_filters<'a>(token_iter: &mut TokenIterator<'a>) -> RepeatedField<Filter> {
    parse_repeated::<Filter>(token_iter, parse_filter, Token::Where, true)
}

fn parse_filter<'a>(token_iter: &mut TokenIterator<'a>) -> Filter {
    let id = parse_identifier(token_iter);

    match token_iter.next() {
        None => {
            panic!("Expected period, got none.");
        }
        Some(Token::Period) => {
            let mut properties = RepeatedField::<String>::new();
            let mut property = parse_identifier(token_iter);
            properties.push(property);
            while let Some(Token::Period) = token_iter.peek() {
                // Consume token period
                token_iter.next();
                property = parse_identifier(token_iter);
                properties.push(property);
            }
            consume_token(
                token_iter,
                &Token::Equals,
                "Only support equality in properties.",
            );
            let value_oneof = parse_value(token_iter);

            Filter {
                id,
                properties,
                value_oneof,
                ..Default::default()
            }
        }
        Some(token) => panic!("Unrecognized token: {:?}", token),
    }
}

fn parse_actions<'a>(token_iter: &mut TokenIterator<'a>) -> RepeatedField<Action> {
    parse_repeated::<Action>(token_iter, parse_action, Token::Return, true)
}

fn parse_action<'a>(token_iter: &mut TokenIterator<'a>) -> Action {
    let id = parse_identifier(token_iter);
    match token_iter.next() {
        None => {
            panic!("Expected token period, got none.");
        }
        Some(Token::Period) => {
            let mut properties = RepeatedField::<String>::new();
            let mut property = parse_identifier(token_iter);
            properties.push(property);
            while let Some(Token::Period) = token_iter.peek() {
                // Consume token period
                token_iter.next();
                property = parse_identifier(token_iter);
                properties.push(property);
            }
            Action {
                id,
                properties,
                ..Default::default()
            }
        }
        Some(token) => panic!("Unrecognized token: {:?}", token),
    }
}

fn parse_identifier<'a>(token_iter: &mut TokenIterator<'a>) -> String {
    let identifier_token = token_iter.next().unwrap();
    match &identifier_token {
        Token::Identifier(id_name) => id_name.to_string(),
        _ => panic!(
            "Invalid token: {:?}, expected Token::Identifier",
            identifier_token
        ),
    }
}

fn parse_value<'a>(token_iter: &mut TokenIterator<'a>) -> Option<Filter_oneof_value_oneof> {
    let value_token = token_iter.next().unwrap();
    match &value_token {
        Token::Value(value) => Some(Filter_oneof_value_oneof::u32(*value)),
        Token::Identifier(s) => Some(Filter_oneof_value_oneof::str(s.to_string())),
        _ => panic!(
            "Invalid token: {:?}, expected Token::Value or Token::Identifier",
            value_token
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::super::lexer::get_tokens;
    use super::*;

    // Macro to test that parser parses successfully
    macro_rules! test_parser_success {
        ($input_code:expr,$parser_routine:ident,$test_name:ident) => {
            #[test]
            fn $test_name() {
                let input = $input_code;
                let tokens = &mut get_tokens(input);
                let token_iter = &mut tokens.iter().peekable();
                println!("{:?}", $parser_routine(token_iter));
                assert!(token_iter.peek().is_none(), "token iterator is not empty");
            }
        };
    }

    // Macro to test that parser fails to parse with correct panic message
    macro_rules! test_parser_fail {
        ($input_code:expr,$parser_routine:ident,$test_name:ident,$panic_msg:expr) => {
            #[test]
            #[should_panic(expected=$panic_msg)]
            fn $test_name() {
                let input = $input_code;
                let tokens = &mut get_tokens(input);
                let token_iter = &mut tokens.iter().peekable();
                println!("{:?}", $parser_routine(token_iter));
                assert!(token_iter.peek().is_none(), "token iterator is not empty");
            }
        };
    }

    test_parser_success!(r"5", parse_value, test_parse_value);
    test_parser_success!(r"absc", parse_identifier, test_parse_identifier);
    test_parser_fail!(
        r"5",
        parse_identifier,
        test_parse_identifier_fail,
        "Invalid token: Value(5), expected Token::Identifier"
    );
    test_parser_success!(r"MATCH n-->m : a,", parse_patterns, test_parse_pattern1);
    test_parser_success!(r"MATCH n-*>m : b,", parse_patterns, test_parse_pattern2);
    test_parser_fail!(
        r"n",
        parse_patterns,
        test_parse_pattern_fail,
        "Pattern must start with the keyword MATCH."
    );

    test_parser_success!(r"WHERE n.abc == 5,", parse_filters, test_parse_filter);

    test_parser_fail!(
        r"n",
        parse_filter,
        test_parse_filter_fail,
        "Expected period, got none."
    );
    test_parser_success!(
        r"MATCH n-->m : a, n-*>m : b, WHERE n.abc == 5,",
        parse_prog,
        test_parse_prog
    );
    test_parser_success!(
        r"MATCH n-->m : a, n-*>m : b,",
        parse_prog,
        test_parse_prog_no_filter
    );
    test_parser_fail!(
        r"n-->m",
        parse_prog,
        test_parse_prog_fail,
        "Pattern must start with the keyword MATCH."
    );
    test_parser_success!(
        r"MATCH n-->m :a, WHERE n.x.y.z == 5,",
        parse_prog,
        test_parse_nested_properties
    );
    test_parser_success!(
        r"MATCH n-->m: a, WHERE n.x == k,",
        parse_prog,
        test_parse_str_value
    );

    #[test]
    fn test_parse_prog_str_value() {
        let input = r"MATCH n-->m: a, WHERE n.x == k,";
        let tokens = &mut get_tokens(input);
        let token_iter = &mut tokens.iter().peekable();
        let prog = parse_prog(token_iter);
        assert_eq!(
            prog,
            Program {
                patterns: RepeatedField::<Pattern>::from_vec(vec![Pattern {
                    src_id: "n".to_string(),
                    dst_id: "m".to_string(),
                    rel_typ: Pattern_RelationshipType::EDGE,
                    rel_id: "a".to_string(),
                    ..Default::default()
                }]),
                filters: RepeatedField::<Filter>::from_vec(vec![Filter {
                    id: "n".to_string(),
                    properties: RepeatedField::<String>::from_vec(vec!["x".to_string()]),
                    value_oneof: Some(Filter_oneof_value_oneof::str("k".to_string())),
                    ..Default::default()
                }]),
                actions: RepeatedField::<Action>::new(),
                ..Default::default()
            }
        )
    }

    test_parser_success!(r"RETURN n.x,", parse_actions, test_parse_return_action);
    #[test]
    fn test_parse_actions_return() {
        let input = r"RETURN n.x,";
        let tokens = &mut get_tokens(input);
        let token_iter = &mut tokens.iter().peekable();
        let actions = parse_actions(token_iter);
        assert_eq!(
            actions,
            RepeatedField::<Action>::from_vec(vec![Action {
                id: "n".to_string(),
                properties: RepeatedField::<String>::from_vec(vec!["x".to_string()]),
                ..Default::default()
            }])
        );
    }
    test_parser_success!(
        r"MATCH n-->m: a, WHERE n.y == o, RETURN n.x,",
        parse_prog,
        test_parse_return
    );

    #[test]
    fn test_parse_action_without_where() {
        let input = r"MATCH n-->m: a, RETURN n.x,";
        let tokens = &mut get_tokens(input);
        let token_iter = &mut tokens.iter().peekable();
        let prog = parse_prog(token_iter);
        assert_eq!(
            prog,
            Program {
                patterns: RepeatedField::<Pattern>::from_vec(vec![Pattern {
                    src_id: "n".to_string(),
                    dst_id: "m".to_string(),
                    rel_typ: Pattern_RelationshipType::EDGE,
                    rel_id: "a".to_string(),
                    ..Default::default()
                }]),
                filters: RepeatedField::<Filter>::new(),
                actions: RepeatedField::<Action>::from_vec(vec![Action {
                    id: "n".to_string(),
                    properties: RepeatedField::<String>::from_vec(vec!["x".to_string()]),
                    ..Default::default()
                }]),
                ..Default::default()
            }
        )
    }
}
