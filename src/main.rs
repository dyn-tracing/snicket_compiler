extern crate dyntracing;
extern crate handlebars;
extern crate serde;

use dyntracing::{lexer, parser};
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize)]
struct Data {
    root: String,
    paths: Vec<String>,
    return_action: Vec<String>,
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

    let query = "MATCH a-->b : x, b-->c : y, a-->d: z, \
                    WHERE a.service_name == productpagev1, \
                            b.service_name == reviewsv2, \
                            c.service_name == ratingsv1, \
                            d.service_name == detailsv1, \
                    RETURN a.service_name,";
    let tokens = lexer::get_tokens(query);
    let mut token_iter = tokens.iter().peekable();
    let _program = parser::parse_prog(&mut token_iter);

    // assert_eq!(code_gen.paths.len(), 2);
    // assert_eq!(code_gen.paths, vec![vec!["a", "b", "c"], vec!["a", "d"],]);

    // let paths: Vec<String> = code_gen
    //     .paths
    //     .iter_mut()
    //     .map(|path| path.join("-"))
    //     .collect();

    // assert_eq!(paths.len(), 2);
    // assert_eq!(paths, vec!["a-b-c", "a-d"]);

    // assert_eq!(code_gen.return_action, vec!["a", "service_name"]);

    // let data = Data {
    //     root: "productpagev1".to_string(),
    //     paths,
    //     return_action: code_gen
    //         .return_action
    //         .into_iter()
    //         .map(|x| x.to_string())
    //         .collect(),
    // };

    // let handlebars = Handlebars::new();

    // let output = handlebars
    //     .render_template(&template_str, &data)
    //     .expect("handlebar render failed");

    // let mut file = File::create("./wasm/filter.cc").expect("file create failed.");
    // file.write_all(output.as_bytes()).expect("write failed");
}
