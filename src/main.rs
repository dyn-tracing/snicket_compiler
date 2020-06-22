extern crate dyntracing;
extern crate handlebars;

use dyntracing::{code_gen, lexer, parser, tree_fold::TreeFold};
use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
        Ok(_) => print!("Successfully read {}", display),
    }

    let query = r"MATCH frontend-->recommendationservice MATCH recommendationservice-->productcatalogservice";
    let tokens = lexer::get_tokens(query);
    let mut token_iter = tokens.iter().peekable();
    let parse_tree = parser::parse_prog(&mut token_iter);

    let mut code_gen = code_gen::CodeGen::new();
    code_gen.visit_prog(&parse_tree);

    assert_eq!(code_gen.paths.len(), 1);
    assert_eq!(
        code_gen.paths[0],
        vec!["frontend", "recommendationservice", "productcatalogservice"]
    );

    let mut data = BTreeMap::new();
    data.insert("path", code_gen.paths[0].join(","));

    let handlebars = Handlebars::new();

    print!(
        "{}",
        handlebars
            .render_template(&template_str, &data)
            .ok()
            .unwrap()
    );
}
