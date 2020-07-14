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
        panic!(
            "Invalid token: Null, expected {:?}.\nError message: {:?}",
            expected, error_msg
        );
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
    let filters = if token_iter.peek().is_none() {
        Filters {
            filter_vector: Vec::new(),
        }
    } else {
        parse_filters(token_iter)
    };
    let actions = if token_iter.peek().is_none() {
        Actions {
            action_vector: Vec::new(),
        }
    } else {
        parse_actions(token_iter)
    };
    Prog { patterns, filters, actions }
}

fn parse_patterns<'a>(token_iter: &mut TokenIterator<'a>) -> Patterns<'a> {
    let is_ident = |token: &Token| match token {
        Token::Identifier(_) => true,
        _ => false,
    };

    let mut pattern_vector = Vec::<Pattern>::new();
    match_token(
        token_iter,
        Token::Match,
        "Patterns must start with the keyword MATCH.",
    );
    loop {
        if token_iter.peek().is_none() || !is_ident(&token_iter.peek().unwrap()) {
            if pattern_vector.is_empty() {
                panic!("Need at least one pattern.");
            } else {
                return Patterns { pattern_vector };
            }
        } else {
            let pattern = parse_pattern(token_iter);
            match_token(
                token_iter,
                Token::Comma,
                "Expected comma as separator between patterns.",
            );
            pattern_vector.push(pattern);
        }
    }
}

fn parse_pattern<'a>(token_iter: &mut TokenIterator<'a>) -> Pattern<'a> {
    let from_node = parse_identifier(token_iter);
    let rel_type_token = token_iter.next().unwrap();
    let to_node = parse_identifier(token_iter);
    match_token(
        token_iter,
        Token::Colon,
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
    let mut filter_vector = Vec::<Filter<'a>>::new();
    match_token(
        token_iter,
        Token::Where,
        "Filters must start with the keyword WHERE.",
    );
    loop {
        if token_iter.peek().is_none() {
            return Filters { filter_vector };
        } else {
            let filter = parse_filter(token_iter);
            match_token(
                token_iter,
                Token::Comma,
                "Expected comma as separator between filters.",
            );
            filter_vector.push(filter);
        }
    }
}

fn parse_filter<'a>(token_iter: &mut TokenIterator<'a>) -> Filter<'a> {
    let node = parse_identifier(token_iter);
    let operator_token = token_iter.next().unwrap();
    match &operator_token {
        Token::Colon => {
            let label = parse_identifier(token_iter);
            Filter::Label(node, label)
        }
        Token::Period => {
            let mut properties = Vec::new();
            let mut property = parse_identifier(token_iter);
            properties.push(property);
            while let Some(Token::Period) = token_iter.peek() {
                // Consume token period
                token_iter.next();
                property = parse_identifier(token_iter);
                properties.push(property);
            }
            match_token(
                token_iter,
                Token::Equals,
                "Only support equality in properties.",
            );
            let val = parse_value(token_iter);
            Filter::Property(node, properties, val)
        }
        _ => panic!("Unrecognized token: {:?}", operator_token),
    }
}

fn parse_actions<'a>(token_iter: &mut TokenIterator<'a>) -> Actions<'a> {
    let mut action_vector = Vec::<Action<'a>>::new();
    match_token(
        token_iter,
        Token::Return,
        "Actions must start with the keyword RETURN.",
    );
    loop {
        if token_iter.peek().is_none() {
            return Actions { action_vector };
        } else {
            let action = parse_action(token_iter);
            match_token(
                token_iter,
                Token::Comma,
                "Expected comma as separator between filters.",
            );
            action_vector.push(action);
        }
    }
}

fn parse_action<'a>(token_iter: &mut TokenIterator<'a>) -> Action<'a> {
    let node = parse_identifier(token_iter);
    let operator_token = token_iter.next().unwrap();
    match &operator_token {
        Token::Period => {
            let mut properties = Vec::new();
            let mut property = parse_identifier(token_iter);
            properties.push(property);
            while let Some(Token::Period) = token_iter.peek() {
                // Consume token period
                token_iter.next();
                property = parse_identifier(token_iter);
                properties.push(property);
            }
            Action::Property(node, properties)
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
        "Patterns must start with the keyword MATCH."
    );
    test_parser_success!(r"WHERE n.abc == 5,", parse_filters, test_parse_filter);
    test_parser_success!(r"WHERE n:Node,", parse_filters, test_parse_filter2);
    test_parser_fail!(
        r"n",
        parse_filters,
        test_parse_filter_fail,
        "Filters must start with the keyword WHERE."
    );
    test_parser_success!(
        r"MATCH n-->m : a, n-*>m : b, WHERE n:Node, m:Node , n.abc == 5,",
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
        "Patterns must start with the keyword MATCH."
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
}
