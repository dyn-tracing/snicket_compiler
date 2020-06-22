use grammar::*;
use std::iter::Peekable;
use token::Token;

type TokenIterator<'a> = Peekable<std::slice::Iter<'a, Token<'a>>>;

// Helper function to consume next token and match it against a specified token
// Throw an error if either:
// 1. token_iter is empty
// 2. the next token does not match
fn match_token<'a>(
    token_iter: &mut TokenIterator<'a>,
    expected: Token<'a>,
    error_msg: &'static str,
) {
    if token_iter.peek().is_none() {
        panic!("token_iter is empty. Can't consume next token");
    } else {
        let next_token = token_iter.next().unwrap();
        if *next_token != expected {
            panic!(
                "\nInvalid token: {:?}, expected {:?}.\nError message: {:?}",
                next_token, expected, error_msg
            );
        }
    }
}

pub fn parse_prog<'a>(token_iter: &mut TokenIterator<'a>) -> Prog<'a> {
    let patterns = parse_patterns(token_iter);
    let filters = parse_filters(token_iter);
    Prog { patterns, filters }
}

fn parse_patterns<'a>(token_iter: &mut TokenIterator<'a>) -> Patterns<'a> {
    let is_pattern = |token: &Token| match token {
        Token::Match => true,
        _ => false,
    };

    let mut pattern_vector = Vec::<Pattern>::new();
    loop {
        if token_iter.peek().is_none() || !is_pattern(&token_iter.peek().unwrap()) {
            if pattern_vector.is_empty() {
                panic!("Need at least one pattern and the pattern must start with MATCH.");
            } else {
                return Patterns { pattern_vector };
            }
        } else {
            let pattern = parse_pattern(token_iter);
            pattern_vector.push(pattern);
        }
    }
}

fn parse_pattern<'a>(token_iter: &mut TokenIterator<'a>) -> Pattern<'a> {
    match_token(
        token_iter,
        Token::Match,
        "Pattern must start with the keyword MATCH.",
    );
    let from_node = parse_identifier(token_iter);
    let rel_type_token = token_iter.next().unwrap();
    let relationship_type = match rel_type_token {
        Token::Edge => Relationship::Edge(),
        Token::Path => Relationship::Path(),
        _ => panic!("Unsupported relationship type: {:?}", rel_type_token),
    };
    let to_node = parse_identifier(token_iter);
    Pattern {
        from_node,
        to_node,
        relationship_type,
    }
}

fn parse_filters<'a>(token_iter: &mut TokenIterator<'a>) -> Filters<'a> {
    let mut filter_vector = Vec::<Filter<'a>>::new();
    loop {
        if token_iter.peek().is_none() {
            return Filters { filter_vector };
        } else {
            filter_vector.push(parse_filter(token_iter));
        }
    }
}

fn parse_filter<'a>(token_iter: &mut TokenIterator<'a>) -> Filter<'a> {
    match_token(
        token_iter,
        Token::Where,
        "Filter must start with the keyword WHERE.",
    );
    let node = parse_identifier(token_iter);
    let operator_token = token_iter.next().unwrap();
    match &operator_token {
        Token::Colon => {
            let label = parse_identifier(token_iter);
            Filter::Label(node, label)
        }
        Token::Period => {
            let property = parse_identifier(token_iter);
            match_token(
                token_iter,
                Token::Equals,
                "Only support equality in properties.",
            );
            let val = parse_value(token_iter);
            Filter::Property(node, property, val)
        }
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

fn parse_value<'a>(token_iter: &mut TokenIterator<'a>) -> Value {
    let value_token = token_iter.next().unwrap();
    match &value_token {
        Token::Value(value) => Value { value: *value },
        _ => panic!("Invalid token: {:?}, expected Token::Value", value_token),
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
    test_parser_success!(r"MATCH n-->m", parse_pattern, test_parse_pattern1);
    test_parser_success!(r"MATCH n-*>m", parse_pattern, test_parse_pattern2);
    test_parser_fail!(
        r"n",
        parse_pattern,
        test_parse_pattern_fail,
        "Invalid token: Identifier(\"n\"), expected Match"
    );
    test_parser_success!(r"WHERE n.abc == 5", parse_filter, test_parse_filter);
    test_parser_success!(r"WHERE n:Node", parse_filter, test_parse_filter2);
    test_parser_fail!(
        r"n",
        parse_filter,
        test_parse_filter_fail,
        "Invalid token: Identifier(\"n\"), expected Where"
    );
    test_parser_success!(
        r"MATCH n-->m MATCH n-*>m WHERE n:Node WHERE m:Node WHERE
                         n.abc == 5",
        parse_prog,
        test_parse_prog
    );
    test_parser_success!(
        r"MATCH n-->m MATCH n-*>m",
        parse_prog,
        test_parse_prog_no_filter
    );
    test_parser_fail!(
        r"n-->m",
        parse_prog,
        test_parse_prog_fail,
        "Need at least one pattern and the pattern must start with MATCH."
    );
}
