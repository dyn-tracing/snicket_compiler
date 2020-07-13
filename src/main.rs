extern crate dyntracing;
extern crate handlebars;
extern crate serde;

use dyntracing::{code_gen, lexer, parser, tree_fold::TreeFold};
use handlebars::Handlebars;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize)]
struct Data {
    root: String,
    paths: Vec<String>,
}

fn main() {
    let template_path = Path::new("filter.cc.hbars");
    let display = template_path.display();
    let mut template_file = match File::open(&template_path) {
        Err(msg) => panic!("Failed to open {}: {}", display, msg),
        Ok(file) => file,
    };

    let mut template_str = String::new();
    match template_file.read_to_string(&mut template_str) {
        Err(msg) => panic!("Failed to read {}: {}", display, msg),
        Ok(_) => println!("Successfully read {}", display),
    }

    let query = r"MATCH productpagev1-->reviewsv2 : x, reviewsv2-->ratingsv1 : y, productpagev1-->detailsv1: z,";
    let tokens = lexer::get_tokens(query);
    let mut token_iter = tokens.iter().peekable();
    let parse_tree = parser::parse_prog(&mut token_iter);

    let mut code_gen = code_gen::CodeGen::new();
    code_gen.visit_prog(&parse_tree);

    assert_eq!(code_gen.paths.len(), 2);
    assert_eq!(
        code_gen.paths,
        vec![
            vec!["productpagev1", "reviewsv2", "ratingsv1"],
            vec!["productpagev1", "detailsv1"],
        ]
    );

    let paths: Vec<String> = code_gen
        .paths
        .iter_mut()
        .map(|path| path.join("-"))
        .collect();

    assert_eq!(paths.len(), 2);
    assert_eq!(
        paths,
        vec![
            "productpagev1-reviewsv2-ratingsv1",
            "productpagev1-detailsv1"
        ]
    );

    let data = Data {
        root: "productpagev1".to_string(),
        paths,
    };

    let handlebars = Handlebars::new();

    let output = handlebars
        .render_template(&template_str, &data)
        .expect("handlebar render failed");

    let mut file = File::create("./wasm/filter.cc").expect("file create failed.");
    file.write_all(output.as_bytes()).expect("write failed");
}
