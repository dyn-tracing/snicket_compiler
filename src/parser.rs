use grammar::*;
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

pub fn parse_prog<'a>(token_iter: &mut TokenIterator<'a>) -> Prog<'a> {
    Prog {
        patterns: parse_patterns(token_iter),
        filters: parse_filters(token_iter),
        action: parse_action(token_iter),
    }
}

// Parses multiple T and returns the result as a vector.
fn parse_repeated<'a, T>(
    token_iter: &mut TokenIterator<'a>,
    parse_func: fn(&mut TokenIterator<'a>) -> T,
    expected_token: Token,
    allow_empty: bool,
) -> Vec<T> {
    match token_iter.peek() {
        None => Vec::new(),
        Some(&next_token) => {
            if *next_token != expected_token {
                if allow_empty {
                    return Vec::new();
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
                consume_token(token_iter, &Token::Comma, "Expected a comma (,)");
            }
            elem_vec
        }
    }
}

fn parse_patterns<'a>(token_iter: &mut TokenIterator<'a>) -> Patterns<'a> {
    Patterns(parse_repeated::<Pattern>(
        token_iter,
        parse_pattern,
        Token::Match,
        false,
    ))
}

fn parse_pattern<'a>(token_iter: &mut TokenIterator<'a>) -> Pattern<'a> {
    let from_node = parse_identifier(token_iter);
    let rel_type_token = token_iter.next().unwrap();
    let to_node = parse_identifier(token_iter);
    consume_token(
        token_iter,
        &Token::Colon,
        "Pattern must end with relationship name.",
    );
    let rel_name = parse_identifier(token_iter);
    Pattern {
        from_node,
        to_node,
        relationship_type: match rel_type_token {
            Token::Edge => Relationship::Edge(rel_name),
            Token::Path => Relationship::Path(rel_name),
            _ => panic!("Unsupported relationship type: {:?}", rel_type_token),
        },
    }
}

fn parse_filters<'a>(token_iter: &mut TokenIterator<'a>) -> Filters<'a> {
    Filters(parse_repeated::<Filter>(
        token_iter,
        parse_filter,
        Token::Where,
        true,
    ))
}

fn parse_filter<'a>(token_iter: &mut TokenIterator<'a>) -> Filter<'a> {
    let node = parse_identifier(token_iter);
    match token_iter.next() {
        None => {
            panic!("Expected period, got none.");
        }
        Some(Token::Period) => {
            let property = parse_identifier(token_iter);
            consume_token(
                token_iter,
                &Token::Equals,
                "Only support equality in properties.",
            );
            let val = parse_value(token_iter);

            Filter::Property(node, property, val)
        }
        Some(token) => panic!("Unrecognized token: {:?}", token),
    }
}

fn parse_action<'a>(token_iter: &mut TokenIterator<'a>) -> Action<'a> {
    if token_iter.peek().is_none() {
        return Action::None;
    }

    consume_token(token_iter, &Token::Return, "Expected RETURN");
    let id = parse_identifier(token_iter);
    let operator_token = token_iter.next().unwrap();
    match &operator_token {
        Token::Period => {
            let property = parse_identifier(token_iter);
            consume_token(token_iter, &Token::Comma, "Must end with a comma");
            Action::Property(id, property)
        }
        Token::Comma => Action::CallUdf(id),
        _ => panic!("Unrecognized token: {:?}", operator_token),
    }
}

fn parse_identifier<'a>(token_iter: &mut TokenIterator<'a>) -> Identifier<'a> {
    let identifier_token = token_iter.next().unwrap();
    match &identifier_token {
        Token::Identifier(id_name) => Identifier { id_name },
        _ => panic!(
            "Invalid token: {:?}, expected Token::Identifier",
            identifier_token
        ),
    }
}

fn parse_value<'a>(token_iter: &mut TokenIterator<'a>) -> Value<'a> {
    let value_token = token_iter.next().unwrap();
    match &value_token {
        Token::Value(value) => Value::U32(*value),
        Token::Identifier(s) => Value::Str(s),
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
    test_parser_fail!(
        r"MATCH n-->m :a, WHERE n.x.y.z == 5,",
        parse_prog,
        test_parse_nested_properties,
        "Invalid token: Period, expected Equals"
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
            Prog {
                patterns: Patterns(vec![Pattern {
                    from_node: Identifier { id_name: "n" },
                    to_node: Identifier { id_name: "m" },
                    relationship_type: Relationship::Edge(Identifier { id_name: "a" })
                }]),
                filters: Filters(vec![Filter::Property(
                    Identifier { id_name: "n" },
                    Identifier { id_name: "x" },
                    Value::Str("k")
                )]),
                action: Action::None,
            }
        )
    }

    test_parser_success!(r"RETURN n.x,", parse_action, test_parse_return_property);
    #[test]
    fn test_parse_actions_return() {
        let input = r"RETURN n.x,";
        let tokens = &mut get_tokens(input);
        let token_iter = &mut tokens.iter().peekable();
        let action = parse_action(token_iter);
        assert_eq!(
            action,
            Action::Property(Identifier { id_name: "n" }, Identifier { id_name: "x" },)
        )
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
            Prog {
                patterns: Patterns(vec![Pattern {
                    from_node: Identifier { id_name: "n" },
                    to_node: Identifier { id_name: "m" },
                    relationship_type: Relationship::Edge(Identifier { id_name: "a" })
                }]),
                filters: Filters::new(),
                action: Action::Property(Identifier { id_name: "n" }, Identifier { id_name: "x" },)
            }
        )
    }

    #[test]
    fn test_parse_action_call_udf() {
        let input = r"MATCH n-->m: a, RETURN f,";
        let tokens = &mut get_tokens(input);
        let token_iter = &mut tokens.iter().peekable();
        let prog = parse_prog(token_iter);
        assert_eq!(
            prog,
            Prog {
                patterns: Patterns(vec![Pattern {
                    from_node: Identifier { id_name: "n" },
                    to_node: Identifier { id_name: "m" },
                    relationship_type: Relationship::Edge(Identifier { id_name: "a" })
                }]),
                filters: Filters::new(),
                action: Action::CallUdf(Identifier { id_name: "f" })
            }
        )
    }

    #[test]
    fn test_parse_graph_property() {
        let input: &str = r"MATCH n-->m: a, RETURN graph.num_vertices,";
        let tokens = &mut get_tokens(input);
        let token_iter = &mut tokens.iter().peekable();
        let prog = parse_prog(token_iter);
        assert_eq!(
            prog,
            Prog {
                patterns: Patterns(vec![Pattern {
                    from_node: Identifier { id_name: "n" },
                    to_node: Identifier { id_name: "m" },
                    relationship_type: Relationship::Edge(Identifier { id_name: "a" })
                }]),
                filters: Filters::new(),
                action: Action::Property(
                    Identifier { id_name: "graph" },
                    Identifier {
                        id_name: "num_vertices"
                    }
                )
            }
        )
    }
}
